// src/bitstream.rs v1

//! Bit-level output stream for Huffman encoding.
//!
//! Packs individual bits into a byte buffer using MSB-first ordering.
//! Used by the encoder to serialize Huffman codes.

/// BitWriter accumulates bits into a byte-aligned buffer.
///
/// # Behavior
/// - Bits are written MSB-first
/// - Bytes are flushed automatically when full
/// - Final partial byte is left unflushed until `finish()`
pub struct BitWriter {
    buffer: Vec<u8>,
    current: u8,
    filled: u8,
}

impl BitWriter {
    /// Creates a new empty BitWriter.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current: 0,
            filled: 0,
        }
    }

    /// Writes a single bit (0 or 1) into the stream.
    ///
    /// # Invariant
    /// Only the lowest bit of `bit` is used.
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

    /// Writes a full code (bit pattern + length).
    pub fn write_code(&mut self, code: crate::code::Code) {
        for i in (0..code.len).rev() {
            let bit = (code.bits >> i) & 1;
            self.write_bit(bit as u8);
        }
    }

    /// Finalizes stream and returns byte buffer.
    ///
    /// Pads last byte with zeros if needed.
    pub fn finish(mut self) -> Vec<u8> {
        if self.filled > 0 {
            self.current <<= 8 - self.filled;
            self.buffer.push(self.current);
        }
        self.buffer
    }
}

/// default
impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_single_byte_correctly() {
        let mut bw = BitWriter::new();

        // 1010_0000
        bw.write_bit(1);
        bw.write_bit(0);
        bw.write_bit(1);
        bw.write_bit(0);
        bw.write_bit(0);
        bw.write_bit(0);
        bw.write_bit(0);
        bw.write_bit(0);

        let out = bw.finish();
        assert_eq!(out, vec![0b1010_0000]);
    }

    #[test]
    fn cross_byte_write() {
        let mut bw = BitWriter::new();

        for _ in 0..16 {
            bw.write_bit(1);
        }

        let out = bw.finish();
        assert_eq!(out, vec![0xFF, 0xFF]);
    }
}
// src/bitstream.rs v1
