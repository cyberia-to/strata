// ---
// tags: jali, rust
// crystal-type: source
// crystal-domain: comp
// ---
//! NTT domain transforms for negacyclic convolution.
//!
//! For R_q = F_p[x]/(x^n+1) we need the negacyclic NTT:
//! - Standard NTT works modulo x^n - 1
//! - Negacyclic NTT works modulo x^n + 1
//!
//! Implementation:
//! 1. Pre-multiply: coeffs[i] *= psi^i where psi is a primitive 2n-th root of unity
//! 2. Apply nebu's forward NTT (with precomputed twiddles when available)
//! 3. For inverse: apply nebu's INTT, then post-multiply by psi^(-i)
//!
//! When the `fast` feature is enabled (acpu), the twist step uses
//! `gl_mul_batch` for hardware-accelerated batch multiplication.

use nebu::Goldilocks;
use nebu::field::P;

use crate::ring::RingElement;

/// Primitive root of F_p* (same as nebu's internal G=7).
const G: Goldilocks = Goldilocks::new(7);

/// Compute a primitive 2n-th root of unity.
#[inline]
fn psi_for(n: usize) -> Goldilocks {
    let two_n = (2 * n) as u64;
    G.exp((P - 1) / two_n)
}

/// Precompute twist factors psi^0, psi^1, ..., psi^(n-1).
fn precompute_twist(n: usize) -> alloc::vec::Vec<Goldilocks> {
    let psi = psi_for(n);
    let mut table = alloc::vec::Vec::with_capacity(n);
    let mut psi_pow = Goldilocks::ONE;
    for _ in 0..n {
        table.push(psi_pow);
        psi_pow *= psi;
    }
    table
}

/// Apply twist: coeffs[i] *= twist_table[i].
#[cfg(not(feature = "fast"))]
fn apply_twist(coeffs: &mut [Goldilocks], twist: &[Goldilocks], n: usize) {
    for i in 0..n {
        coeffs[i] *= twist[i];
    }
}

/// Apply twist using acpu batch multiply (hardware-accelerated).
#[cfg(feature = "fast")]
fn apply_twist(coeffs: &mut [Goldilocks], twist: &[Goldilocks], n: usize) {
    // Safety: Goldilocks is repr(transparent) over u64
    let coeffs_u64 = unsafe { core::slice::from_raw_parts_mut(coeffs.as_mut_ptr() as *mut u64, n) };
    let twist_u64 = unsafe { core::slice::from_raw_parts(twist.as_ptr() as *const u64, n) };
    let mut dst = alloc::vec![0u64; n];
    acpu::field::goldilocks::gl_mul_batch(coeffs_u64, twist_u64, &mut dst);
    coeffs_u64.copy_from_slice(&dst);
}

/// Forward negacyclic NTT on raw coefficient slice.
pub fn negacyclic_forward(coeffs: &mut [Goldilocks], n: usize) {
    assert!(n.is_power_of_two());
    assert!(coeffs.len() >= n);

    let twist = precompute_twist(n);
    apply_twist(coeffs, &twist, n);

    // Use precomputed twiddle NTT for speed
    let twiddles = nebu::ntt::precompute_twiddles_vec(n);
    nebu::ntt::ntt_with_twiddles(&mut coeffs[..n], &twiddles);
}

/// Inverse negacyclic NTT on raw coefficient slice.
pub fn negacyclic_inverse(coeffs: &mut [Goldilocks], n: usize) {
    assert!(n.is_power_of_two());
    assert!(coeffs.len() >= n);

    nebu::ntt::intt(&mut coeffs[..n]);

    // Untwist: multiply by psi^(-i)
    let psi = psi_for(n);
    let psi_inv = psi.inv();
    let mut untwist = alloc::vec::Vec::with_capacity(n);
    let mut psi_inv_pow = Goldilocks::ONE;
    for _ in 0..n {
        untwist.push(psi_inv_pow);
        psi_inv_pow *= psi_inv;
    }
    apply_twist(coeffs, &untwist, n);
}

/// Convert a RingElement to NTT form (in-place).
pub fn to_ntt(elem: &mut RingElement) {
    assert!(!elem.is_ntt, "already in NTT form");
    let n = elem.n;
    negacyclic_forward(&mut elem.coeffs, n);
    elem.is_ntt = true;
}

/// Convert a RingElement from NTT form back to coefficient form (in-place).
pub fn from_ntt(elem: &mut RingElement) {
    assert!(elem.is_ntt, "not in NTT form");
    let n = elem.n;
    negacyclic_inverse(&mut elem.coeffs, n);
    elem.is_ntt = false;
}

extern crate alloc;
