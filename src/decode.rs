// src/decode.rs v3

//! Huffman decoder module.
//!
//! Handles both normal and degenerate (single-symbol) Huffman trees.

use crate::bitstream::BitReader;
use crate::tree::Node;

/// Decode Huffman bitstream using tree traversal.
///
/// # Parameters
/// - `root`: Huffman tree root
/// - `input`: encoded bytes
/// - `expected_len`: number of symbols to output
pub fn decode(root: &Node, input: &[u8], expected_len: usize) -> Vec<u8> {
    // ----------------------------
    // EDGE CASE: single-symbol tree
    // ----------------------------
    if root.left.is_none() && root.right.is_none() {
        if let Some(sym) = root.symbol {
            return vec![sym; expected_len];
        }
    }

    let mut reader = BitReader::new(input.to_vec());
    let mut output = Vec::with_capacity(expected_len);

    let mut node = root;

    while output.len() < expected_len {
        let bit = match reader.read_bit() {
            Some(b) => b,
            None => break,
        };

        node = if bit == 0 {
            node.left
                .as_ref()
                .expect("invalid Huffman tree: missing left child")
        } else {
            node.right
                .as_ref()
                .expect("invalid Huffman tree: missing right child")
        };

        if let Some(sym) = node.symbol {
            output.push(sym);
            node = root;
        }
    }

    output
}
// src/decode.rs v3
