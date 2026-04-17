//! Semiring, Ring, Field, Codec implementations for Fp2, Fp3, Fp4.

use super::{Fp2, Fp3, Fp4};
use crate::field::Goldilocks;
use core::ops::{AddAssign, MulAssign, SubAssign};
use strata_core::{Codec, Field, Ring, Semiring};

// ── AddAssign / SubAssign / MulAssign ────────────────────────────

macro_rules! impl_assign_ops {
    ($ty:ty) => {
        impl AddAssign for $ty {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl SubAssign for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl MulAssign for $ty {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    };
}

impl_assign_ops!(Fp2);
impl_assign_ops!(Fp3);
impl_assign_ops!(Fp4);

// ── Semiring ─────────────────────────────────────────────────────

impl Semiring for Fp2 {
    const ZERO: Self = Fp2::ZERO;
    const ONE: Self = Fp2::ONE;
}

impl Semiring for Fp3 {
    const ZERO: Self = Fp3::ZERO;
    const ONE: Self = Fp3::ONE;
}

impl Semiring for Fp4 {
    const ZERO: Self = Fp4::ZERO;
    const ONE: Self = Fp4::ONE;
}

// ── Ring ─────────────────────────────────────────────────────────

impl Ring for Fp2 {}
impl Ring for Fp3 {}
impl Ring for Fp4 {}

// ── Field ────────────────────────────────────────────────────────

impl Field for Fp2 {
    fn inv(self) -> Self {
        Fp2::inv(self)
    }
    fn sqrt(self) -> Option<Self> {
        None // TODO: Fp2 sqrt via Cipolla or norm-based algorithm
    }
}

impl Field for Fp3 {
    fn inv(self) -> Self {
        Fp3::inv(self)
    }
    fn sqrt(self) -> Option<Self> {
        None // TODO: Fp3 sqrt
    }
}

impl Field for Fp4 {
    fn inv(self) -> Self {
        Fp4::inv(self)
    }
    fn sqrt(self) -> Option<Self> {
        None // TODO: Fp4 sqrt
    }
}

// ── Codec ────────────────────────────────────────────────────────

impl Codec for Fp2 {
    fn byte_len() -> usize {
        16
    }
    fn encode(&self, buf: &mut [u8]) {
        <Goldilocks as Codec>::encode(&self.re, &mut buf[..8]);
        <Goldilocks as Codec>::encode(&self.im, &mut buf[8..16]);
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 16 {
            return None;
        }
        let re = <Goldilocks as Codec>::decode(&bytes[..8])?;
        let im = <Goldilocks as Codec>::decode(&bytes[8..16])?;
        Some(Fp2 { re, im })
    }
}

impl Codec for Fp3 {
    fn byte_len() -> usize {
        24
    }
    fn encode(&self, buf: &mut [u8]) {
        <Goldilocks as Codec>::encode(&self.c0, &mut buf[..8]);
        <Goldilocks as Codec>::encode(&self.c1, &mut buf[8..16]);
        <Goldilocks as Codec>::encode(&self.c2, &mut buf[16..24]);
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 24 {
            return None;
        }
        let c0 = <Goldilocks as Codec>::decode(&bytes[..8])?;
        let c1 = <Goldilocks as Codec>::decode(&bytes[8..16])?;
        let c2 = <Goldilocks as Codec>::decode(&bytes[16..24])?;
        Some(Fp3 { c0, c1, c2 })
    }
}

impl Codec for Fp4 {
    fn byte_len() -> usize {
        32
    }
    fn encode(&self, buf: &mut [u8]) {
        <Goldilocks as Codec>::encode(&self.c0, &mut buf[..8]);
        <Goldilocks as Codec>::encode(&self.c1, &mut buf[8..16]);
        <Goldilocks as Codec>::encode(&self.c2, &mut buf[16..24]);
        <Goldilocks as Codec>::encode(&self.c3, &mut buf[24..32]);
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 32 {
            return None;
        }
        let c0 = <Goldilocks as Codec>::decode(&bytes[..8])?;
        let c1 = <Goldilocks as Codec>::decode(&bytes[8..16])?;
        let c2 = <Goldilocks as Codec>::decode(&bytes[16..24])?;
        let c3 = <Goldilocks as Codec>::decode(&bytes[24..32])?;
        Some(Fp4 { c0, c1, c2, c3 })
    }
}

// ── property tests ───────────────────────────────────────────────

strata_core::test_field_axioms!(Fp2, fp2_axioms, |v: u64| Fp2 {
    re: Goldilocks::new(v).canonicalize(),
    im: Goldilocks::new(v.wrapping_mul(7)).canonicalize(),
});

strata_core::test_field_axioms!(Fp3, fp3_axioms, |v: u64| Fp3 {
    c0: Goldilocks::new(v).canonicalize(),
    c1: Goldilocks::new(v.wrapping_mul(3)).canonicalize(),
    c2: Goldilocks::new(v.wrapping_mul(11)).canonicalize(),
});

strata_core::test_field_axioms!(Fp4, fp4_axioms, |v: u64| Fp4 {
    c0: Goldilocks::new(v).canonicalize(),
    c1: Goldilocks::new(v.wrapping_mul(5)).canonicalize(),
    c2: Goldilocks::new(v.wrapping_mul(13)).canonicalize(),
    c3: Goldilocks::new(v.wrapping_mul(17)).canonicalize(),
});
