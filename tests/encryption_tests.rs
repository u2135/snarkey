use snarkey_core::encryption::{encrypt, decrypt};

#[test]
fn test_encryption() {
    let message = "Hello, world!";
    let encrypted = encrypt::encrypt_message(message);
    let decrypted = decrypt::decrypt_message(&encrypted).unwrap();
    assert_eq!(message, decrypted);
}
