//! trait implementations for F2_128 — tiers 1, 2, 4.
//! in characteristic 2: sub = add = XOR, neg = identity.

use crate::tower::F2_128;
use strata_core::{Codec, Field, Ring, Semiring};
use strata_ext::Batch;
use strata_proof::{Dot, Reduce};

extern crate alloc;
use alloc::vec::Vec;

// ── tier 1 ───────────────────────────────────────────────────────

impl Codec for F2_128 {
    fn byte_len() -> usize {
        16
    }
    fn encode(&self, buf: &mut [u8]) {
        buf[..16].copy_from_slice(&self.0.to_le_bytes());
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 16 {
            return None;
        }
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&bytes[..16]);
        Some(F2_128(u128::from_le_bytes(buf)))
    }
}

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
    #[inline]
    fn square(self) -> Self {
        F2_128::square(self)
    }
    fn sqrt(self) -> Option<Self> {
        // in char 2, every element has a unique square root
        Some(F2_128::sqrt(self))
    }
}

// ── tier 2 ───────────────────────────────────────────────────────

impl Reduce for F2_128 {
    fn reduce(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 16, "need at least 16 bytes for F2_128");
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&bytes[..16]);
        F2_128(u128::from_le_bytes(buf))
    }
}

impl Dot for F2_128 {}

// ── tier 4 ───────────────────────────────────────────────────────

impl Batch for F2_128 {
    fn batch_inv(elements: &mut [Self]) {
        let input = elements.to_vec();
        crate::batch::batch_inv_128(&input, elements);
    }
}
