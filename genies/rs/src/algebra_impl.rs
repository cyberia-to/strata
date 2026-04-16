//! Semiring, Ring, Field implementations for Fq.

use crate::fq::Fq;
use cyb_algebra::{Field, Ring, Semiring};

impl Semiring for Fq {
    const ZERO: Self = Fq::ZERO;
    const ONE: Self = Fq::ONE;
}

impl Ring for Fq {}

impl Field for Fq {
    #[inline]
    fn inv(self) -> Self {
        Fq::inv(&self)
    }

    fn from_hash(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 64, "need at least 64 bytes for Fq");
        let mut limbs = [0u64; 8];
        for (i, chunk) in bytes[..64].chunks_exact(8).enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(chunk);
            limbs[i] = u64::from_le_bytes(buf);
        }
        Fq::reduce(&Fq::from_limbs(limbs).limbs)
    }
}
