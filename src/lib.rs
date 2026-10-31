//! # alloy-trie
//!
//! Rayon-parallel Merkle Patricia Trie (MPT) for Ethereum state root computation.
//!
//! ## Node types
//!
//! ```text
//! MPT Node
//! ├── Leaf   { key_suffix: Nibbles, value: Vec<u8> }
//! ├── Branch { children: [Option<Box<Node>>; 16], value: Option<Vec<u8>> }
//! └── Extension { shared_prefix: Nibbles, child: Box<Node> }
//! ```

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod hash;
pub mod nibbles;
pub mod node;
pub mod trie;
