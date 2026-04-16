---
tags: cyber, core
alias: algebra, five algebras, algebraic backends
crystal-type: entity
crystal-domain: math
---
# algebra

five algebraic structures for verifiable computation. the arithmetic foundation
of the [[cyber]] stack — every proof, every hash, every commitment reduces to
operations in one of these five algebras.

```
algebra → hemera (hash) → lens (commitment) → nox (execution) → zheng (proof) → bbg (state)
```

## why five

one algebra cannot span all computation. arithmetic circuits want fast prime fields.
bitwise operations want characteristic 2. FHE needs polynomial rings. optimization
needs tropical semirings. privacy needs isogeny curves. each structure matches the
shape of the computation it verifies:

| algebra | structure | operations | what it proves |
|---------|-----------|-----------|---------------|
| [[nebu]] | Goldilocks field F_p | add, mul, inv in 4-5 cycles | arithmetic: tri-kernel, state transitions |
| [[kuro]] | binary tower F₂¹²⁸ | XOR, AND at 1 constraint each | bitwise: quantized AI inference, comparison |
| [[jali]] | polynomial ring R_q | NTT multiply, automorphisms | lattice: FHE bootstrapping, KEM |
| [[trop]] | tropical semiring (min,+) | shortest path, assignment | optimization: routing, transport, Viterbi |
| [[genies]] | isogeny field F_q | 512-bit curve walks | privacy: stealth addresses, VDF, blind signatures |

a circuit that mixes bitwise and arithmetic operations splits across kuro and nebu.
a circuit that proves FHE correctness splits across jali, kuro, and nebu. the proof
system ([[zheng]]) folds them together — the algebra layer provides the scalars.

## the trait hierarchy

three levels. each includes the previous:

```rust
/// semiring: add and multiply with identities. no subtraction.
/// trop lives here — min has no inverse.
trait Semiring: Copy + Eq + Add + Mul {
    const ZERO: Self;  // additive identity
    const ONE: Self;   // multiplicative identity
}

/// ring: semiring with subtraction and negation.
/// jali's RingElement would live here (if it were Copy).
trait Ring: Semiring + Sub + Neg {}

/// field: ring with multiplicative inverse.
/// nebu, kuro, genies satisfy this.
trait Field: Ring {
    fn inv(self) -> Self;
    fn from_hash(bytes: &[u8]) -> Self;
}
```

the hierarchy is the API contract between algebra and the rest of the stack.
[[hemera]] hashes field elements. [[lens]] commits multilinear polynomials over fields.
[[zheng]] verifies constraints over fields. they all program against these traits,
not against concrete types.

## the five algebras

### nebu — Goldilocks field (F_p)

the workhorse. p = 2^64 - 2^32 + 1 — chosen because reduction is two shifts
and an add, giving 4-5 cycle multiply on modern CPUs. the 2^32 roots of unity
enable NTT (number theoretic transform) for fast polynomial multiplication.

```rust
use nebu::Goldilocks;

let a = Goldilocks::new(42);
let b = a.inv();
assert_eq!(a * b, Goldilocks::ONE);
```

73 tests. extensions: Fp2, Fp3, Fp4 for higher-degree arithmetic.

### kuro — binary tower (F₂¹²⁸)

128 field elements packed in one machine word. addition is XOR (one instruction).
multiplication is Karatsuba over the tower: F₂ → F₂² → F₂⁴ → ... → F₂¹²⁸,
where each level is defined by x² + x + α.

```rust
use kuro::F2_128;

let a = F2_128(0xDEAD_BEEF);
let b = F2_128(0xCAFE_BABE);
assert_eq!(a + a, F2_128::ZERO);  // char 2: a + a = 0
assert_eq!(-a, a);                 // char 2: negation is identity
```

77 tests. Packed128: 128 F₂ elements in one u128 for SIMD-style batch operations.

### jali — polynomial ring (R_q)

R_q = F_p[x]/(x^n+1) with n up to 4096. the cyclotomic ring where FHE lives.
multiply via negacyclic NTT: twist by ψ^i, forward NTT, pointwise multiply,
inverse NTT, untwist. noise tracking monitors decryption budget through operations.

