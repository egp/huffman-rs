pub fn build_freq(input: &[u8]) -> [u32; 256] {
    let mut freq = [0u32; 256];
    for &b in input {
        freq[b as usize] += 1;
    }
    freq
}