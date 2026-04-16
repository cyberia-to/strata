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
//! ## Fma
//!
//! fused multiply-accumulate: compute Σ aᵢ·bᵢ efficiently.
//! zheng uses this for CCS constraint evaluation (matrix-vector products
//! over field elements). a naive loop does N multiplies + N additions.
//! fma can exploit CPU/GPU fused operations or delayed reduction.

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

/// fused multiply-accumulate: Σ aᵢ·bᵢ.
///
/// default implementation is a simple loop. algebras can override with
/// hardware fma, delayed reduction, or vectorized operations.
///
/// zheng constraint evaluation: Σ constraint_coeff[i] · witness[i].
/// lens multilinear evaluation: Σ eval[i] · basis[i].
pub trait Fma: Field {
    /// compute a[0]*b[0] + a[1]*b[1] + ... + a[n-1]*b[n-1].
    /// slices must be the same length.
    fn sum_of_products(a: &[Self], b: &[Self]) -> Self {
        assert_eq!(a.len(), b.len());
        let mut acc = Self::ZERO;
        for (&ai, &bi) in a.iter().zip(b.iter()) {
            acc = acc + ai * bi;
        }
        acc
    }
}
