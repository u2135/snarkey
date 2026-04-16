use halo2_proofs::pasta::Fp;
use halo2_gadgets::poseidon::primitives::ConstantLength;
use halo2_gadgets::poseidon::primitives as poseidon;
use rand::Rng;
use std::marker::PhantomData;

use crate::circuit::poseidon_circuit::{HashCircuit, PoseidonSpec};

const MSGSIZE: usize = 1;
const L: usize = 2;
const K: u32 = 8;

/// Encrypts a field element message using:
///     ciphertext = message + H(key, nonce)
///
/// Returns (public_data, proof) where public_data = [nonce, ciphertext]
pub fn encrypt_and_prove<const WIDTH: usize, const RATE: usize>(message: Fp, key: Fp) -> (Vec<Fp>, Vec<u8>) {
    // 1. Sample a random nonce.
    let mut rng = rand::thread_rng();
    let nonce_val: u64 = rng.gen();
    let nonce = Fp::from(nonce_val);

    // 2. Off-circuit: native `halo2_poseidon::Hash` (via `primitives`), not the circuit gadget.
    let a = poseidon::Hash::<Fp, PoseidonSpec<WIDTH, RATE>, ConstantLength<L>, WIDTH, RATE>::init()
        .hash([key, nonce]);

    // 3. Compute ciphertext = message + a.
    let ciphertext = message + a;
    let public_data = vec![nonce, ciphertext];

    // 4. Instantiate the circuit with secrets.
    let circuit = HashCircuit::<PoseidonSpec<WIDTH, RATE>, WIDTH, RATE, MSGSIZE> {
        message: halo2_proofs::circuit::Value::known([message]),
        key: halo2_proofs::circuit::Value::known(key),
        nonce: halo2_proofs::circuit::Value::known(nonce),
        _spec: PhantomData,
    };

    // 5. The public instance is the expected ciphertext.
    let instance = vec![vec![ciphertext]];

    // 6. Generate a dummy proof using MockProver.
    let proof = dummy_prove(&circuit, instance, K).expect("proof creation failed");
    (public_data, proof)
}

fn dummy_prove<C>(circuit: &C, instance: Vec<Vec<Fp>>, k: u32) -> Result<Vec<u8>, String>
where
    C: halo2_proofs::plonk::Circuit<Fp>,
{
    use halo2_proofs::dev::MockProver;
    let prover = MockProver::run(k, circuit, instance)
        .map_err(|e| format!("Prover run failure: {:?}", e))?;
    prover.verify().map_err(|e| format!("Verification failed: {:?}", e))?;
    Ok(vec![1, 2, 3]) // Dummy proof bytes.
}