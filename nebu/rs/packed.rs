//! Packed Goldilocks — scalar fallback (WIDTH=1).
//!
//! This is the portable baseline. SIMD implementations (WIDTH=4 via AVX2,
//! WIDTH=8 via AVX-512) are provided by honeycrisp/acpu when available.
//!
//! The scalar fallback still participates in generic Packed code — it just
//! processes one element at a time instead of N. This lets all algorithms
//! be written once against the Packed trait.

use crate::field::Goldilocks;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use strata_compute::Packed;
use strata_core::{Field, Ring, Semiring};

/// Scalar-width packed Goldilocks (WIDTH=1).
///
/// wraps a single Goldilocks element. implements Packed so that
/// generic code compiles on all platforms. SIMD backends provide
/// wider implementations via honeycrisp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PackedGoldilocks(pub Goldilocks);

impl Add for PackedGoldilocks {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for PackedGoldilocks {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for PackedGoldilocks {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for PackedGoldilocks {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Mul for PackedGoldilocks {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign for PackedGoldilocks {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl Neg for PackedGoldilocks {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Semiring for PackedGoldilocks {
    const ZERO: Self = Self(Goldilocks::ZERO);
    const ONE: Self = Self(Goldilocks::ONE);
}

impl Ring for PackedGoldilocks {}

impl Field for PackedGoldilocks {
    #[inline]
    fn inv(self) -> Self {
        Self(self.0.inv())
    }
    #[inline]
    fn square(self) -> Self {
        Self(self.0.square())
    }
    fn sqrt(self) -> Option<Self> {
        crate::sqrt::sqrt(self.0).map(Self)
    }
}

impl Packed for PackedGoldilocks {
    type Scalar = Goldilocks;
    const WIDTH: usize = 1;

    #[inline]
    fn from_slice(slice: &[Goldilocks]) -> Self {
        assert_eq!(slice.len(), 1);
        Self(slice[0])
    }

    #[inline]
    fn to_slice(&self, slice: &mut [Goldilocks]) {
        assert_eq!(slice.len(), 1);
        slice[0] = self.0;
    }

    #[inline]
    fn broadcast(val: Goldilocks) -> Self {
        Self(val)
    }

    #[inline]
    fn extract(&self, index: usize) -> Goldilocks {
        assert_eq!(index, 0);
        self.0
    }

    #[inline]
    fn interleave_low(self, _other: Self) -> Self {
        self // WIDTH=1, interleave is identity
    }

    #[inline]
    fn interleave_high(self, other: Self) -> Self {
        other // WIDTH=1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_broadcast_extract() {
        let v = Goldilocks::new(42);
        let p = PackedGoldilocks::broadcast(v);
        assert_eq!(p.extract(0), v);
    }

    #[test]
    fn packed_add() {
        let a = PackedGoldilocks(Goldilocks::new(3));
        let b = PackedGoldilocks(Goldilocks::new(7));
        assert_eq!((a + b).0, Goldilocks::new(10));
    }

    #[test]
    fn packed_mul() {
        let a = PackedGoldilocks(Goldilocks::new(5));
        let b = PackedGoldilocks(Goldilocks::new(6));
        assert_eq!((a * b).0, Goldilocks::new(30));
    }

    #[test]
    fn packed_inv() {
        let a = PackedGoldilocks(Goldilocks::new(42));
        assert_eq!((a * a.inv()).0, Goldilocks::ONE);
    }

    #[test]
    fn packed_from_to_slice() {
        let v = Goldilocks::new(99);
        let p = PackedGoldilocks::from_slice(&[v]);
        let mut out = [Goldilocks::ZERO];
        p.to_slice(&mut out);
        assert_eq!(out[0], v);
    }
}
