use crate::hash::poseidon_hash;
use base64;

pub fn encrypt_message(plain: &str) -> String {
    let key = poseidon_hash("secret_key");
    let encrypted: Vec<u8> = plain.bytes().map(|b| b ^ key as u8).collect();
    base64::encode(encrypted)
}
