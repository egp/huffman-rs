// src/code.rs v1

//! Huffman code generation module.
//!
//! Converts a Huffman tree into a prefix-free code table using
//! deterministic DFS traversal. Each leaf node is assigned a binary
//! code based on its path from the root.
//!
//! Encoding rule:
//! - left edge = 0
//! - right edge = 1

use crate::tree::Node;

/// Represents a Huffman bit code.
///
/// # Representation
/// - `bits`: encoded path (LSB-aligned)
/// - `len`: number of valid bits in `bits`
///
/// # Invariant
/// Codes are prefix-free by construction of Huffman tree.
#[derive(Debug, Clone, Copy)]
pub struct Code {
    pub bits: u64,
    pub len: u8,
}

/// Builds a full 256-entry code table from a Huffman tree.
///
/// # Arguments
/// * `root` - root of Huffman tree
///
/// # Returns
/// Array indexed by symbol (0–255), containing Huffman codes.
/// Missing symbols remain default (0,0).
pub fn build_codes(root: &Node) -> [Code; 256] {
    let mut table = [Code { bits: 0, len: 0 }; 256];

    fn dfs(node: &Node, table: &mut [Code; 256], bits: u64, depth: u8) {
        match (&node.symbol, &node.left, &node.right) {
            // leaf node
            (Some(sym), None, None) => {
                table[*sym as usize] = Code { bits, len: depth };
            }

            // internal node
            (_, Some(left), Some(right)) => {
                dfs(left, table, bits << 1, depth + 1);
                dfs(right, table, (bits << 1) | 1, depth + 1);
            }

            _ => {
                // invalid node state (should never happen if tree is correct)
            }
        }
    }

    dfs(root, &mut table, 0, 0);

    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;

    #[test]
    fn single_symbol_code_is_zero() {
        let root = Node {
            freq: 1,
            symbol: Some(b'a'),
            left: None,
            right: None,
        };

        let codes = build_codes(&root);

        assert_eq!(codes[b'a' as usize].len, 0);
    }
}
// src/code.rs v1
