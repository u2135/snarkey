use halo2_proofs::pasta::Fp;
use halo2_gadgets::poseidon::primitives::ConstantLength;
use halo2_gadgets::poseidon::Hash;
use std::marker::PhantomData;
//use typenum::{U1, U2, U3};

use crate::circuit::poseidon_circuit::{HashCircuit, PoseidonSpec};

pub const WIDTH: usize = 3;
pub const RATE: usize = 2;
pub const MSGSIZE: usize = 1;
const K: u32 = 8;

/// Given public_data ([nonce, ciphertext]) and the secret key,
/// recovers the original message as:
///  message = ciphertext - H(key, nonce)
pub fn decrypt_and_verify(public_data: &[Fp], key: Fp, proof: &[u8]) -> Option<Fp> {
    let nonce = public_data[0];
    let ciphertext = public_data[1];

    let hasher = || {
        Hash::<_, PoseidonSpec<WIDTH, RATE>, ConstantLength<2>, WIDTH, RATE, MSGSIZE>::init()
    };
    let a = hasher().hash([key, nonce]);

    let message = ciphertext - a;

    let circuit = HashCircuit::<PoseidonSpec<WIDTH, RATE>, WIDTH, RATE, MSGSIZE> {
        message: halo2_proofs::circuit::Value::known([message]),
        key: halo2_proofs::circuit::Value::known(key),
        nonce: halo2_proofs::circuit::Value::known(nonce),
        _spec: PhantomData,
    };
    let instance = vec![vec![ciphertext]];

    if dummy_verify(&circuit, instance, K, proof).is_ok() {
        Some(message)
    } else {
        None
    }
}

fn dummy_verify<C, const MSG: usize>(
    circuit: &C,
    instance: Vec<Vec<Fp>>,
    k: u32,
    _proof: &[u8],
) -> Result<(), String>
where
    C: halo2_proofs::plonk::Circuit<Fp>,
{
    use halo2_proofs::dev::MockProver;
    let prover = MockProver::run(k, circuit, instance)
        .map_err(|e| format!("Prover run failure: {:?}", e))?;
    prover.verify().map_err(|e| format!("Verification failed: {:?}", e))
}