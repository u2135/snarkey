use criterion::{criterion_group, criterion_main, Criterion};
use snarkey_core::encryption::{encrypt, decrypt};

fn encryption_benchmark(c: &mut Criterion) {
    let message = "Hello, world!";
    
    c.bench_function("encrypt and decrypt", |b| b.iter(|| {
        let encrypted = encrypt::encrypt_message(message);
        let decrypted = decrypt::decrypt_message(&encrypted).unwrap();
        assert_eq!(message, decrypted);
    }));
}

criterion_group!(benches, encryption_benchmark);
criterion_main!(benches);
