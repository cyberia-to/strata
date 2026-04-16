//! trait implementations for Tropical — tier 1 only (Semiring + Encode).
//! Tropical has no subtraction or inversion — Semiring is the highest level.

use crate::element::Tropical;
use core::ops::{Add, AddAssign, Mul, MulAssign};
use cyb_algebra::{Encode, Semiring};

// ── std::ops (required by Semiring) ──────────────────────────────

impl Add for Tropical {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Tropical::add(self, rhs) // min
    }
}

impl AddAssign for Tropical {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Tropical::add(*self, rhs);
    }
}

impl Mul for Tropical {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Tropical::mul(self, rhs) // saturating add
    }
}

impl MulAssign for Tropical {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Tropical::mul(*self, rhs);
    }
}

// ── tier 1 ───────────────────────────────────────────────────────

impl Encode for Tropical {
    fn byte_len() -> usize {
        8
    }
    fn encode(&self, buf: &mut [u8]) {
        buf[..8].copy_from_slice(&self.0.to_le_bytes());
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[..8]);
        Some(Tropical(u64::from_le_bytes(buf)))
    }
}

impl Semiring for Tropical {
    const ZERO: Self = Tropical::ZERO; // +inf (additive identity for min)
    const ONE: Self = Tropical::ONE; // 0 (multiplicative identity for +)
}
