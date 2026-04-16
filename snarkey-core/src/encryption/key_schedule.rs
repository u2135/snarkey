pub fn generate_key(seed: &str) -> u64 {
    let hash = seahash::hash(seed.as_bytes());
    hash
}
