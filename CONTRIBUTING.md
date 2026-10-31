# Hướng dẫn Đóng góp — `trie`

Cảm ơn bạn đã quan tâm đến **`alloy-trie`**! MPT là cấu trúc dữ liệu có tính đúng đắn cực kỳ quan trọng — mọi thay đổi phải đi kèm tests đầy đủ.

---

## Mục lục

1. [Môi trường Phát triển](#1-môi-trường-phát-triển)
2. [Cấu trúc Repository](#2-cấu-trúc-repository)
3. [Quy trình Đóng góp](#3-quy-trình-đóng-góp)
4. [Tiêu chuẩn Code](#4-tiêu-chuẩn-code)
5. [Viết Tests](#5-viết-tests)
6. [Thêm Tính năng Parallel](#6-thêm-tính-năng-parallel)
7. [Commit Convention](#7-commit-convention)
8. [Pull Request Checklist](#8-pull-request-checklist)

---

## 1. Môi trường Phát triển

```bash
git clone https://github.com/wikialisalabs/trie.git
cd trie
rustup show
cargo build
cargo test
```

---

## 2. Cấu trúc Repository

```
trie/
├── src/
│   ├── lib.rs       # Crate root, feature gates
│   ├── nibbles.rs   # Nibbles key encoding
│   ├── hash.rs      # Keccak-256 wrapper
│   ├── node.rs      # Node enum (Leaf / Extension / Branch)
│   └── trie.rs      # MerkleTrie public API
├── Cargo.toml
├── README.md
└── CONTRIBUTING.md
```

---

## 3. Quy trình Đóng góp

1. Kiểm tra [Issues](https://github.com/wikialisalabs/trie/issues).
2. Fork → branch `feat/<tên>`.
3. Implement + tests (xem mục 5).
4. Chạy CI cục bộ.
5. Mở PR.

---

## 4. Tiêu chuẩn Code

- **Determinism:** `trie.root()` với cùng dữ liệu phải luôn trả cùng kết quả, bất kể thứ tự insert.
- **No silent data loss:** `insert` phải overwrite đúng giá trị cũ, không bao giờ mất dữ liệu.
- **Hash correctness:** Chỉ thay stub hash bằng Keccak-256 thật khi có test vector từ official Ethereum tests.
- Không dùng `unsafe`.

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 5. Viết Tests

```bash
# Unit tests
cargo test

# Test specific module
cargo test nibbles
cargo test trie::tests
```

### Yêu cầu tối thiểu

| Thay đổi | Tests bắt buộc |
|----------|----------------|
| Thay đổi `nibbles` | Encoding correctness, common prefix, slice |
| Thay đổi `node` | Leaf hash, Branch hash, Extension hash |
| Thay đổi `trie` | Empty root, insert+get, root determinism, overwrite |

### Test vector từ Ethereum

Khi thay thế stub hash bằng Keccak thật, phải pass test vector từ:
- [ethereum/tests/TrieTests](https://github.com/ethereum/tests/tree/develop/TrieTests)

---

## 6. Thêm Tính năng Parallel

Parallel hashing (Rayon) chỉ được implement với feature flag `parallel`:

```rust
#[cfg(feature = "parallel")]
use rayon::prelude::*;
```

Mọi code parallel **không được** thay đổi kết quả so với sequential. Thêm test:

```rust
#[test]
#[cfg(feature = "parallel")]
fn parallel_root_equals_sequential() { ... }
```

---

## 7. Commit Convention

```
feat(trie): implement trie.remove() with node rebalancing
fix(nibbles): correct slice_from index boundary
test(trie): add ethereum test vectors for root computation
perf(trie): add rayon parallel hashing under parallel feature
```

---

## 8. Pull Request Checklist

```
[ ] cargo fmt --all                          → no diff
[ ] cargo clippy --all-features -- -D warnings → 0 warnings
[ ] cargo test --all-features                → all pass
[ ] Root determinism verified                ✓
[ ] No data loss on insert/overwrite         ✓
[ ] Parallel result == sequential result     ✓ (if parallel touched)
[ ] PR linked to Issue                       ✓
```

---

## Liên hệ

- **Email:** root@wikialisalabs.axvn.vn
- **Issues:** https://github.com/wikialisalabs/trie/issues

*Cảm ơn bạn đã đóng góp! 🦀🌳*
