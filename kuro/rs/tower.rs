//! F₂ tower field arithmetic
//!
//! Tower construction: each level F_{2^{2k}} is defined as
//! F_{2^k}[x] / (x² + x + α_k) where α_k is the canonical
//! element of the previous level.
//!
//! Representation: each element of F_{2^n} is stored as n bits
//! packed into the smallest integer type that fits.
//!
//! Complete operations at every level:
//! - add: XOR (characteristic 2)
//! - mul: tower Karatsuba
//! - inv: tower-recursive via sub-field inversion
//! - square: Frobenius endomorphism (linear in char 2)
//! - sqrt: inverse Frobenius a^(2^(n-1))
//! - frobenius: a^(2^k) for any k
//! - trace: Tr(a) = a + a² + a⁴ + ... + a^(2^(n-1)) ∈ F₂
//! - norm: N(a) = a · a² (product down to sub-field)
//! - exp: general exponentiation by square-and-multiply

// ---------------------------------------------------------------------------
// F₂ — the base field. One bit.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct F2(pub u8); // only bit 0 is used

impl F2 {
    pub const ZERO: Self = F2(0);
    pub const ONE: Self = F2(1);

    #[inline(always)]
    pub fn add(self, rhs: Self) -> Self {
        F2(self.0 ^ rhs.0)
    }

    #[inline(always)]
    pub fn mul(self, rhs: Self) -> Self {
        F2(self.0 & rhs.0)
    }

    #[inline(always)]
    pub fn inv(self) -> Self {
        assert!(self.0 == 1, "inverse of zero");
        self
    }

    #[inline(always)]
    pub fn square(self) -> Self {
        self
    }

    #[inline(always)]
    pub fn sqrt(self) -> Self {
        self
    }

    #[inline(always)]
    pub fn trace(self) -> Self {
        self
    }

    #[inline(always)]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn from_u8(v: u8) -> Self {
        F2(v & 1)
    }
}

// ---------------------------------------------------------------------------
// F₂² — first extension. 2 bits.
// Defined by x² + x + 1 over F₂.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct F2_2(pub u8); // bits 0-1

impl F2_2 {
    pub const ZERO: Self = F2_2(0);
    pub const ONE: Self = F2_2(1);
    pub const BITS: u32 = 2;

    #[inline(always)]
    pub fn add(self, rhs: Self) -> Self {
        F2_2(self.0 ^ rhs.0)
    }

    #[inline(always)]
    pub fn mul(self, rhs: Self) -> Self {
        let a0 = self.0 & 1;
        let a1 = (self.0 >> 1) & 1;
        let b0 = rhs.0 & 1;
        let b1 = (rhs.0 >> 1) & 1;
        let c0 = (a0 & b0) ^ (a1 & b1);
        let c1 = (a0 & b1) ^ (a1 & b0) ^ (a1 & b1);
        F2_2(c0 | (c1 << 1))
    }

    #[inline(always)]
    pub fn square(self) -> Self {
        let a0 = self.0 & 1;
        let a1 = (self.0 >> 1) & 1;
        F2_2((a0 ^ a1) | (a1 << 1))
    }

    pub fn inv(self) -> Self {
        assert!(self.0 != 0, "inverse of zero");
        // |F₂²*| = 3, so a⁻¹ = a^(3-1) = a²
        self.square()
    }

    pub fn frobenius(self, k: u32) -> Self {
        let mut r = self;
        for _ in 0..k {
            r = r.square();
        }
        r
    }

    pub fn sqrt(self) -> Self {
        self.frobenius(1)
    }

    pub fn trace(self) -> F2 {
        F2((self.0 >> 1) & 1)
    }

    pub fn norm(self) -> F2 {
        let p = self.mul(self.square());
        F2(p.0 & 1)
    }

    pub fn exp(self, mut e: u128) -> Self {
        if e == 0 {
            return Self::ONE;
        }
        let mut base = self;
        let mut result = Self::ONE;
        while e > 0 {
            if e & 1 == 1 {
                result = result.mul(base);
            }
            base = base.square();
            e >>= 1;
        }
        result
    }

