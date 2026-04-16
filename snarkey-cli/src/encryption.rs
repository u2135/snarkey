use snarkey_core::encryption::encrypt;

pub fn encrypt_cli(message: &str) -> String {
    encrypt::encrypt_message(message)
}

pub fn decrypt_cli(cipher: &str) -> Result<String, &'static str> {
    snarkey_core::encryption::decrypt::decrypt_message(cipher)
}
