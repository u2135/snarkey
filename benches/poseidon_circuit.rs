use std::marker::PhantomData;

use cpazk::poseidon_circuit::{HashCircuit, PoseidonSpec};
use criterion::{criterion_group, criterion_main, Criterion};
use group::ff::Field;
use halo2_gadgets::poseidon::{self, primitives::{ConstantLength, Spec}, Hash};
use halo2_proofs::{circuit::Value, pasta::{pallas, vesta, Fp}, plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier}, poly::commitment::Params, transcript::{Blake2bRead, Blake2bWrite, Challenge255}};
use rand::rngs::OsRng;

const K: u32 = 7;

fn bench_poseidon<S, const WIDTH: usize, const RATE: usize, const L: usize>(
    name: &str,
    c: &mut Criterion,
) where
    S: Spec<Fp, WIDTH, RATE> + Copy + Clone,
{
    // Initialize the polynomial commitment parameters
    let params: Params<vesta::Affine> = Params::new(K);

    let empty_circuit = HashCircuit::<S, WIDTH, RATE, L> {
        message: Value::unknown(),
        _spec: PhantomData,
        key: Value::unknown(),
        nonce: Value::unknown(),
    };

    // Initialize the proving key
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk, &empty_circuit).expect("keygen_pk should not fail");

    let prover_name = name.to_string() + "-prover";
    let verifier_name = name.to_string() + "-verifier";

    let mut rng = OsRng;
    let message = (0..L)
        .map(|_| pallas::Base::random(rng))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // TODO: CHECK THE PART BELOW
    let mut input: [Fp; L] = [Fp::one(); L];
    let key = Fp::one();
    let nonce = Fp::one();
    input[0] = key;
    input[1] = nonce;
    let hasher = || halo2_gadgets::poseidon::primitives::Hash::<_, S, ConstantLength<L>, WIDTH, RATE>::init();
    let output = hasher().hash(input);

    let circuit = HashCircuit::<S, WIDTH, RATE, L> {
        message: Value::known(message),
        _spec: PhantomData,
        key: Value::known(key),
        nonce: Value::known(nonce),
    };

    c.bench_function(&prover_name, |b| {
        b.iter(|| {
            let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
            create_proof(
                &params,
                &pk,
                &[circuit],
                &[&[&[output]]],
                &mut rng,
                &mut transcript,
            )
            .expect("proof generation should not fail")
        })
    });

    // Create a proof
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[&[output]]],
        &mut rng,
        &mut transcript,
    )
    .expect("proof generation should not fail");
    let proof = transcript.finalize();

    c.bench_function(&verifier_name, |b| {
        b.iter(|| {
            let strategy = SingleVerifier::new(&params);
            let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
            assert!(verify_proof(
                &params,
                pk.get_vk(),
                strategy,
                &[&[&[output]]],
                &mut transcript
            )
            .is_ok());
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_poseidon::<PoseidonSpec<3, 2>, 3, 2, 2>("WIDTH = 3, RATE = 2", c);
    bench_poseidon::<PoseidonSpec<9, 8>, 9, 8, 2>("WIDTH = 9, RATE = 8", c);
    bench_poseidon::<PoseidonSpec<12, 11>, 12, 11, 2>("WIDTH = 12, RATE = 11", c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);