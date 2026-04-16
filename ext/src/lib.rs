#![no_std]
//! strata-ext — tier 4: structural traits.
//!
//! traits for specific algebraic structures that not every field needs.
//!
//! ## Extension
//!
//! tower fields: F_p → Fp2 → Fp3 → Fp4 (nebu extensions), or
//! F₂ → F₂² → F₂⁴ → ... → F₂¹²⁸ (kuro tower). an extension field
//! has a base field, a degree, and a Frobenius endomorphism.
//!
//! ## Batch
//!
//! Montgomery's trick: invert N elements with 1 inversion + 3(N-1)
//! multiplications. nebu and kuro both implement this independently —
//! this trait unifies the interface.
//!
//! ## Blind
//!
//! timing-safe operations for privacy-critical code. genies (CSIDH)
//! requires constant-time arithmetic — no branches on secret data.
//! other algebras may implement this for defense in depth.

use strata_core::Field;

/// an extension field over a base field.
///
/// E is an extension of B if B ⊂ E and E is a vector space over B.
/// the extension degree [E:B] = dim_B(E) — how many base elements
/// represent one extension element.
///
/// examples:
/// - Fp2 over Goldilocks: degree 2, Frobenius is conjugation
/// - F₂¹²⁸ over F₂⁶⁴: degree 2, defined by x² + x + α
/// - F₂¹²⁸ over F₂: degree 128, Frobenius is squaring
pub trait Extension<Base: Field>: Field {
    /// extension degree: [Self : Base].
    const DEGREE: usize;

    /// decompose into base field coefficients.
    /// returns DEGREE elements.
    fn to_base_elements(&self) -> alloc::vec::Vec<Base>;

    /// construct from base field coefficients.
    fn from_base_elements(coeffs: &[Base]) -> Self;

    /// embed a base element into the extension (as degree-0 coefficient).
    fn from_base(b: Base) -> Self;

    /// Frobenius endomorphism: x ↦ x^(|Base|^power).
    /// for prime fields of char p: Frobenius(x) = x^p.
    /// for binary fields: Frobenius(x) = x² (squaring).
    fn frobenius_map(&self, power: usize) -> Self;
}

/// batch operations via Montgomery's trick.
///
/// invert N elements using 1 inversion + 3(N-1) multiplications.
/// prefix products → invert the product → back-propagate individual inverses.
pub trait Batch: Field {
    /// batch-invert a slice of field elements in-place.
    /// zero elements are left as zero.
    fn batch_inv(elements: &mut [Self]);
}

/// constant-time operations for privacy-critical arithmetic.
///
/// genies (CSIDH) requires this: isogeny walks on secret exponents
/// must not leak timing information. no branches on secret data,
/// no variable-time memory access, no early exits.
pub trait Blind: Field {
    /// constant-time equality comparison.
    fn ct_eq(&self, other: &Self) -> bool;

    /// constant-time conditional select: if choice is true, return a; else b.
    /// executes both branches regardless of choice.
    fn ct_select(a: &Self, b: &Self, choice: bool) -> Self;

    /// constant-time conditional swap: if choice is true, swap a and b.
    fn ct_swap(a: &mut Self, b: &mut Self, choice: bool);
}

extern crate alloc;
