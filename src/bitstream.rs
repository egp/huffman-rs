pub struct BitWriter {
    buffer: Vec<u8>,
    current: u8,
    bits_filled: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current: 0,
            bits_filled: 0,
        }
    }

    pub fn write_bit(&mut self, bit: bool) {
        self.current <<= 1;
        if bit {
            self.current |= 1;
        }
        self.bits_filled += 1;

        if self.bits_filled == 8 {
            self.buffer.push(self.current);
            self.current = 0;
            self.bits_filled = 0;
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.bits_filled > 0 {
            self.current <<= 8 - self.bits_filled;
            self.buffer.push(self.current);
        }
        self.buffer
    }
}