//! MPT node types: Leaf, Extension, Branch.

use crate::hash::{hash, Hash};
use crate::nibbles::Nibbles;

/// An MPT node.
#[derive(Debug, Clone)]
pub enum Node {
    /// Leaf node: stores a value at the end of a key suffix.
    Leaf {
        /// Remaining key suffix (nibbles).
        key: Nibbles,
        /// Stored value.
        value: Vec<u8>,
    },
    /// Extension node: shared prefix pointing to a single child.
    Extension {
        /// Shared nibble prefix.
        prefix: Nibbles,
        /// Child node.
        child: Box<Node>,
    },
    /// Branch node: 16-way fanout plus optional inline value.
    Branch {
        /// 16 children indexed by nibble (0x0..=0xf).
        children: [Option<Box<Node>>; 16],
        /// Optional value stored at this branch.
        value: Option<Vec<u8>>,
    },
}

impl Node {
    /// Compute the hash of this node.
    pub fn hash(&self) -> Hash {
        match self {
            Node::Leaf { key, value } => {
                let mut data = Vec::new();
                for i in 0..key.len() {
                    data.push(key.get(i).unwrap_or(0));
                }
                data.extend_from_slice(value);
                hash(&data)
            }
            Node::Extension { prefix, child } => {
                let child_hash = child.hash();
                let mut data = Vec::new();
                for i in 0..prefix.len() {
                    data.push(prefix.get(i).unwrap_or(0));
                }
                data.extend_from_slice(&child_hash);
                hash(&data)
            }
            Node::Branch { children, value } => {
                let mut data = Vec::new();
                for child in children.iter() {
                    if let Some(c) = child {
                        data.extend_from_slice(&c.hash());
                    } else {
                        data.extend_from_slice(&[0u8; 32]);
                    }
                }
                if let Some(v) = value {
                    data.extend_from_slice(v);
                }
                hash(&data)
            }
        }
    }
}

/// Create a new empty branch node.
pub fn empty_branch() -> Node {
    const NONE: Option<Box<Node>> = None;
    Node::Branch {
        children: [NONE; 16],
        value: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_hash_deterministic() {
        let leaf = Node::Leaf {
            key: Nibbles::from_bytes(&[0xab]),
            value: b"hello".to_vec(),
        };
        assert_eq!(leaf.hash(), leaf.hash());
    }

    #[test]
    fn branch_no_children_hash() {
        let branch = empty_branch();
        assert_eq!(branch.hash(), branch.hash());
    }
}
