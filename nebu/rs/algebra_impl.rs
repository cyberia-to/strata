//! trait implementations for Goldilocks — all four tiers.

use crate::field::{Goldilocks, P};
use strata_compute::{Bits, Spectral};
use strata_core::{Codec, Field, Ring, Semiring};
use strata_ext::Batch;
use strata_proof::{Dot, Reduce};

extern crate alloc;
use alloc::vec::Vec;

// ── tier 1: universal ────────────────────────────────────────────

impl Codec for Goldilocks {
    fn byte_len() -> usize {
        8
    }
    fn encode(&self, buf: &mut [u8]) {
        buf[..8].copy_from_slice(&self.as_u64().to_le_bytes());
    }
    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[..8]);
        let val = u64::from_le_bytes(buf);
        // reject non-canonical: value must be < p
        if val >= P {
            return None;
        }
        Some(Goldilocks::new(val))
    }
}

impl Semiring for Goldilocks {
    const ZERO: Self = Goldilocks::ZERO;
    const ONE: Self = Goldilocks::ONE;
}

impl Ring for Goldilocks {}

impl Field for Goldilocks {
    #[inline]
    fn inv(self) -> Self {
        Goldilocks::inv(self)
    }
    #[inline]
    fn square(self) -> Self {
        Goldilocks::square(self)
    }
    fn sqrt(self) -> Option<Self> {
        crate::sqrt::sqrt(self)
    }
}

// ── tier 2: proof ────────────────────────────────────────────────

impl Reduce for Goldilocks {
    fn reduce(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 8, "need at least 8 bytes for Goldilocks");
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[..8]);
        Goldilocks::new(u64::from_le_bytes(buf)).canonicalize()
    }
}

impl Dot for Goldilocks {
    /// inner product, each product reduced by the field multiply.
    ///
    /// a delayed-reduction u128 accumulator does not work here: p^2 > 2^127,
    /// so the sum of two unreduced products already overflows u128.
    fn dot(a: &[Self], b: &[Self]) -> Self {
        assert_eq!(a.len(), b.len());
        // Each product is reduced by the field multiply; the u128 sum of
        // two unreduced products would already overflow (p^2 > 2^127).
        a.iter()
            .zip(b.iter())
            .fold(Goldilocks::ZERO, |acc, (&x, &y)| acc + x * y)
    }
}

// ── tier 3: compute ──────────────────────────────────────────────

/// primitive root of F_p* (generator = 7).
const G: u64 = 7;

impl Spectral for Goldilocks {
    /// p - 1 = 2^32 · (2^32 - 1), so two-adicity = 32.
    const TWO_ADICITY: u32 = 32;

    /// 2^32-th root of unity: 7^((p-1)/2^32) = 7^(2^32 - 1) mod p.
    ///
    /// verified: ROOT_OF_UNITY^(2^32) = 1 mod p.
    /// verified: ROOT_OF_UNITY^(2^31) ≠ 1 mod p (primitive).
    const ROOT_OF_UNITY: Self = Goldilocks::new(0x185629DCDA58878C);

    /// inverse of the root of unity: ROOT_OF_UNITY^(-1) mod p.
    ///
    /// verified: ROOT_OF_UNITY * ROOT_OF_UNITY_INV = 1 mod p.
    const ROOT_OF_UNITY_INV: Self = Goldilocks::new(0x76B6B635B6FC8719);

    const GENERATOR: Self = Goldilocks::new(G);
}

impl Bits for Goldilocks {
    const NUM_BITS: u32 = 64;

    fn to_bits_le(&self) -> Vec<bool> {
        let v = self.as_u64();
        (0..64).map(|i| (v >> i) & 1 == 1).collect()
    }

    fn from_bits_le(bits: &[bool]) -> Self {
        let mut v = 0u64;
        for (i, &bit) in bits.iter().enumerate().take(64) {
            if bit {
                v |= 1 << i;
            }
        }
        Goldilocks::new(v).canonicalize()
    }
}

// ── tier 4: structure ────────────────────────────────────────────

impl Batch for Goldilocks {
    fn batch_inv(elements: &mut [Self]) {
        let n = elements.len();
        if n == 0 {
            return;
        }

        // Montgomery's trick: prefix products → invert → propagate back
        let mut prefix = Vec::with_capacity(n);
        let mut acc = Self::ONE;
        for &e in elements.iter() {
            if e == Self::ZERO {
                prefix.push(acc);
            } else {
                acc *= e;
                prefix.push(acc);
            }
        }

        let mut inv_acc = acc.inv();

        for i in (1..n).rev() {
            if elements[i] == Self::ZERO {
                continue;
            }
            let inv_i = inv_acc * prefix[i - 1];
            inv_acc *= elements[i];
            elements[i] = inv_i;
        }
        if elements[0] != Self::ZERO {
            elements[0] = inv_acc;
        }
    }
}

// ── property tests ───────────────────────────────────────────────

strata_core::test_field_axioms!(Goldilocks, goldilocks_axioms, |v: u64| Goldilocks::new(v)
    .canonicalize());

#[cfg(test)]
mod spectral_tests {
    use super::*;
    use strata_compute::Spectral;
    use strata_core::Field;

    #[test]
    fn root_of_unity_order() {
        // ROOT_OF_UNITY^(2^32) = 1
        let mut r = Goldilocks::ROOT_OF_UNITY;
        for _ in 0..32 {
            r = r.square();
        }
        assert_eq!(r, Goldilocks::ONE, "root^(2^32) should be 1");
    }

    #[test]
    fn root_of_unity_primitive() {
        // ROOT_OF_UNITY^(2^31) ≠ 1
        let mut r = Goldilocks::ROOT_OF_UNITY;
        for _ in 0..31 {
            r = r.square();
        }
        assert_ne!(
            r,
            Goldilocks::ONE,
            "root^(2^31) should NOT be 1 (primitive)"
        );
    }

    #[test]
    fn root_inv_product() {
        assert_eq!(
            Goldilocks::ROOT_OF_UNITY * Goldilocks::ROOT_OF_UNITY_INV,
            Goldilocks::ONE,
            "root * root_inv should be 1"
        );
    }

    #[test]
    fn generator_is_primitive_root() {
        // g = 7 should be a primitive root of F_p*
        // g^((p-1)/2) should be -1 (Euler criterion for primitive root)
        let half_order = (P - 1) / 2;
        let result = Goldilocks::GENERATOR.pow(half_order);
        assert_eq!(
            result,
            Goldilocks::NEG_ONE,
            "generator^((p-1)/2) should be -1"
        );
    }

    #[test]
    fn root_of_unity_at_various_sizes() {
        // root_of_unity(n) should have order exactly n
        for log_n in 1..=20 {
            let n = 1usize << log_n;
            let omega = Goldilocks::root_of_unity(n);
            // omega^n = 1
            assert_eq!(
                omega.pow(n as u64),
                Goldilocks::ONE,
                "omega^{n} should be 1"
            );
            // omega^(n/2) ≠ 1 (primitive)
            if n > 1 {
                assert_ne!(
                    omega.pow((n / 2) as u64),
                    Goldilocks::ONE,
                    "omega^({}/2) should NOT be 1",
                    n
                );
            }
        }
    }
}
