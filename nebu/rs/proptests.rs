//! Property-based tests for Goldilocks using proptest.
//!
//! Generates random field elements and verifies algebraic axioms
//! over thousands of cases. This catches edge cases that deterministic
//! test vectors miss.

#[cfg(test)]
mod tests {
    use crate::field::{Goldilocks, P};
    use proptest::prelude::*;
    use strata_core::{Codec, Field, Semiring};
    use strata_proof::Reduce;

    /// strategy: random canonical Goldilocks element in [0, p)
    fn goldilocks() -> impl Strategy<Value = Goldilocks> {
        (0..P).prop_map(Goldilocks::new)
    }

    /// strategy: random nonzero Goldilocks element
    fn nonzero() -> impl Strategy<Value = Goldilocks> {
        (1..P).prop_map(Goldilocks::new)
    }

    proptest! {
        // ── additive axioms ──────────────────────────────────────

        #[test]
        fn add_commutative(a in goldilocks(), b in goldilocks()) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn add_associative(a in goldilocks(), b in goldilocks(), c in goldilocks()) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn add_identity(a in goldilocks()) {
            prop_assert_eq!(a + Goldilocks::ZERO, a);
        }

        #[test]
        fn add_inverse(a in goldilocks()) {
            prop_assert_eq!(a + (-a), Goldilocks::ZERO);
        }

        // ── multiplicative axioms ────────────────────────────────

        #[test]
        fn mul_commutative(a in goldilocks(), b in goldilocks()) {
            prop_assert_eq!(a * b, b * a);
        }

        #[test]
        fn mul_associative(a in goldilocks(), b in goldilocks(), c in goldilocks()) {
            prop_assert_eq!((a * b) * c, a * (b * c));
        }

        #[test]
        fn mul_identity(a in goldilocks()) {
            prop_assert_eq!(a * Goldilocks::ONE, a);
        }

        #[test]
        fn mul_inverse(a in nonzero()) {
            prop_assert_eq!(a * a.inv(), Goldilocks::ONE);
        }

        // ── distributivity ───────────────────────────────────────

        #[test]
        fn distributive(a in goldilocks(), b in goldilocks(), c in goldilocks()) {
            prop_assert_eq!(a * (b + c), a * b + a * c);
        }

        // ── Field methods ────────────────────────────────────────

        #[test]
        fn square_is_mul(a in goldilocks()) {
            prop_assert_eq!(a.square(), a * a);
        }

        #[test]
        fn double_is_add(a in goldilocks()) {
            prop_assert_eq!(a.double(), a + a);
        }

        #[test]
        fn try_inv_nonzero(a in nonzero()) {
            prop_assert_eq!(a.try_inv(), Some(a.inv()));
        }

        #[test]
        fn sqrt_of_square(a in goldilocks()) {
            let sq = a.square();
            if let Some(root) = sq.sqrt() {
                prop_assert_eq!(root.square(), sq);
            }
        }

        #[test]
        fn pow_matches_mul(a in goldilocks(), e in 0u64..100) {
            let mut expected = Goldilocks::ONE;
            for _ in 0..e {
                expected = expected * a;
            }
            prop_assert_eq!(a.pow(e), expected);
        }

        // ── Codec roundtrip ──────────────────────────────────────

        #[test]
        fn codec_roundtrip(a in goldilocks()) {
            let mut buf = [0u8; 8];
            a.encode(&mut buf);
            let decoded = Goldilocks::decode(&buf).unwrap();
            prop_assert_eq!(decoded, a);
        }

        #[test]
        fn codec_rejects_noncanonical(extra in 0u64..1000) {
            let val = P + extra; // >= p, non-canonical
            let buf = val.to_le_bytes();
            prop_assert!(Goldilocks::decode(&buf).is_none());
        }

        // ── Reduce ───────────────────────────────────────────────

        #[test]
        fn reduce_deterministic(bytes in prop::collection::vec(any::<u8>(), 8..64)) {
            let a = Goldilocks::reduce(&bytes);
            let b = Goldilocks::reduce(&bytes);
            prop_assert_eq!(a, b);
        }

        #[test]
        fn reduce_is_canonical(bytes in prop::collection::vec(any::<u8>(), 8..64)) {
            let a = Goldilocks::reduce(&bytes);
            prop_assert!(a.as_u64() < P);
        }
    }
}
