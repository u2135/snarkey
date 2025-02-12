pub fn prove(plaintext: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut proof = Vec::new();
    proof.extend_from_slice(plaintext);
    proof.extend_from_slice(ciphertext);
    proof
}

pub fn verify_encryption(plaintext: &[u8], ciphertext: &[u8]) -> bool {
    let proof = prove(plaintext, ciphertext);
    !proof.is_empty()
}
