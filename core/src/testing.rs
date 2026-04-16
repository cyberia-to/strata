//! Property-based testing macros for algebraic axioms.
//!
//! `test_semiring_axioms!` — commutativity, associativity, identity, distributivity
//! `test_ring_axioms!` — semiring + additive inverse
//! `test_field_axioms!` — ring + multiplicative inverse, sqrt, pow
//!
//! Each macro generates a test module with deterministic pseudo-random elements.

/// generate N pseudo-random u64 values from a seed.
#[doc(hidden)]
pub fn test_rng(seed: u64, n: usize) -> alloc::vec::Vec<u64> {
    let mut state = if seed == 0 {
        0xDEAD_BEEF_CAFE_BABE
    } else {
        seed
    };
    let mut out = alloc::vec::Vec::with_capacity(n);
    for _ in 0..n {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        out.push(state);
    }
    out
}

extern crate alloc;

/// test semiring axioms.
#[macro_export]
macro_rules! test_semiring_axioms {
    ($ty:ty, $mod_name:ident, $from_u64:expr) => {
        #[cfg(test)]
        mod $mod_name {
            extern crate alloc;
            use super::*;
            use $crate::Semiring;

            fn elems() -> alloc::vec::Vec<$ty> {
                $crate::testing::test_rng(42, 20)
                    .into_iter()
                    .map($from_u64)
                    .collect()
            }

            #[test]
            fn additive_identity() {
                for a in elems() {
                    assert_eq!(a + <$ty as Semiring>::ZERO, a);
                }
            }
            #[test]
            fn multiplicative_identity() {
                for a in elems() {
                    assert_eq!(a * <$ty as Semiring>::ONE, a);
                }
            }
            #[test]
            fn add_commutative() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] + w[1], w[1] + w[0]);
                }
            }
            #[test]
            fn mul_commutative() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] * w[1], w[1] * w[0]);
                }
            }
            #[test]
            fn add_associative() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!((w[0] + w[1]) + w[2], w[0] + (w[1] + w[2]));
                }
            }
            #[test]
            fn mul_associative() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!((w[0] * w[1]) * w[2], w[0] * (w[1] * w[2]));
                }
            }
            #[test]
            fn distributive() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!(w[0] * (w[1] + w[2]), w[0] * w[1] + w[0] * w[2]);
                }
            }
            #[test]
            fn zero_annihilates() {
                for a in elems() {
                    assert_eq!(a * <$ty as Semiring>::ZERO, <$ty as Semiring>::ZERO);
                }
            }
        }
    };
}

/// test ring axioms (semiring + additive inverse).
#[macro_export]
macro_rules! test_ring_axioms {
    ($ty:ty, $mod_name:ident, $from_u64:expr) => {
        #[cfg(test)]
        mod $mod_name {
            extern crate alloc;
            use super::*;
            use $crate::Semiring;

            fn elems() -> alloc::vec::Vec<$ty> {
                $crate::testing::test_rng(42, 20)
                    .into_iter()
                    .map($from_u64)
                    .collect()
            }

            // semiring axioms
            #[test]
            fn additive_identity() {
                for a in elems() {
                    assert_eq!(a + <$ty as Semiring>::ZERO, a);
                }
            }
            #[test]
            fn multiplicative_identity() {
                for a in elems() {
                    assert_eq!(a * <$ty as Semiring>::ONE, a);
                }
            }
            #[test]
            fn add_commutative() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] + w[1], w[1] + w[0]);
                }
            }
            #[test]
            fn mul_commutative() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] * w[1], w[1] * w[0]);
                }
            }
            #[test]
            fn add_associative() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!((w[0] + w[1]) + w[2], w[0] + (w[1] + w[2]));
                }
            }
            #[test]
            fn mul_associative() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!((w[0] * w[1]) * w[2], w[0] * (w[1] * w[2]));
                }
            }
            #[test]
            fn distributive() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!(w[0] * (w[1] + w[2]), w[0] * w[1] + w[0] * w[2]);
                }
            }
            #[test]
            fn zero_annihilates() {
                for a in elems() {
                    assert_eq!(a * <$ty as Semiring>::ZERO, <$ty as Semiring>::ZERO);
                }
            }

            // ring axioms
            #[test]
            fn additive_inverse() {
                for a in elems() {
                    assert_eq!(a + (-a), <$ty as Semiring>::ZERO);
                }
            }
            #[test]
            fn sub_is_add_neg() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] - w[1], w[0] + (-w[1]));
                }
            }
        }
    };
}

