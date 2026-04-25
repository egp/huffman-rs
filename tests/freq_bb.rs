use huffman_rs::freq::build_freq;

#[test]
fn empty_input_all_zero() {
    let f = build_freq(&[]);
    assert_eq!(f, [0u32; 256]);
}

#[test]
fn single_symbol() {
    let f = build_freq(b"a");
    assert_eq!(f[b'a' as usize], 1);
}
