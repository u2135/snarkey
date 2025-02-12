pub fn poseidon_hash(input: &str) -> u64 {
    let hash = seahash::hash(input.as_bytes());
    hash
}