/// test field axioms (ring + multiplicative inverse, sqrt, pow).
#[macro_export]
macro_rules! test_field_axioms {
    ($ty:ty, $mod_name:ident, $from_u64:expr) => {
        #[cfg(test)]
        mod $mod_name {
            extern crate alloc;
            use super::*;
            use $crate::{Field, Semiring};

            fn elems() -> alloc::vec::Vec<$ty> {
                $crate::testing::test_rng(42, 20)
                    .into_iter()
                    .map($from_u64)
                    .collect()
            }
            fn nonzero() -> alloc::vec::Vec<$ty> {
                elems()
                    .into_iter()
                    .filter(|e| *e != <$ty as Semiring>::ZERO)
                    .collect()
            }

            // semiring axioms
            #[test]
            fn additive_identity() {
                for a in elems() {
                    assert_eq!(a + <$ty as Semiring>::ZERO, a);
                }
            }
            #[test]
            fn multiplicative_identity() {
                for a in elems() {
                    assert_eq!(a * <$ty as Semiring>::ONE, a);
                }
            }
            #[test]
            fn add_commutative() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] + w[1], w[1] + w[0]);
                }
            }
            #[test]
            fn mul_commutative() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] * w[1], w[1] * w[0]);
                }
            }
            #[test]
            fn add_associative() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!((w[0] + w[1]) + w[2], w[0] + (w[1] + w[2]));
                }
            }
            #[test]
            fn mul_associative() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!((w[0] * w[1]) * w[2], w[0] * (w[1] * w[2]));
                }
            }
            #[test]
            fn distributive() {
                let e = elems();
                for w in e.windows(3) {
                    assert_eq!(w[0] * (w[1] + w[2]), w[0] * w[1] + w[0] * w[2]);
                }
            }
            #[test]
            fn zero_annihilates() {
                for a in elems() {
                    assert_eq!(a * <$ty as Semiring>::ZERO, <$ty as Semiring>::ZERO);
                }
            }

            // ring axioms
            #[test]
            fn additive_inverse() {
                for a in elems() {
                    assert_eq!(a + (-a), <$ty as Semiring>::ZERO);
                }
            }
            #[test]
            fn sub_is_add_neg() {
                let e = elems();
                for w in e.windows(2) {
                    assert_eq!(w[0] - w[1], w[0] + (-w[1]));
                }
            }

            // field axioms
            #[test]
            fn mul_inverse() {
                for a in nonzero() {
                    assert_eq!(a * a.inv(), <$ty as Semiring>::ONE);
                }
            }
            #[test]
            fn try_inv_zero() {
                assert!(<$ty as Field>::try_inv(<$ty as Semiring>::ZERO).is_none());
            }
            #[test]
            fn try_inv_nonzero() {
                for a in nonzero() {
                    assert_eq!(a.try_inv(), Some(a.inv()));
                }
            }
            #[test]
            fn square_is_mul() {
                for a in elems() {
                    assert_eq!(a.square(), a * a);
                }
            }
            #[test]
            fn double_is_add() {
                for a in elems() {
                    assert_eq!(a.double(), a + a);
                }
            }
            #[test]
            fn pow_zero() {
                for a in nonzero() {
                    assert_eq!(a.pow(0), <$ty as Semiring>::ONE);
                }
            }
            #[test]
            fn pow_one() {
                for a in elems() {
                    assert_eq!(a.pow(1), a);
                }
            }
            #[test]
            fn pow_two() {
                for a in elems() {
                    assert_eq!(a.pow(2), a.square());
                }
            }
            #[test]
            fn sqrt_of_square() {
                for a in elems() {
                    let sq = <$ty as Field>::square(a);
                    if let Some(root) = <$ty as Field>::sqrt(sq) {
                        assert_eq!(<$ty as Field>::square(root), sq, "sqrt(a²)² = a²");
                    }
                }
            }
        }
    };
}