    #[inline(always)]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

// ---------------------------------------------------------------------------
// Macro: tower levels F₂⁴ through F₂¹²⁸
// ---------------------------------------------------------------------------

macro_rules! tower_level {
    (
        $name:ident, $repr:ty, $bits:expr,
        $half:ident, $half_bits:expr,
        $lo_mask:expr, $alpha:expr
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name(pub $repr);

        impl $name {
            pub const ZERO: Self = $name(0);
            pub const ONE: Self = $name(1);
            pub const BITS: u32 = $bits;

            /// Extension polynomial constant: x² + x + ALPHA.
            /// ALPHA = product of all previous tower generators (Wiedemann tower).
            const ALPHA: $half = $half($alpha);

            #[inline(always)]
            fn lo(self) -> $half {
                $half((self.0 & $lo_mask) as _)
            }

            #[inline(always)]
            fn hi(self) -> $half {
                $half((self.0 >> $half_bits) as _)
            }

            #[inline(always)]
            fn pack(lo: $half, hi: $half) -> Self {
                $name((lo.0 as $repr) | ((hi.0 as $repr) << $half_bits))
            }

            #[inline(always)]
            pub fn add(self, rhs: Self) -> Self {
                $name(self.0 ^ rhs.0)
            }

            #[inline]
            pub fn mul(self, rhs: Self) -> Self {
                let a_lo = self.lo();
                let a_hi = self.hi();
                let b_lo = rhs.lo();
                let b_hi = rhs.hi();

                let ll = a_lo.mul(b_lo);
                let hh = a_hi.mul(b_hi);
                let cross = a_lo.add(a_hi).mul(b_lo.add(b_hi)).add(ll).add(hh);

                let c_lo = ll.add(hh.mul(Self::ALPHA));
                let c_hi = cross.add(hh);

                Self::pack(c_lo, c_hi)
            }

            #[inline]
            pub fn square(self) -> Self {
                let lo = self.lo();
                let hi = self.hi();
                let lo_sq = lo.square();
                let hi_sq = hi.square();
                let c_lo = lo_sq.add(hi_sq.mul(Self::ALPHA));
                Self::pack(c_lo, hi_sq)
            }

            /// Frobenius: a^(2^k). Optimized for small k.
            pub fn frobenius(self, k: u32) -> Self {
                let mut r = self;
                for _ in 0..k {
                    r = r.square();
                }
                r
            }

            /// Square root: a^(2^(n-1)).
            /// Computed as `inv(a).square()` would be wrong.
            /// Use repeated squaring n-1 times. For correctness, not speed.
            pub fn sqrt(self) -> Self {
                self.frobenius($bits - 1)
            }

            /// Trace: Tr(a) ∈ F₂. Tower-recursive.
            /// Tr_{2^{2k}/2}(a_lo + a_hi·x) = Tr_{2^k/2}(a_lo + a_hi)
            /// since Tr_{2^{2k}/2^k}(a_lo + a_hi·x) = a_lo + a_hi (relative trace)
            /// and absolute trace = Tr_{2^k/2} ∘ Tr_{2^{2k}/2^k}.
            pub fn trace(self) -> F2 {
                let relative = self.lo().add(self.hi()); // Tr down one level
                relative.trace() // recurse
            }

            /// Norm to sub-field: N(a) = a · conjugate(a).
            /// conjugate(a_lo + a_hi·x) = a_lo + a_hi + a_hi·x
            pub fn norm(self) -> $half {
                let a_lo = self.lo();
                let a_hi = self.hi();
                // N(a) = a_lo·(a_lo + a_hi) + a_hi²·α
                a_lo.mul(a_lo.add(a_hi)).add(a_hi.square().mul(Self::ALPHA))
            }

            /// Tower-recursive inversion.
            /// For a = a_lo + a_hi·x:
            ///   Δ = a_lo·(a_lo + a_hi) + a_hi²·α
            ///   a⁻¹ = Δ⁻¹·(a_lo + a_hi) + Δ⁻¹·a_hi·x
            pub fn inv(self) -> Self {
                assert!(self.0 != 0, "inverse of zero");
                let a_lo = self.lo();
                let a_hi = self.hi();

                let delta = a_lo.mul(a_lo.add(a_hi)).add(a_hi.square().mul(Self::ALPHA));
                let delta_inv = delta.inv();

                let c_lo = delta_inv.mul(a_lo.add(a_hi));
                let c_hi = delta_inv.mul(a_hi);

                Self::pack(c_lo, c_hi)
            }

            pub fn exp(self, mut e: u128) -> Self {
                if e == 0 {
                    return Self::ONE;
                }
                let mut base = self;
                let mut result = Self::ONE;
                while e > 0 {
                    if e & 1 == 1 {
                        result = result.mul(base);
                    }
                    base = base.square();
                    e >>= 1;
                }
                result
            }

            #[inline(always)]
            pub fn is_zero(self) -> bool {
                self.0 == 0
            }
        }
    };
}

// Wiedemann tower: α_k = product of all previous generators.
// The constant at each level is 1 << (half_bits - 1) in the sub-field representation.
//                                                      lo_mask              alpha
tower_level!(F2_4, u8, 4, F2_2, 2, 0x03u8, 0x02); // α = x₀
tower_level!(F2_8, u8, 8, F2_4, 4, 0x0Fu8, 0x08); // α = x₀·x₁
tower_level!(F2_16, u16, 16, F2_8, 8, 0x00FFu16, 0x80); // α = x₀·x₁·x₂
tower_level!(F2_32, u32, 32, F2_16, 16, 0x0000_FFFFu32, 0x8000); // α = x₀···x₃
tower_level!(
    F2_64,
    u64,
    64,
    F2_32,
    32,
    0x0000_0000_FFFF_FFFFu64,
    0x8000_0000
); // α = x₀···x₄
tower_level!(
    F2_128,
    u128,
    128,
    F2_64,
    64,
    0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFFu128,
    0x8000_0000_0000_0000
); // α = x₀···x₅

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
//
// `inv()`'s zero-check was only a `debug_assert!`, compiled out in release
// (the profile `kuro-cli` and every downstream caller of the raw, unchecked
// `inv()` actually ships). At every level, `inv(0)` silently returned `0`
// instead of failing: the base `F2`/`F2_2` cases return `self` (already
// zero) or `square(0) == 0`; each tower level then reduces to the same
// silent zero through its own recursive `delta.inv()` call. `checked_inv_*`
// in `inv.rs` already guards every call it makes with `is_zero()` first —
// these are regressions on the raw `inv()` a caller can still reach
// directly, closing the gap pre-emptively, same framing as rows
// 77/80/81/82/85/86/90/93/99/108/117/118/120/125/126/128/130/131/139/151/
// 167/174/232/233/241 elsewhere in this codebase.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_inv_zero_panics() {
        F2::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_2_inv_zero_panics() {
        F2_2::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_4_inv_zero_panics() {
        F2_4::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_8_inv_zero_panics() {
        F2_8::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_16_inv_zero_panics() {
        F2_16::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_32_inv_zero_panics() {
        F2_32::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_64_inv_zero_panics() {
        F2_64::ZERO.inv();
    }

    #[test]
    #[should_panic(expected = "inverse of zero")]
    fn f2_128_inv_zero_panics() {
        F2_128::ZERO.inv();
    }

    // Correctness on non-zero inputs is unchanged: a * a.inv() == ONE.
    #[test]
    fn inv_is_multiplicative_inverse_on_nonzero() {
        assert_eq!(F2::ONE.inv(), F2::ONE);
        assert_eq!(F2_2::ONE.mul(F2_2::ONE.inv()), F2_2::ONE);
        assert_eq!(F2_4(5).mul(F2_4(5).inv()), F2_4::ONE);
        assert_eq!(F2_8(200).mul(F2_8(200).inv()), F2_8::ONE);
        assert_eq!(F2_16(4321).mul(F2_16(4321).inv()), F2_16::ONE);
        assert_eq!(F2_32(123_456_789).mul(F2_32(123_456_789).inv()), F2_32::ONE);
        assert_eq!(F2_64(0x1234_5678_9abc_def1).mul(F2_64(0x1234_5678_9abc_def1).inv()), F2_64::ONE);
        assert_eq!(
            F2_128(0x1234_5678_9abc_def1_0011_2233_4455_6677).mul(
                F2_128(0x1234_5678_9abc_def1_0011_2233_4455_6677).inv()
            ),
            F2_128::ONE
        );
    }
}
