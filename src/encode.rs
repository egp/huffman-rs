// src/encode.rs v3

//! Huffman encoder with self-contained file format.

use crate::bitstream::BitWriter;
use crate::code::build_codes;
use crate::freq::build_freq;
use crate::tree::build_tree;

const MAGIC: &[u8; 2] = b"HR";
const VERSION: u8 = 1;

pub fn encode(input: &[u8], passphrase: &str) -> Vec<u8> {
    let freq = build_freq(input);
    let tree = match build_tree(&freq) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let codes = build_codes(&tree);

    let mut writer = BitWriter::new();

    for &b in input {
        writer.write_code(codes[b as usize]);
    }

    let mut payload = writer.finish();

    // -----------------------
    // XOR layer (payload only)
    // -----------------------
    let xor_enabled = !passphrase.is_empty();
    if xor_enabled {
        xor_inplace(&mut payload, passphrase.as_bytes());
    }

    // -----------------------
    // HEADER
    // -----------------------
    let mut out = Vec::new();

    // magic
    out.extend_from_slice(MAGIC);

    // version
    out.push(VERSION);

    // flags
    let mut flags = 0u8;
    if xor_enabled {
        flags |= 1;
    }
    out.push(flags);

    // original length
    out.extend_from_slice(&(input.len() as u32).to_le_bytes());

    // freq table
    for &f in &freq {
        out.extend_from_slice(&f.to_le_bytes());
    }

    // payload
    out.extend_from_slice(&payload);

    out
}

// simple repeating XOR (MVP)
fn xor_inplace(data: &mut [u8], key: &[u8]) {
    for (i, b) in data.iter_mut().enumerate() {
        *b ^= key[i % key.len()];
    }
}

// src/encode.rs v3
