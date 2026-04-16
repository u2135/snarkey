use snarkey_core::circuit::encryption_circuit::{prove, verify_encryption};

pub fn verify_cli(plaintext: &str, ciphertext: &str) -> bool {
    let proof = prove(plaintext.as_bytes(), ciphertext.as_bytes());
    verify_encryption(proof)
}
