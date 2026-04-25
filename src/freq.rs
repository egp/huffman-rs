// src/freq.rs v1
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

    #[test]
    fn freq_counts_correctly() {
        let f = build_freq(b"aba");
        assert_eq!(f[b'a' as usize], 2);
        assert_eq!(f[b'b' as usize], 1);
    }
}
// src/freq.rs v1