//! trait implementations for Fq — tiers 1, 2, 4.

use crate::fq::Fq;
use strata_core::{Codec, Field, Ring, Semiring};
use strata_ext::{Batch, Blind};
use strata_proof::{Dot, Reduce};

extern crate alloc;
use alloc::vec::Vec;

// ── tier 1 ───────────────────────────────────────────────────────

impl Codec for Fq {
    fn byte_len() -> usize {
        64
    }
    fn encode(&self, buf: &mut [u8]) {
        for (i, &limb) in self.limbs.iter().enumerate() {
            buf[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_le_bytes());
        }
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 64 {
            return None;
        }
        let mut limbs = [0u64; 8];
        for (i, chunk) in bytes[..64].chunks_exact(8).enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(chunk);
            limbs[i] = u64::from_le_bytes(buf);
        }
        Some(Fq::reduce(&Fq::from_limbs(limbs).limbs))
    }
}

impl Semiring for Fq {
    const ZERO: Self = Fq::ZERO;
    const ONE: Self = Fq::ONE;
}

impl Ring for Fq {}

impl Field for Fq {
    #[inline]
    fn inv(self) -> Self {
        Fq::inv(&self)
    }
    #[inline]
    fn square(self) -> Self {
        Fq::square(&self)
    }
    fn sqrt(self) -> Option<Self> {
        Fq::sqrt(&self)
    }
    fn pow_bytes(self, exp: &[u64]) -> Self {
        // pad or truncate to 8 limbs for Fq::pow_limbs
        let mut limbs = [0u64; 8];
        let n = exp.len().min(8);
        limbs[..n].copy_from_slice(&exp[..n]);
        Fq::pow_limbs(&self, &limbs)
    }
}

// ── tier 2 ───────────────────────────────────────────────────────

impl Reduce for Fq {
    fn reduce(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 64, "need at least 64 bytes for Fq");
        let mut limbs = [0u64; 8];
        for (i, chunk) in bytes[..64].chunks_exact(8).enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(chunk);
            limbs[i] = u64::from_le_bytes(buf);
        }
        Fq::reduce(&Fq::from_limbs(limbs).limbs)
    }
}

impl Dot for Fq {}

// ── tier 4 ───────────────────────────────────────────────────────

impl Batch for Fq {
    fn batch_inv(elements: &mut [Self]) {
        let n = elements.len();
        if n == 0 {
            return;
        }

        let mut prefix = Vec::with_capacity(n);
        let mut acc = Self::ONE;
        for &e in elements.iter() {
            if e == Self::ZERO {
                prefix.push(acc);
            } else {
                acc = acc * e;
                prefix.push(acc);
            }
        }

        let mut inv_acc = acc.inv();

        for i in (1..n).rev() {
            if elements[i] == Self::ZERO {
                continue;
            }
            let inv_i = inv_acc * prefix[i - 1];
            inv_acc = inv_acc * elements[i];
            elements[i] = inv_i;
        }
        if elements[0] != Self::ZERO {
            elements[0] = inv_acc;
        }
    }
}

impl Blind for Fq {
    fn ct_eq(&self, other: &Self) -> bool {
        // constant-time: OR all limb XOR differences
        let mut diff = 0u64;
        for i in 0..8 {
            diff |= self.limbs[i] ^ other.limbs[i];
        }
        diff == 0
    }

    fn ct_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mask = if choice { u64::MAX } else { 0 };
        let mut result = [0u64; 8];
        for i in 0..8 {
            result[i] = (a.limbs[i] & mask) | (b.limbs[i] & !mask);
        }
        Fq::from_limbs(result)
    }

    fn ct_swap(a: &mut Self, b: &mut Self, choice: bool) {
        let mask = if choice { u64::MAX } else { 0 };
        for i in 0..8 {
            let t = (a.limbs[i] ^ b.limbs[i]) & mask;
            a.limbs[i] ^= t;
            b.limbs[i] ^= t;
        }
    }
}