```rust
use jali::ring::RingElement;
use nebu::Goldilocks;

let mut a = RingElement::new(1024);
a.coeffs[0] = Goldilocks::new(42);
let b = a.mul(&a);  // polynomial multiplication via NTT
```

70 tests. automorphisms: Galois σ_k for key switching in FHE.

### trop — tropical semiring (min, +)

tropical addition is min. tropical multiplication is ordinary addition (saturating).
additive identity is +inf. multiplicative identity is 0. no subtraction exists —
you cannot un-min.

this algebra proves optimization: shortest paths (Dijkstra), assignment (Hungarian),
sequence alignment (Viterbi), transport (Kantorovich). the prover runs the algorithm,
the verifier checks the witness via LP dual certificates.

```rust
use trop::Tropical;

let a = Tropical::from_u64(3);
let b = Tropical::from_u64(7);
assert_eq!(a.add(b), Tropical::from_u64(3));   // min(3, 7) = 3
assert_eq!(a.mul(b), Tropical::from_u64(10));   // 3 + 7 = 10
```

77 tests. Kleene star (all-pairs shortest paths), tropical determinant,
minimum mean cycle eigenvalue.

### genies — isogeny curves (F_q)

512-bit prime field for CSIDH: q = 4 · 3 · 5 · 7 · ... · 587 - 1.
the one module with a foreign prime — Goldilocks p+1 has no small odd factors,
making CSIDH impossible over F_p. eight u64 limbs, schoolbook multiplication,
Barrett reduction. all arithmetic is constant-time (no secret-dependent branching).

```rust
use genies::Fq;

let a = Fq::from_u64(42);
let b = Fq::inv(&a);
assert_eq!(Fq::mul(&a, &b), Fq::ONE);
```

55 tests. Montgomery curves, isogeny walks, class group action for
Diffie-Hellman, VRF, VDF, blind signatures.

## crates

| crate | crates.io | what |
|-------|-----------|------|
| cyb-algebra | [cyb-algebra](https://crates.io/crates/cyb-algebra) | Semiring, Ring, Field traits |
| cyb-nebu | [cyb-nebu](https://crates.io/crates/cyb-nebu) | Goldilocks F_p |
| cyb-kuro | [cyb-kuro](https://crates.io/crates/cyb-kuro) | F₂ binary tower |
| cyb-jali | [cyb-jali](https://crates.io/crates/cyb-jali) | polynomial ring R_q |
| cyb-trop | [cyb-trop](https://crates.io/crates/cyb-trop) | tropical semiring |
| cyb-genies | [cyb-genies](https://crates.io/crates/cyb-genies) | isogeny curves F_q |
| cyber-algebra | [cyber-algebra](https://crates.io/crates/cyber-algebra) | facade: re-exports all |

```toml
# everything
[dependencies]
cyber-algebra = "0.1"

# just one algebra
[dependencies]
cyb-nebu = "0.1"

# just the traits (for libraries generic over Field)
[dependencies]
cyb-algebra = "0.1"
```

## workspace structure

```
algebra/
├── core/           cyb-algebra        Semiring → Ring → Field
├── src/            cyber-algebra      facade re-exports
├── nebu/                              Goldilocks F_p
│   ├── rs/         cyb-nebu           core library (73 tests)
│   ├── wgsl/       nebu-wgsl          GPU compute shaders
│   ├── cli/        nebu-cli           command-line tool
│   ├── tri/                           Trident ZK circuits
│   └── specs/                         canonical specifications
├── kuro/           (same structure)   binary tower F₂
├── jali/                              polynomial ring R_q
├── trop/                              tropical semiring
└── genies/                            isogeny curves F_q
```

## who uses this

| consumer | what it needs | why |
|----------|--------------|-----|
| [[hemera]] | Goldilocks (nebu) | Poseidon2 hash operates over F_p |
| [[lens]] | all five fields | polynomial commitment per algebra |
| [[nox]] | Goldilocks, F₂ (nebu, kuro) | VM registers are field elements |
| [[zheng]] | all five fields | constraint verification per algebra |
| [[mudra]] | R_q, F_q (jali, genies) | KEM, CSIDH, FHE protocols |

## 352 tests

```bash
cargo test -p cyb-nebu -p cyb-kuro -p cyb-jali -p cyb-trop -p cyb-genies
```

## license

cyber license: don't trust. don't fear. don't beg.
