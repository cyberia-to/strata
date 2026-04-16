//! Semiring, Ring, Field implementations for F2_128.
//! In characteristic 2: sub = add = XOR, neg = identity.

use crate::tower::F2_128;
use cyb_algebra::{Field, Ring, Semiring};

impl Semiring for F2_128 {
    const ZERO: Self = F2_128::ZERO;
    const ONE: Self = F2_128::ONE;
}

impl Ring for F2_128 {}

impl Field for F2_128 {
    #[inline]
    fn inv(self) -> Self {
        F2_128::inv(self)
    }

    fn from_hash(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 16, "need at least 16 bytes for F2_128");
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&bytes[..16]);
        F2_128(u128::from_le_bytes(buf))
    }
}
