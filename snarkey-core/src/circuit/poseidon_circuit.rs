use crate::circuit::add_chip::{AddChip, AddConfig, AddInstruction};
use group::ff::PrimeField;
use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
};

use halo2_gadgets::poseidon::{
    primitives::{generate_constants, ConstantLength, Mds, Spec},
    Hash, Pow5Chip, Pow5Config,
};
use std::convert::TryInto;
use std::marker::PhantomData;
//use typenum::U1;


const L: usize = 2;

#[derive(Debug, Clone)]
pub struct MyConfig<const WIDTH: usize, const RATE: usize, const MSGSIZE: usize> {
    input: [Column<Advice>; L],
    expected: [Column<Instance>; MSGSIZE],
    poseidon_config: Pow5Config<Fp, WIDTH, RATE>,
    add_config: AddConfig,
}

#[derive(Clone, Copy)]
pub struct HashCircuit<S, const WIDTH: usize, const RATE: usize, const MSGSIZE: usize>
where
    S: Spec<Fp, WIDTH, RATE> + Clone + Copy,
{
    pub message: Value<[Fp; MSGSIZE]>,
    pub key: Value<Fp>,
    pub nonce: Value<Fp>,
    pub _spec: PhantomData<S>,
}

impl<S, const WIDTH: usize, const RATE: usize, const MSGSIZE: usize> Circuit<Fp>
    for HashCircuit<S, WIDTH, RATE, MSGSIZE>
