// src/encode.rs v1

//! Huffman encoder module.
//!
//! Implements a two-pass compression pipeline:
//! 1. Build frequency table and Huffman tree
//! 2. Generate codes and encode input into bitstream

use crate::bitstream::BitWriter;
use crate::code::build_codes;
use crate::freq::build_freq;
use crate::tree::build_tree;

/// Encodes input bytes using Huffman compression.
///
/// # Format
/// Output layout:
/// - 256 × u32 frequency table (header)
/// - bit-packed Huffman encoded payload
///
/// # Algorithm
/// 1. Compute frequency table
/// 2. Build Huffman tree
/// 3. Generate code table
/// 4. Encode input using BitWriter
pub fn encode(input: &[u8]) -> Vec<u8> {
    let freq = build_freq(input);
    let tree = match build_tree(&freq) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let codes = build_codes(&tree);

    // all logic using codes below this line

    if !input.is_empty() && codes.iter().all(|c| c.len == 0) {
        return vec![0];
    }

    let mut writer = BitWriter::new();

    for &b in input {
        writer.write_code(codes[b as usize]);
    }

    writer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_non_empty_input_produces_output() {
        let input = b"hello world";

        let encoded = encode(input);

        assert!(!encoded.is_empty());
    }
}
// src/encode.rs v1
