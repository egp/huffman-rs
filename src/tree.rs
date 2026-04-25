#[derive(Debug)]
pub struct Node {
    pub freq: u32,
    pub symbol: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}