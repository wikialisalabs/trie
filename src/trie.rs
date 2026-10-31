//! High-level MPT trie: insert, get, root hash.

use crate::hash::Hash;
use crate::nibbles::Nibbles;
use crate::node::{empty_branch, Node};
use std::collections::HashMap;

/// A Merkle Patricia Trie.
#[derive(Debug, Default)]
pub struct MerkleTrie {
    /// Key-value store backing the trie.
    store: HashMap<Vec<u8>, Vec<u8>>,
}

impl MerkleTrie {
    /// Create an empty trie.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a key-value pair.
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.store.insert(key, value);
    }

    /// Get the value for a key, if present.
    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.store.get(key)
    }

    /// Remove a key.
    pub fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.store.remove(key)
    }

    /// Compute the Merkle root hash of all key-value pairs.
    ///
    /// Builds a trie structure over sorted keys and returns the root hash.
    pub fn root(&self) -> Hash {
        if self.store.is_empty() {
            return [0u8; 32];
        }

        // Build leaf nodes sorted by key for determinism
        let mut entries: Vec<_> = self.store.iter().collect();
        entries.sort_by_key(|(k, _)| k.as_slice());

        // Build branch at root level (first nibble fanout)
        let mut root = empty_branch();
        for (key, value) in entries {
            let nibbles = Nibbles::from_bytes(key);
            Self::insert_into(&mut root, nibbles, 0, value.clone());
        }
        root.hash()
    }

    fn insert_into(node: &mut Node, key: Nibbles, depth: usize, value: Vec<u8>) {
        match node {
            Node::Branch { children, .. } => {
                if depth >= key.len() {
                    return;
                }
                let nibble = key.get(depth).unwrap() as usize;
                if children[nibble].is_none() {
                    children[nibble] = Some(Box::new(Node::Leaf {
                        key: key.slice_from(depth + 1),
                        value,
                    }));
                } else if let Some(child) = &mut children[nibble] {
                    Self::insert_into(child, key, depth + 1, value);
                }
            }
            Node::Leaf { value: existing, .. } => {
                *existing = value; // overwrite
            }
            Node::Extension { child, .. } => {
                Self::insert_into(child, key, depth, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_root() {
        let t = MerkleTrie::new();
        assert_eq!(t.root(), [0u8; 32]);
    }

    #[test]
    fn insert_and_get() {
        let mut t = MerkleTrie::new();
        t.insert(b"key".to_vec(), b"value".to_vec());
        assert_eq!(t.get(b"key"), Some(&b"value".to_vec()));
    }

    #[test]
    fn root_changes_on_insert() {
        let mut t = MerkleTrie::new();
        let r0 = t.root();
        t.insert(b"a".to_vec(), b"1".to_vec());
        let r1 = t.root();
        assert_ne!(r0, r1);
    }

    #[test]
    fn root_is_deterministic() {
        let mut t1 = MerkleTrie::new();
        t1.insert(b"k1".to_vec(), b"v1".to_vec());
        t1.insert(b"k2".to_vec(), b"v2".to_vec());

        let mut t2 = MerkleTrie::new();
        t2.insert(b"k1".to_vec(), b"v1".to_vec());
        t2.insert(b"k2".to_vec(), b"v2".to_vec());

        assert_eq!(t1.root(), t2.root());
    }
}
