//! Nibble sequences — the key encoding used in MPT nodes.

/// A nibble is a 4-bit value (0x0..=0xf).
pub type Nibble = u8;

/// A sequence of nibbles derived from a byte key.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Nibbles(Vec<Nibble>);

impl Nibbles {
    /// Construct nibbles from a byte slice (each byte → 2 nibbles, big-endian).
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut nibbles = Vec::with_capacity(bytes.len() * 2);
        for &b in bytes {
            nibbles.push(b >> 4);
            nibbles.push(b & 0x0f);
        }
        Self(nibbles)
    }

    /// Length in nibbles.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the nibble at position `i`.
    pub fn get(&self, i: usize) -> Option<Nibble> {
        self.0.get(i).copied()
    }

    /// Return the common prefix length between `self` and `other`.
    pub fn common_prefix_len(&self, other: &Self) -> usize {
        self.0
            .iter()
            .zip(other.0.iter())
            .take_while(|(a, b)| a == b)
            .count()
    }

    /// Slice from `start` to end.
    pub fn slice_from(&self, start: usize) -> Self {
        Self(self.0[start..].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_length() {
        let n = Nibbles::from_bytes(&[0xab, 0xcd]);
        assert_eq!(n.len(), 4);
        assert_eq!(n.get(0), Some(0xa));
        assert_eq!(n.get(1), Some(0xb));
        assert_eq!(n.get(2), Some(0xc));
        assert_eq!(n.get(3), Some(0xd));
    }

    #[test]
    fn common_prefix() {
        let a = Nibbles::from_bytes(&[0xab, 0xcd]);
        let b = Nibbles::from_bytes(&[0xab, 0xef]);
        assert_eq!(a.common_prefix_len(&b), 2); // 'a', 'b' match
    }
}
