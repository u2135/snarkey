use std::marker::PhantomData;

use criterion::{criterion_group, criterion_main, Criterion};
use either::Either;
use group::ff::PrimeField;
use halo2_gadgets::poseidon::primitives::{ConstantLength, Hash, Spec};
use halo2_proofs::pasta::vesta::Affine;
use halo2_proofs::plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier};
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::transcript::{Blake2bRead, Blake2bWrite, Challenge255};
use halo2_proofs::{circuit::Value, pasta::Fp};
use rand::rngs::OsRng;
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
    let output = output.iter().map(|x| x.as_slice()).collect::<Vec<_>>();

    let prover_name = name.to_string() + "-prover";
    let verifier_name = name.to_string() + "-verifier";

    // Initialize the polynomial commitment parameters
    let params: Params<Affine> = Params::new(K);

    // Initialize the proving key
    let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk, &circuit).expect("keygen_pk should not fail");
    let mut rng = OsRng;

    c.bench_function(&prover_name, |b| {
        b.iter(|| {
            let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
            create_proof(
                &params,
                &pk,
                &[circuit],
                &[output.as_slice()],
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
        &[output.as_slice()],
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
                &[output.as_slice()],
                &mut transcript
            )
            .is_ok());
        });
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
