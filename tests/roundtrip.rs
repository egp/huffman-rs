// tests/roundtrip.rs v3

use huffman_rs::{decode::decode, encode::encode};

#[test]
fn roundtrip_repetitive() {
    let data = b"aaaaaa";

    let encoded = encode(data, "");
    let decoded = decode(&encoded, "");

    assert!(!encoded.is_empty());
    assert_ne!(encoded, data);
    assert_eq!(decoded, data);
}

#[test]
fn encoding_changes_data() {
    let data = b"hello world";

    let encoded = encode(data, "");

    assert_ne!(encoded, data);
}

#[test]
fn roundtrip_with_passphrase() {
    let data = b"hello world";

    let encoded = encode(data, "This is a passphrase");
    let decoded = decode(&encoded, "This is a passphrase");

    assert_eq!(decoded, data);
}

#[test]
fn wrong_passphrase_produces_garbage() {
    let data = b"hello world";

    let encoded = encode(data, "correct");
    let decoded = decode(&encoded, "wrong");

    assert_ne!(decoded, data);
}

// tests/roundtrip.rs v3
