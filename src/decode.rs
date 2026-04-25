// src/decode.rs v2
pub fn decode(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }
    input[1..].to_vec()
}
// src/decode.rs v2
