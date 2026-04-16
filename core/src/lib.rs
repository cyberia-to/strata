#![no_std]
//! cyb-algebra — algebraic trait hierarchy for verifiable computation.
//!
//! four tiers of traits organized by consumer need:
//!
//! ## tier 1: universal (this crate)
//!
//! every algebra implements at least one of these.
//!
//! ```text
//! Encode     encode, decode — serialization
//! Semiring   add, mul, zero, one — tropical lives here
//! Ring       + sub, neg — polynomial rings live here
//! Field      + inv — finite fields live here
//! ```
//!
//! ## tier 2: proof system (`cyb-algebra-proof`)
//!
//! traits needed by lens (commitment) and zheng (verification).
//!
//! ```text
//! Reduce   reduce(bytes) → element — Fiat-Shamir challenges
//! Dot          dot — fused multiply-accumulate for constraint evaluation
//! ```
//!
//! ## tier 3: computation (`cyb-algebra-compute`)
//!
//! traits needed by nox (execution) and jali (ring arithmetic).
//!
//! ```text
//! Spectral   roots_of_unity, two_adicity — fields with NTT/transform domain
//! Bits       to_bits, from_bits — bit decomposition for binary operations
//! ```
//!
//! ## tier 4: structure (`cyb-algebra-ext`)
//!
//! traits for specific algebraic structures.
//!
//! ```text
//! Extension<Base>   base field, degree, frobenius — tower fields
//! Batch             batch_inv — Montgomery's trick
//! Blind      ct_eq, ct_select — timing-safe operations
//! ```
//!
//! ## the five algebras
//!
//! | type | crate | tiers |
//! |------|-------|-------|
//! | Goldilocks | nebu | Field + Reduce + Dot + Spectral + Bits + Extension + Batch |
//! | F₂¹²⁸ | kuro | Field + Reduce + Bits + Extension + Batch |
//! | RingElement | jali | (uses Goldilocks for scalar ops) |
//! | Tropical | trop | Semiring + Encode |
//! | Fq | genies | Field + Reduce + Batch + Blind |

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ── tier 1: universal ────────────────────────────────────────────

/// serialize algebraic elements to and from bytes.
/// every type in the algebra stack implements this.
pub trait Encode: Sized {
    /// expected byte length of the serialized form.
    fn byte_len() -> usize;
    /// serialize to a byte buffer. buffer must be at least `byte_len()` bytes.
    fn encode(&self, buf: &mut [u8]);
    /// deserialize from bytes. returns None if bytes are invalid.
    fn decode(bytes: &[u8]) -> Option<Self>;
}

/// semiring: two operations with identities. no subtraction.
///
/// addition and multiplication are associative and commutative.
/// multiplication distributes over addition. zero annihilates under
/// multiplication (a * 0 = 0).
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

/// field: ring with multiplicative inverse.
///
/// every nonzero element has a unique multiplicative inverse.
/// Goldilocks (nebu), F₂¹²⁸ (kuro), F_q (genies) satisfy this.
pub trait Field: Ring {
    /// multiplicative inverse. panics on zero.
    fn inv(self) -> Self;
    /// a² — often faster than a * a.
    fn square(self) -> Self {
        self * self
    }
    /// a^e via square-and-multiply.
    fn pow(self, mut e: u64) -> Self {
        let mut base = self;
        let mut result = Self::ONE;
        while e > 0 {
            if e & 1 == 1 {
                result = result * base;
            }
            base = base.square();
            e >>= 1;
        }
        result
    }
}
