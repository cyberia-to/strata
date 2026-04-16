//! Semiring implementation for Tropical.
//! Tropical addition = min, tropical multiplication = saturating add.

use crate::element::Tropical;
use core::ops::{Add, AddAssign, Mul, MulAssign};
use cyb_algebra::Semiring;

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

impl Semiring for Tropical {
    const ZERO: Self = Tropical::ZERO; // +inf (additive identity for min)
    const ONE: Self = Tropical::ONE; // 0 (multiplicative identity for +)
}
