use crate::hash::poseidon_hash;
use base64;

pub fn decrypt_message(cipher: &str) -> Result<String, &'static str> {
    let key = poseidon_hash("secret_key");
    let decoded = base64::decode(cipher).map_err(|_| "Invalid ciphertext")?;
    let decrypted: Vec<u8> = decoded.iter().map(|&b| b ^ key as u8).collect();
    String::from_utf8(decrypted).map_err(|_| "Decryption failed")
}
