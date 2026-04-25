// tests/roundtrip.rs v2
use huffman_rs::freq::build_freq;
use huffman_rs::tree::build_tree;
use huffman_rs::{decode::decode, encode::encode};

#[test]
fn roundtrip_repetitive() {
    let data = b"aaaaaa";

    let freq = build_freq(data);
    let tree = build_tree(&freq).unwrap();

    let encoded = encode(data);
    let decoded = decode(&tree, &encoded, data.len());

    assert!(!encoded.is_empty());
    assert_ne!(encoded, data);
    assert_eq!(decoded, data);
}

#[test]
fn encoding_changes_data() {
    let data = b"hello world";
    let encoded = encode(data);

    assert_ne!(encoded, data);
}

// tests/roundtrip.rs v2
