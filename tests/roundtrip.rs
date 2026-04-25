use huffman_rs::{encode::encode, decode::decode};

#[test]
fn roundtrip_empty() {
    let data = b"";
    assert_eq!(decode(&encode(data)), data);
}

#[test]
fn roundtrip_simple() {
    let data = b"hello world";
    assert_eq!(decode(&encode(data)), data);
}