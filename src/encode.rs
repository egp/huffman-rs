// src/encode.rs v2
pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() + 1);
    out.push(0xFF); // simple marker
    out.extend_from_slice(input);
    out
}
// src/encode.rs v2
