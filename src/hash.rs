//! Keccak-256 hashing stub for MPT node hashing.
//!
//! In production this delegates to a Keccak-256 implementation.
//! The stub uses a deterministic mock so tests are reproducible without
//! pulling in external crypto dependencies.

/// A 32-byte Keccak-256 digest.
pub type Hash = [u8; 32];

/// The hash of an empty input `keccak256("")`.
pub const EMPTY_HASH: Hash = [
    0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c,
    0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
    0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b,
    0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
];

/// Compute a deterministic hash of the given bytes.
///
/// **Stub implementation:** XORs all bytes into a 32-byte pattern.
/// Replace with `keccak256` in production.
pub fn hash(data: &[u8]) -> Hash {
    let mut out = [0u8; 32];
    for (i, &b) in data.iter().enumerate() {
        out[i % 32] ^= b;
    }
    out
}

/// Combine two hashes (used when hashing branch children).
pub fn hash_pair(left: &Hash, right: &Hash) -> Hash {
    let mut combined = [0u8; 64];
    combined[..32].copy_from_slice(left);
    combined[32..].copy_from_slice(right);
    hash(&combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hash_constant() {
        // The stub hash of empty input is all-zeros (XOR nothing = 0)
        assert_eq!(hash(&[]), [0u8; 32]);
    }

    #[test]
    fn hash_is_deterministic() {
        let data = b"wikialisalabs";
        assert_eq!(hash(data), hash(data));
    }

    #[test]
    fn hash_pair_differs_from_single() {
        let h1 = hash(b"a");
        let h2 = hash(b"b");
        assert_ne!(hash_pair(&h1, &h2), h1);
    }
}
