#![no_std]
//! cyb-algebra — algebraic trait hierarchy for verifiable computation.
//!
//! Three levels: Semiring → Ring → Field.
//! Five implementations: Goldilocks (nebu), F₂¹²⁸ (kuro), R_q (jali),
//! Tropical (trop), F_q (genies).

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Semiring: two operations (add, mul) with identities (zero, one).
/// Tropical (min, +) satisfies this — no subtraction required.
pub trait Semiring: Copy + Eq + Add<Output = Self> + Mul<Output = Self> + AddAssign + MulAssign {
    const ZERO: Self;
    const ONE: Self;
}

/// Ring: semiring with subtraction and negation.
/// R_q polynomial ring (jali) satisfies this — no general inverse.
pub trait Ring:
    Semiring + Sub<Output = Self> + Neg<Output = Self> + SubAssign
{
}

/// Field: ring with multiplicative inverse.
/// Goldilocks (nebu), F₂¹²⁸ (kuro), F_q (genies) satisfy this.
pub trait Field: Ring {
    fn inv(self) -> Self;
    /// Derive a field element from hash output bytes.
    /// Pure reduction — no crypto dependency.
    fn from_hash(bytes: &[u8]) -> Self;
}
