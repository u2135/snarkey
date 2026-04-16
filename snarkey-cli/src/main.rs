use clap::{Arg, Command};
use snarkey_core::encryption::{encrypt, decrypt};
use snarkey_core::circuit::encryption_circuit::verify_encryption;

fn main() {
    let matches = Command::new("SNARKey CLI")
        .version("1.0")
        .author("Your Name")
        .about("CLI for symmetric encryption using Poseidon hash")
        .subcommand(Command::new("encrypt")
            .about("Encrypt a message")
            .arg(Arg::new("message")
                .help("The plaintext message to encrypt")
                .required(true)))
        .subcommand(Command::new("decrypt")
            .about("Decrypt a message")
            .arg(Arg::new("ciphertext")
                .help("The encrypted message")
                .required(true)))
        .subcommand(Command::new("verify")
            .about("Verify an encryption proof")
            .arg(Arg::new("plaintext")
                .help("Original plaintext message")
                .required(true))
            .arg(Arg::new("ciphertext")
                .help("Corresponding encrypted message")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_m)) => {
            let message = sub_m.get_one::<String>("message").unwrap();
            let encrypted = encrypt::encrypt_message(message);
            println!("Encrypted: {:?}", encrypted);
        }
        Some(("decrypt", sub_m)) => {
            let ciphertext = sub_m.get_one::<String>("ciphertext").unwrap();
            match decrypt::decrypt_message(ciphertext) {
                Ok(plain) => println!("Decrypted: {}", plain),
                Err(e) => println!("Decryption failed: {:?}", e),
            }
        }
        Some(("verify", sub_m)) => {
            let plaintext = sub_m.get_one::<String>("plaintext").unwrap();
            let ciphertext = sub_m.get_one::<String>("ciphertext").unwrap();
            let result = verify_encryption(plaintext.as_bytes(), ciphertext.as_bytes());
            println!("Verification result: {}", result);
        }
        _ => println!("Use --help for options."),
    }
}
