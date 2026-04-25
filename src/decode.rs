// src/decode.rs v4

//! Self-contained Huffman decoder with header parsing.

use crate::bitstream::BitReader;
use crate::tree::{build_tree, Node};

const MAGIC: &[u8; 2] = b"HR";

pub fn decode(input: &[u8], passphrase: &str) -> Vec<u8> {
    if input.len() < 2 + 1 + 1 + 4 + 1024 {
        return Vec::new(); // invalid
    }

    let mut pos = 0;

    // -----------------------
    // HEADER PARSE
    // -----------------------

    // magic
    if &input[pos..pos + 2] != MAGIC {
        return Vec::new();
    }
    pos += 2;

    // version (unused for now)
    let _version = input[pos];
    pos += 1;

    // flags
    let flags = input[pos];
    pos += 1;

    let xor_enabled = (flags & 1) != 0;

    // original length
    let orig_len = u32::from_le_bytes(input[pos..pos + 4].try_into().unwrap()) as usize;
    pos += 4;

    // freq table
    let mut freq = [0u32; 256];
    for f in freq.iter_mut() {
        *f = u32::from_le_bytes(input[pos..pos + 4].try_into().unwrap());
        pos += 4;
    }

    // payload
    let mut payload = input[pos..].to_vec();

    // -----------------------
    // XOR reverse
    // -----------------------
    if xor_enabled && !passphrase.is_empty() {
        xor_inplace(&mut payload, passphrase.as_bytes());
    }

    // -----------------------
    // TREE RECONSTRUCTION
    // -----------------------
    let tree = match build_tree(&freq) {
        Some(t) => t,
        None => return Vec::new(),
    };

    // -----------------------
    // DECODE
    // -----------------------
    decode_with_tree(&tree, &payload, orig_len)
}

// -----------------------
// internal decode
// -----------------------

fn decode_with_tree(root: &Node, input: &[u8], expected_len: usize) -> Vec<u8> {
    // single-symbol case
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
            node.left.as_ref().unwrap()
        } else {
            node.right.as_ref().unwrap()
        };

        if let Some(sym) = node.symbol {
            output.push(sym);
            node = root;
        }
    }

    output
}

// simple repeating XOR (same as encoder)
fn xor_inplace(data: &mut [u8], key: &[u8]) {
    for (i, b) in data.iter_mut().enumerate() {
        *b ^= key[i % key.len()];
    }
}

// src/decode.rs v4
