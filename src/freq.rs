// src/freq.rs v2
//! Frequency analysis module for Huffman encoding.
//!
//! Computes a deterministic byte-frequency table over a 256-symbol alphabet.
//! The output is used as the input distribution for Huffman tree construction.

/// Builds a frequency table from input bytes.
///
/// # Contract
/// - Input: arbitrary byte slice
/// - Output: fixed-size `[u32; 256]` frequency histogram
/// - Deterministic: identical input produces identical output
/// - Total complexity: O(n)
///
/// # Invariants
/// - All 256 entries are always present
/// - Unused symbols remain zero
pub fn build_freq(input: &[u8]) -> [u32; 256] {
    let mut freq = [0u32; 256];
    for &b in input {
        freq[b as usize] += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies correct counting of repeated symbols.
    #[test]
    fn freq_counts_correctly() {
        let f = build_freq(b"aba");
        assert_eq!(f[b'a' as usize], 2);
        assert_eq!(f[b'b' as usize], 1);
    }

    /// Ensures bounded accumulation behavior across small input.
    #[test]
    fn internal_array_bounds() {
        let f = build_freq(b"abc");
        assert!(f.iter().all(|&x| x <= 3));
    }
}
// src/freq.rs v2
