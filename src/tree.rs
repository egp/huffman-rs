use std::cmp::Ordering;
use std::collections::BinaryHeap;

// src/tree.rs v1

#[derive(Debug)]
pub struct Node {
    pub freq: u32,
    pub symbol: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct HeapNode(pub Box<Node>);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq == other.0.freq
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.freq.cmp(&self.0.freq))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq.cmp(&self.0.freq)
    }
}

pub fn build_tree(freq: &[u32; 256]) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (i, &f) in freq.iter().enumerate() {
        if f > 0 {
            heap.push(HeapNode(Box::new(Node {
                freq: f,
                symbol: Some(i as u8),
                left: None,
                right: None,
            })));
        }
    }

    if heap.is_empty() {
        return None;
    }

    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;

        let merged = Node {
            freq: a.freq + b.freq,
            symbol: None,
            left: Some(a),
            right: Some(b),
        };

        heap.push(HeapNode(Box::new(merged)));
    }

    Some(heap.pop().unwrap().0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_invariants_leaf() {
        let n = Node {
            freq: 1,
            symbol: Some(10),
            left: None,
            right: None,
        };

        assert_eq!(n.symbol, Some(10));
        assert!(n.left.is_none());
        assert!(n.right.is_none());
    }
}
// src/tree.rs v1
