use std::marker::PhantomData;

use criterion::{criterion_group, criterion_main, Criterion};
use either::Either;
use group::ff::PrimeField;
use halo2_gadgets::poseidon::primitives::{ConstantLength, Hash, Spec};
use halo2_proofs::dev::MockProver;
use halo2_proofs::{circuit::Value, pasta::Fp};
use snarkey::poseidon_circuit::{HashCircuit, PoseidonSpec};

const K: u32 = 8;

fn bench_poseidon<S, const WIDTH: usize, const RATE: usize, const L: usize, const MSGSIZE: usize>(
    name: &str,
    c: &mut Criterion,
) where
    S: Spec<Fp, WIDTH, RATE> + Copy + Clone,
{
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

    let mut input: [Fp; L] = [Fp::one(); L];
    input[0] = key;
    input[1] = nonce;

    let hasher = || Hash::<_, S, ConstantLength<L>, WIDTH, RATE>::init();
    let a = hasher().hash(input);

    let output = if message.len() == 1 {
        Either::Left(message.map(|val| val + a).into_iter())
    } else {
        Either::Right(message.into_iter().enumerate().map(|(i, msg_i)| {
            let i_ff = Fp::from_u128(i.try_into().unwrap());

            let mut input: [Fp; L] = [Fp::one(); L];
            input[0] = a;
            input[1] = i_ff;

            let r_i = hasher().hash(input);
            msg_i + &r_i
        }))
    };
    let output: Vec<Vec<Fp>> = output.map(|val| vec![val]).collect();

    let prover_name = name.to_string() + "-prover";
    let verifier_name = name.to_string() + "-verifier";

    c.bench_function(&prover_name, |b| {
        b.iter(|| {
            MockProver::run(K, &circuit, output.clone()).unwrap();
        })
    });

    // Create a proof
    let prover = MockProver::run(K, &circuit, output).unwrap();
    c.bench_function(&verifier_name, |b| {
        b.iter(|| prover.verify().is_ok());
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_poseidon::<PoseidonSpec<3, 2>, 3, 2, 2, 1>("MSGSIZE = 1, K = 8", c);
    bench_poseidon::<PoseidonSpec<3, 2>, 3, 2, 2, 2>("MSGSIZE = 2, K = 8", c);
    bench_poseidon::<PoseidonSpec<3, 2>, 3, 2, 2, 3>("MSGSIZE = 3, K = 8", c);
    bench_poseidon::<PoseidonSpec<3, 2>, 3, 2, 2, 4>("MSGSIZE = 1, K = 8", c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
