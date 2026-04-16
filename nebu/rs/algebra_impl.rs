//! Semiring, Ring, Field implementations for Goldilocks.

use crate::field::Goldilocks;
use cyb_algebra::{Field, Ring, Semiring};

impl Semiring for Goldilocks {
    const ZERO: Self = Goldilocks::ZERO;
    const ONE: Self = Goldilocks::ONE;
}

impl Ring for Goldilocks {}

impl Field for Goldilocks {
    #[inline]
    fn inv(self) -> Self {
        Goldilocks::inv(self)
    }

    fn from_hash(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 8, "need at least 8 bytes for Goldilocks");
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[..8]);
        let val = u64::from_le_bytes(buf);
        Goldilocks::new(val).canonicalize()
    }
}
