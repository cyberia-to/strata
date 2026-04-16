#![no_std]
//! cyb-algebra-proof — tier 2: proof system traits.
//!
//! traits needed by lens (polynomial commitment) and zheng (constraint verification).
//! consumers that only do field arithmetic (hemera) don't need this tier.
//!
//! ## Hash2Field
//!
//! derive a field element from hash output bytes. this is the bridge between
//! hemera (which produces bytes) and field operations (which need elements).
//! used for Fiat-Shamir challenges in lens and zheng.
//!
//! ## Dot
//!
//! compute the inner product Σ aᵢ·bᵢ of two field element vectors.
//! zheng uses this for CCS constraint evaluation (matrix-vector products).
//! lens uses it for multilinear polynomial evaluation.
//! algebras can override the default loop with hardware FMA, delayed
//! modular reduction, or vectorized operations.

use cyb_algebra::Field;

/// derive a field element from arbitrary bytes.
///
/// the bytes typically come from a hash function (hemera). the reduction
/// maps bytes to a field element deterministically. the distribution
/// should be close to uniform over the field.
///
/// used by lens Transcript::squeeze_field and zheng Fiat-Shamir.
pub trait Hash2Field: Field {
    /// reduce hash output bytes to a field element.
    /// the input length depends on the field:
    /// - Goldilocks: ≥ 8 bytes (take low 8 bytes, reduce mod p)
    /// - F₂¹²⁸: ≥ 16 bytes (take low 16 bytes, interpret as u128)
    /// - F_q: ≥ 64 bytes (take 64 bytes, reduce mod q)
    fn from_hash(bytes: &[u8]) -> Self;
}

/// inner product of two field element vectors: Σ aᵢ·bᵢ.
///
/// the fundamental operation for constraint evaluation and polynomial
/// evaluation. given vectors a = [a₀, a₁, ...] and b = [b₀, b₁, ...],
/// computes a₀·b₀ + a₁·b₁ + ... + aₙ·bₙ.
///
/// default implementation is a simple loop. algebras can override with:
/// - hardware FMA (fused multiply-add, avoids intermediate rounding)
/// - delayed modular reduction (accumulate in wider integer, reduce once)
/// - SIMD vectorization (process 4-8 products in parallel)
///
/// consumers:
/// - zheng: Σ constraint_coeff[i] · witness[i] (CCS evaluation)
/// - lens: Σ eval[i] · basis[i] (multilinear extension evaluation)
/// - nox: Σ weight[i] · value[i] (linear combination jets)
pub trait Dot: Field {
    /// compute a[0]*b[0] + a[1]*b[1] + ... + a[n-1]*b[n-1].
    /// panics if slices differ in length.
    fn dot(a: &[Self], b: &[Self]) -> Self {
        assert_eq!(a.len(), b.len());
        let mut acc = Self::ZERO;
        for (&ai, &bi) in a.iter().zip(b.iter()) {
            acc = acc + ai * bi;
        }
        acc
    }
}
