// src/tree.rs v2
//! Huffman tree construction module.
//!
//! Builds a deterministic binary tree from a frequency table using a
//! min-heap priority queue. This tree forms the basis of prefix-free
//! Huffman encoding.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A node in the Huffman tree.
///
/// # Structure
/// - Leaf nodes contain a symbol (`Some(u8)`)
/// - Internal nodes contain `None`
///
/// # Invariants
/// - If `symbol.is_some()`, both children must be `None`
/// - Internal nodes always have two children
#[derive(Debug)]
pub struct Node {
    pub freq: u32,
    pub symbol: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

/// Wrapper enabling min-heap behavior for Huffman nodes.
///
/// Rust's `BinaryHeap` is a max-heap by default, so ordering is reversed.
// src/tree.rs v3

#[derive(Debug)]
pub struct HeapNode(pub Box<Node>);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq == other.0.freq
    }
}

impl Eq for HeapNode {}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse for min-heap behavior
        other.0.freq.cmp(&self.0.freq)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Constructs a deterministic Huffman tree from a frequency table.
///
/// # Algorithm
/// 1. Insert all non-zero frequency symbols into a min-heap
/// 2. Repeatedly merge two lowest-frequency nodes
/// 3. Return the final root node
///
/// # Returns
/// - `Some(root)` if input contains at least one symbol
/// - `None` if frequency table is empty
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

    /// Leaf node invariant: symbol-only node has no children.
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
// src/tree.rs v2
