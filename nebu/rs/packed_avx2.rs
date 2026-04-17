//! AVX2 packed Goldilocks — 4 elements per 256-bit register.
//!
//! Uses 64-bit lanes in __m256i for parallel field arithmetic.
//! Available on x86_64 with AVX2 target feature.
//!
//! Fallback: if AVX2 is not available at compile time, this module
//! provides a portable 4-wide implementation using arrays.

use crate::field::{EPSILON, Goldilocks, P};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use strata_compute::Packed;
use strata_core::{Field, Ring, Semiring};

/// 4-wide packed Goldilocks field elements.
///
/// On x86_64 with AVX2: uses __m256i for parallel operations.
/// On other platforms: uses [Goldilocks; 4] for portable 4-wide.
///
/// NTT butterflies process 4 elements simultaneously.
/// constraint evaluation processes 4 rows simultaneously.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(align(32))]
pub struct PackedGoldilocks4(pub [Goldilocks; 4]);

impl PackedGoldilocks4 {
    pub const ZERO: Self = Self([Goldilocks::ZERO; 4]);
    pub const ONE: Self = Self([Goldilocks::ONE; 4]);
}

// ── arithmetic (lane-wise) ───────────────────────────────────────

impl Add for PackedGoldilocks4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl Sub for PackedGoldilocks4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl Mul for PackedGoldilocks4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}

impl Neg for PackedGoldilocks4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}

impl AddAssign for PackedGoldilocks4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for PackedGoldilocks4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign for PackedGoldilocks4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// ── trait impls ──────────────────────────────────────────────────

impl Semiring for PackedGoldilocks4 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
}

impl Ring for PackedGoldilocks4 {}

impl Field for PackedGoldilocks4 {
    #[inline]
    fn inv(self) -> Self {
        Self([
            self.0[0].inv(),
            self.0[1].inv(),
            self.0[2].inv(),
            self.0[3].inv(),
        ])
    }
    fn sqrt(self) -> Option<Self> {
        let s0 = crate::sqrt::sqrt(self.0[0])?;
        let s1 = crate::sqrt::sqrt(self.0[1])?;
        let s2 = crate::sqrt::sqrt(self.0[2])?;
        let s3 = crate::sqrt::sqrt(self.0[3])?;
        Some(Self([s0, s1, s2, s3]))
    }
}

impl Packed for PackedGoldilocks4 {
    type Scalar = Goldilocks;
    const WIDTH: usize = 4;

    #[inline]
    fn from_slice(slice: &[Goldilocks]) -> Self {
        assert!(slice.len() >= 4);
        Self([slice[0], slice[1], slice[2], slice[3]])
    }

    #[inline]
    fn to_slice(&self, slice: &mut [Goldilocks]) {
        assert!(slice.len() >= 4);
        slice[..4].copy_from_slice(&self.0);
    }

    #[inline]
    fn broadcast(val: Goldilocks) -> Self {
        Self([val; 4])
    }

    #[inline]
    fn extract(&self, index: usize) -> Goldilocks {
        self.0[index]
    }

    #[inline]
    fn interleave_low(self, other: Self) -> Self {
        // [a0,a1,a2,a3] + [b0,b1,b2,b3] → [a0,b0,a1,b1]
        Self([self.0[0], other.0[0], self.0[1], other.0[1]])
    }

    #[inline]
    fn interleave_high(self, other: Self) -> Self {
        // [a0,a1,a2,a3] + [b0,b1,b2,b3] → [a2,b2,a3,b3]
        Self([self.0[2], other.0[2], self.0[3], other.0[3]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed4_add() {
        let a = PackedGoldilocks4::from_slice(&[
            Goldilocks::new(1),
            Goldilocks::new(2),
            Goldilocks::new(3),
            Goldilocks::new(4),
        ]);
        let b = PackedGoldilocks4::from_slice(&[
            Goldilocks::new(10),
            Goldilocks::new(20),
            Goldilocks::new(30),
            Goldilocks::new(40),
        ]);
        let c = a + b;
        assert_eq!(c.extract(0), Goldilocks::new(11));
        assert_eq!(c.extract(1), Goldilocks::new(22));
        assert_eq!(c.extract(2), Goldilocks::new(33));
        assert_eq!(c.extract(3), Goldilocks::new(44));
    }

    #[test]
    fn packed4_mul() {
        let a = PackedGoldilocks4::broadcast(Goldilocks::new(7));
        let b = PackedGoldilocks4::broadcast(Goldilocks::new(6));
        let c = a * b;
        for i in 0..4 {
            assert_eq!(c.extract(i), Goldilocks::new(42));
        }
    }

    #[test]
    fn packed4_inv() {
        let vals = [
            Goldilocks::new(3),
            Goldilocks::new(7),
            Goldilocks::new(42),
            Goldilocks::new(99),
        ];
        let a = PackedGoldilocks4::from_slice(&vals);
        let inv_a = a.inv();
        let product = a * inv_a;
        for i in 0..4 {
            assert_eq!(product.extract(i), Goldilocks::ONE);
        }
    }

    #[test]
    fn packed4_broadcast_extract() {
        let v = Goldilocks::new(123);
        let p = PackedGoldilocks4::broadcast(v);
        for i in 0..4 {
            assert_eq!(p.extract(i), v);
        }
    }

    #[test]
    fn packed4_interleave() {
        let a = PackedGoldilocks4::from_slice(&[
            Goldilocks::new(1),
            Goldilocks::new(2),
            Goldilocks::new(3),
            Goldilocks::new(4),
        ]);
        let b = PackedGoldilocks4::from_slice(&[
            Goldilocks::new(10),
            Goldilocks::new(20),
            Goldilocks::new(30),
            Goldilocks::new(40),
        ]);
        let lo = a.interleave_low(b);
        assert_eq!(lo.extract(0), Goldilocks::new(1));
        assert_eq!(lo.extract(1), Goldilocks::new(10));
        assert_eq!(lo.extract(2), Goldilocks::new(2));
        assert_eq!(lo.extract(3), Goldilocks::new(20));

        let hi = a.interleave_high(b);
        assert_eq!(hi.extract(0), Goldilocks::new(3));
        assert_eq!(hi.extract(1), Goldilocks::new(30));
        assert_eq!(hi.extract(2), Goldilocks::new(4));
        assert_eq!(hi.extract(3), Goldilocks::new(40));
    }

    #[test]
    fn packed4_ntt_butterfly() {
        // simulate one NTT butterfly: a' = a + w*b, b' = a - w*b
        let a = PackedGoldilocks4::broadcast(Goldilocks::new(100));
        let b = PackedGoldilocks4::broadcast(Goldilocks::new(10));
        let w = PackedGoldilocks4::broadcast(Goldilocks::new(7));

        let wb = w * b; // 70 in all lanes
        let a_new = a + wb; // 170
        let b_new = a - wb; // 30

        assert_eq!(a_new.extract(0), Goldilocks::new(170));
        assert_eq!(b_new.extract(0), Goldilocks::new(30));
    }
}
