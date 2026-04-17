#![no_std]
//! strata-core — algebraic trait hierarchy for verifiable computation.
//!
//! four tiers of traits organized by consumer need:
//!
//! ## tier 1: universal (this crate)
//!
//! every algebra implements at least one of these.
//!
//! ```text
//! Codec      encode, decode — serialization
//! Semiring   add, mul, zero, one — tropical lives here
//! Ring       + sub, neg — polynomial rings live here
//! Field      + inv, sqrt, double, pow — finite fields live here
//! ```
//!
//! ## tier 2: proof system (`strata-proof`)
//!
//! traits needed by lens (commitment) and zheng (verification).
//!
//! ```text
//! Reduce   reduce(bytes) → element — Fiat-Shamir challenges
//! Dot      dot(a, b) → scalar — inner product for constraint evaluation
//! ```
//!
//! ## tier 3: computation (`strata-compute`)
//!
//! traits needed by nox (execution) and jali (ring arithmetic).
//!
//! ```text
//! Spectral   roots_of_unity, two_adicity — fields with NTT/transform domain
//! Bits       to_bits, from_bits — bit decomposition for binary operations
//! ```
//!
//! ## tier 4: structure (`strata-ext`)
//!
//! traits for specific algebraic structures.
//!
//! ```text
//! Extension<Base>   base field, degree, frobenius — tower fields
//! Batch             batch_inv — Montgomery's trick
//! Blind             ct_eq, ct_select — timing-safe operations
//! ```
//!
//! ## the five algebras
//!
//! | type | crate | tiers |
//! |------|-------|-------|
//! | Goldilocks | nebu | Field + Reduce + Dot + Spectral + Bits + Extension + Batch |
//! | F₂¹²⁸ | kuro | Field + Reduce + Dot + Bits + Extension + Batch |
//! | RingElement | jali | (uses Goldilocks for scalar ops) |
//! | Tropical | trop | Semiring + Codec |
//! | Fq | genies | Field + Reduce + Dot + Batch + Blind |

pub mod testing;

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ── tier 1: universal ────────────────────────────────────────────

/// serialize algebraic elements to and from bytes.
///
/// `encode` produces canonical bytes. `decode` rejects non-canonical input
/// (returns None) — it NEVER silently reduces. this is critical for soundness:
/// a verifier accepting non-canonical elements can break proof security.
pub trait Codec: Sized {
    /// byte length of the canonical serialized form.
    fn byte_len() -> usize;

    /// serialize to canonical little-endian bytes.
    /// buffer must be at least `byte_len()` bytes.
    fn encode(&self, buf: &mut [u8]);

    /// deserialize from bytes.
    /// returns None if bytes are too short OR the value is non-canonical
    /// (e.g., >= modulus for prime fields).
    fn decode(bytes: &[u8]) -> Option<Self>;
}

/// semiring: two operations with identities. no subtraction.
///
/// addition and multiplication are associative and commutative.
/// multiplication distributes over addition. zero annihilates (a * 0 = 0).
///
/// the tropical semiring (min, +) satisfies this — min has no inverse.
pub trait Semiring:
    Copy + Eq + Add<Output = Self> + Mul<Output = Self> + AddAssign + MulAssign
{
    const ZERO: Self;
    const ONE: Self;
}

/// ring: semiring with subtraction and negation.
///
/// the additive structure is a group (every element has an additive inverse).
/// polynomial ring R_q = F_p[x]/(x^n+1) satisfies this.
pub trait Ring: Semiring + Sub<Output = Self> + Neg<Output = Self> + SubAssign {}

/// field: ring with multiplicative inverse and related operations.
///
/// every nonzero element has a unique multiplicative inverse.
/// Goldilocks (nebu), F₂¹²⁸ (kuro), F_q (genies) satisfy this.
pub trait Field: Ring {
    /// multiplicative inverse.
    ///
    /// # Panics
    /// panics if self is zero. use `try_inv` for non-panicking version.
    fn inv(self) -> Self;

    /// multiplicative inverse, returning None for zero.
    ///
    /// production code should prefer this over `inv` — panicking on zero
    /// in a verifier is a denial-of-service vector.
    fn try_inv(self) -> Option<Self> {
        if self == Self::ZERO {
            None
        } else {
            Some(self.inv())
        }
    }

    /// a² — often faster than a * a (single squaring vs general multiply).
    fn square(self) -> Self {
        self * self
    }

    /// 2a — via addition, avoids multiply overhead.
    fn double(self) -> Self {
        self + self
    }

    /// square root. returns None if self is a quadratic non-residue.
    ///
    /// for prime fields: Tonelli-Shanks or special-case (3 mod 4, 5 mod 8).
    /// for binary fields: inverse Frobenius (a^(2^(n-1))).
    fn sqrt(self) -> Option<Self>;

    /// a^e via square-and-multiply. e is a single u64.
    fn pow(self, mut e: u64) -> Self {
        let mut base = self;
        let mut result = Self::ONE;
        while e > 0 {
            if e & 1 == 1 {
                result *= base;
            }
            base = base.square();
            e >>= 1;
        }
        result
    }

    /// a^e where e is a multi-limb exponent (little-endian u64 limbs).
    ///
    /// required for genies (512-bit exponents) and any field where
    /// the group order exceeds u64.
    fn pow_bytes(self, exp: &[u64]) -> Self {
        let mut result = Self::ONE;
        for &limb in exp.iter().rev() {
            for bit in (0..64).rev() {
                result = result.square();
                if (limb >> bit) & 1 == 1 {
                    result *= self;
                }
            }
        }
        result
    }

    /// a² in-place — avoids allocation for large elements.
    fn square_in_place(&mut self) {
        *self = self.square();
    }

    /// double in-place.
    fn double_in_place(&mut self) {
        *self = self.double();
    }
}
