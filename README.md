# trie

> **Tầng 2 — Data Structures** · Rayon-parallel Merkle Patricia Trie

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---

## Tổng quan

`alloy-trie` implement Merkle Patricia Trie (MPT) — cấu trúc dữ liệu nền tảng để tính Ethereum state root, storage root, và receipt root.

| Module | Nội dung |
|--------|----------|
| `nibbles` | `Nibbles` — key encoding (byte → 2 nibbles) |
| `hash` | Keccak-256 wrapper + `EMPTY_HASH` constant |
| `node` | `Node` enum: `Leaf`, `Extension`, `Branch` |
| `trie` | `MerkleTrie` — insert / get / `root()` |

---

## Cài đặt

```toml
[dependencies]
alloy-trie = { git = "https://github.com/wikialisalabs/trie" }
```

## Sử dụng nhanh

```rust
use alloy_trie::trie::MerkleTrie;

let mut trie = MerkleTrie::new();
trie.insert(b"account:0xdead".to_vec(), b"balance:100".to_vec());
trie.insert(b"account:0xbeef".to_vec(), b"balance:200".to_vec());

let root = trie.root();
println!("State root: {:x?}", root);
```

---

## Feature Flags

| Flag | Mặc định | Mô tả |
|------|----------|-------|
| `std` | ✅ | Standard library |
| `parallel` | ❌ | Rayon parallel node hashing |

---

## Kiểm thử

```bash
cargo test
cargo test --all-features
```

---

## Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Giấy phép

Apache-2.0 © 2026 Wikialisalabs Foundation
