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

const L: usize = 2;

#[derive(Debug, Clone)]
struct MyConfig<const WIDTH: usize, const RATE: usize> {
    input: [Column<Advice>; L],
    expected: Column<Instance>,
    poseidon_config: Pow5Config<Fp, WIDTH, RATE>,
}

#[derive(Clone, Copy)]
struct HashCircuit<S, const WIDTH: usize, const RATE: usize, const MSGSIZE: usize>
where
    S: Spec<Fp, WIDTH, RATE> + Clone + Copy,
{
    message: Value<[Fp; MSGSIZE]>,
    key: Value<Fp>,
    nonce: Value<Fp>,
    _spec: PhantomData<S>,
}

impl<S, const WIDTH: usize, const RATE: usize, const MSGSIZE: usize> Circuit<Fp>
    for HashCircuit<S, WIDTH, RATE, MSGSIZE>
where
    S: Spec<Fp, WIDTH, RATE> + Copy + Clone,
{
    type Config = MyConfig<WIDTH, RATE>;
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
        let expected = meta.instance_column();
        meta.enable_equality(expected);
        let partial_sbox = meta.advice_column();

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();

        meta.enable_constant(rc_b[0]);

        Self::Config {
            input: state[..RATE].try_into().unwrap(),
            expected,
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
            || "load message",
            |mut region| {
                let key = region.assign_advice(|| "load key", config.input[0], 0, || self.key)?;
                let nonce =
                    region.assign_advice(|| "load nonce", config.input[1], 0, || self.nonce)?;
                Ok([key, nonce])
            },
        )?;
        let a = hasher.hash(layouter.namespace(|| "hash"), seed_input)?;

        // TODO: compute r_i = H(a, i)
        // let i: AssignedCell = ...;
        // hasher.hash(..., [a, i]);
        // TODO: compute enc_i = msg_i + r_i

        layouter.constrain_instance(a.cell(), config.expected, 0)
    }
}

#[derive(Debug, Clone, Copy)]
struct PoseidonSpec<const WIDTH: usize, const RATE: usize>;

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
    use halo2_gadgets::poseidon::primitives as poseidon;
    use halo2_proofs::{
        pasta::{pallas, vesta},
        plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier},
        poly::commitment::Params,
        transcript::{Blake2bRead, Blake2bWrite, Challenge255},
    };

    const L: usize = 2;
    const MSGSIZE: usize = 10;
    const WIDTH: usize = 3;
    const RATE: usize = 2;
    type S = PoseidonSpec<WIDTH, RATE>;
    const K: u32 = 7;

    #[test]
    fn run_enc() {
        let mut rng = rand::rngs::OsRng;

        // Initialize the information for the encryption
        let params: Params<vesta::Affine> = Params::new(K);

        let message = (0..MSGSIZE)
            .map(|_| pallas::Base::one())
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

        let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
        let pk = keygen_pk(&params, vk, &circuit).expect("keygen_pk should not fail");

        // Compute the encryption
        let hasher = || poseidon::Hash::<_, S, ConstantLength<L>, WIDTH, RATE>::init();
        let a = hasher().hash([key, nonce]);
        // let output: Vec<Fp> = message
        //     .into_iter()
        //     .enumerate()
        //     .map(|(i, msg_i)| {
        //         let i_ff = Fp::from_u128(i.try_into().unwrap());
        //         let r_i = hasher().hash([a, i_ff]);
        //         msg_i + &r_i
        //     })
        //     .collect();
        let output = [a];

        // Create a proof
        let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
        create_proof(
            &params,
            &pk,
            &[circuit],
            &[&[&output]],
            &mut rng,
            &mut transcript,
        )
        .unwrap();

        let proof = transcript.finalize();

        let strategy = SingleVerifier::new(&params);
        let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
        let verify_proof_result = verify_proof(
            &params,
            pk.get_vk(),
            strategy,
            &[&[&output]],
            &mut transcript,
        );
        assert!(verify_proof_result.is_ok())
    }
}
