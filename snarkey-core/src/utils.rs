pub fn hex_encode(input: &[u8]) -> String {
    hex::encode(input)
}

pub fn hex_decode(input: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(input)
}