where
    S: Spec<Fp, WIDTH, RATE> + Copy + Clone,
{
    type Config = MyConfig<WIDTH, RATE, MSGSIZE>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            message: Value::unknown(),
            key: Value::unknown(),
            nonce: Value::unknown(),
            _spec: PhantomData,
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let expected = (0..MSGSIZE)
            .map(|_| {
                let col = meta.instance_column();
                meta.enable_equality(col);
                col
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("array size invalid");
        let partial_sbox = meta.advice_column();

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();

        meta.enable_constant(rc_b[0]);

        Self::Config {
            input: state[..RATE].try_into().unwrap(),
            expected,
            add_config: AddChip::configure(meta, state[0], state[1], state[2]),
            poseidon_config: Pow5Chip::configure::<S>(
                meta,
                state.try_into().unwrap(),
                partial_sbox,
                rc_a.try_into().unwrap(),
                rc_b.try_into().unwrap(),
            ),
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let chip: Pow5Chip<Fp, WIDTH, RATE> = Pow5Chip::construct(config.poseidon_config.clone());
        let hasher = Hash::<_, _, S, ConstantLength<L>, WIDTH, RATE>::init(
            chip,
            layouter.namespace(|| "init"),
        )
        .expect("hasher construction failed");

        let seed_input = layouter.assign_region(
            || "load key_nonce",
            |mut region| {
                let key = region.assign_advice(|| "load key", config.input[0], 0, || self.key)?;
                let nonce =
                    region.assign_advice(|| "load nonce", config.input[1], 0, || self.nonce)?;
                Ok([key, nonce])
            },
        )?;
        let a = hasher.hash(layouter.namespace(|| "hash"), seed_input)?;

        if MSGSIZE == 1 {
            let i = 0;
            let msg_i: Value<Fp> = self.message.map(|vals| vals[i]);
            let msg_i = layouter.assign_region(
                || "load msg_0",
                |mut region| region.assign_advice(|| "load i", config.input[0], 0, || msg_i),
            )?;

            let chip = AddChip::construct(config.add_config.clone());
            let res = chip.add(&mut layouter, &msg_i, &a)?;
            layouter.constrain_instance(res.cell(), config.expected[i], 0)?;
        } else {
            for i in 0..MSGSIZE {
                let counter = Fp::from_u128(i.try_into().unwrap());
                let seed_input = layouter.assign_region(
                    || "load message",
                    |mut region| {
                        let c_i = region.assign_advice_from_constant(
                            || "load i",
                            config.input[1],
                            0,
                            counter,
                        )?;
                        Ok(c_i)
                    },
                )?;

                let chip: Pow5Chip<Fp, WIDTH, RATE> =
                    Pow5Chip::construct(config.poseidon_config.clone());
                let hasher = Hash::<_, _, S, ConstantLength<L>, WIDTH, RATE>::init(
                    chip,
                    layouter.namespace(|| "init"),
                )
                .expect("hasher construction failed");

                let r_i = hasher.hash(layouter.namespace(|| "hash"), [a.clone(), seed_input])?;

                let msg_i: Value<Fp> = self.message.map(|vals| vals[i]);
                let msg_i = layouter.assign_region(
                    || "load msg_i",
                    |mut region| {
                        let c_i =
                            region.assign_advice(|| "load i", config.input[1], 0, || msg_i)?;
                        Ok(c_i)
                    },
                )?;
                let chip = AddChip::construct(config.add_config.clone());
                let res_i = chip.add(&mut layouter, &msg_i, &r_i)?;

                layouter.constrain_instance(res_i.cell(), config.expected[i], 0)?;
            }
        }

        Ok(())
        // TODO: compute r_i = H(a, i)
        // let i: AssignedCell = ...;
        // hasher.hash(..., [a, i]);
        // TODO: compute enc_i = msg_i + r_i
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PoseidonSpec<const WIDTH: usize, const RATE: usize>;

impl<const WIDTH: usize, const RATE: usize> Spec<Fp, WIDTH, RATE> for PoseidonSpec<WIDTH, RATE> {
    fn full_rounds() -> usize {
        8
    }

    fn partial_rounds() -> usize {
        56
    }

    fn sbox(val: Fp) -> Fp {
        val.pow_vartime(&[5])
    }

    fn secure_mds() -> usize {
        0
    }

    fn constants() -> (Vec<[Fp; WIDTH]>, Mds<Fp, WIDTH>, Mds<Fp, WIDTH>) {
        generate_constants::<_, Self, WIDTH, RATE>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use either::Either;
    use group::ff::PrimeField;
    use halo2_gadgets::poseidon::primitives as poseidon;
    use halo2_proofs::dev::MockProver;

    const L: usize = 2;
    const MSGSIZE: usize = 1;
    const WIDTH: usize = 3;
    const RATE: usize = 2;
    type S = PoseidonSpec<WIDTH, RATE>;
    const K: u32 = 8;

    #[test]
    fn run_enc() {
        println!("msg");
        let message = (0..MSGSIZE)
            .map(|_| Fp::one())
            .collect::<Vec<_>>()
            .try_into()
            .expect("array of wrong size");
        let key = Fp::one();
        let nonce = Fp::one();

        let circuit = HashCircuit::<S, WIDTH, RATE, MSGSIZE> {
            message: Value::known(message),
            key: Value::known(key),
            nonce: Value::known(nonce),
            _spec: PhantomData,
        };

        // Compute the encryption
        let hasher = || {
            Hash::<_, PoseidonSpec<WIDTH, RATE>, ConstantLength<2>, WIDTH, RATE, U1>::init()
        };
        let a = hasher().hash([key, nonce]);
        let output = if message.len() == 1 {
            Either::Left(message.map(|val| val + a).into_iter())
        } else {
            Either::Right(message.into_iter().enumerate().map(|(i, msg_i)| {
                let i_ff = Fp::from_u128(i.try_into().unwrap());
                let r_i = hasher().hash([a, i_ff]);
                msg_i + &r_i
            }))
        };

        // instance is of the form [[col1_inst1, col1_inst2, ...], [col2_inst1, col2_inst2], ...]
        let output = output.map(|val| vec![val]).collect();

        // Create a proof
        println!("creating proof");

        let prover = MockProver::run(K, &circuit, output).unwrap();
        prover.verify().expect("verify");
    }
}