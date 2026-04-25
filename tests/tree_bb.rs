use huffman_rs::tree::build_tree;

#[test]
fn single_symbol_tree() {
    let mut freq = [0u32; 256];
    freq[b'a' as usize] = 5;

    let root = build_tree(&freq).unwrap();

    assert_eq!(root.freq, 5);
    assert_eq!(root.symbol, Some(b'a'));
    assert!(root.left.is_none());
    assert!(root.right.is_none());
}

#[test]
fn two_symbol_tree() {
    let mut freq = [0u32; 256];
    freq[b'a' as usize] = 3;
    freq[b'b' as usize] = 2;

    let root = build_tree(&freq).unwrap();

    assert_eq!(root.freq, 5);
    assert!(root.left.is_some());
    assert!(root.right.is_some());
}
