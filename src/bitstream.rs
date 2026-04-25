// src/bitstream.rs v3

//! Bit-level stream utilities for Huffman encoding/decoding.
//!
//! Provides BitWriter (encoding) and BitReader (decoding).

// =======================
// BitWriter (ENCODING)
// =======================

pub struct BitWriter {
    buffer: Vec<u8>,
    current: u8,
    filled: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current: 0,
            filled: 0,
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        self.current <<= 1;
        self.current |= bit & 1;
        self.filled += 1;

        if self.filled == 8 {
            self.buffer.push(self.current);
            self.current = 0;
            self.filled = 0;
        }
    }

    pub fn write_code(&mut self, code: crate::code::Code) {
        for i in (0..code.len).rev() {
            let bit = (code.bits >> i) & 1;
            self.write_bit(bit as u8);
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.filled > 0 {
            self.current <<= 8 - self.filled;
            self.buffer.push(self.current);
        }
        self.buffer
    }
}

impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

// =======================
// BitReader (DECODING)
// =======================

pub struct BitReader {
    data: Vec<u8>,
    byte_pos: usize,
    bit_pos: u8,
}

impl BitReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    pub fn read_bit(&mut self) -> Option<u8> {
        if self.byte_pos >= self.data.len() {
            return None;
        }

        let byte = self.data[self.byte_pos];

        let bit = (byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos += 1;

        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }

        Some(bit)
    }
}
// src/bitstream.rs v3
