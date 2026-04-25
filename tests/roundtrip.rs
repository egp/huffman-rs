// tests/roundtrip.rs v2
use huffman_rs::{decode::decode, encode::encode};

#[test]
fn roundtrip_repetitive() {
    let data = b"aaaaaa";
    let encoded = encode(data);

    // Expect compression eventually
    assert!(!encoded.is_empty());
    assert_ne!(encoded, data);

    let decoded = decode(&encoded);
    assert_eq!(decoded, data);
}

#[test]
fn encoding_changes_data() {
    let data = b"hello world";
    let encoded = encode(data);

    assert_ne!(encoded, data); // force real encoding
}

// tests/roundtrip.rs v2
