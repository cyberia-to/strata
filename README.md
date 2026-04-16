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

one algebra cannot span all computation. each structure matches the shape of the
computation it verifies:

| algebra | structure | why it exists |
|---------|-----------|--------------|
| [[nebu]] | Goldilocks field F_p | arithmetic: 4-5 cycle multiply, 2^32 roots of unity for NTT |
| [[kuro]] | binary tower F₂¹²⁸ | bitwise: XOR/AND at 1 constraint, 128 elements per machine word |
| [[jali]] | polynomial ring R_q | lattice: FHE bootstrapping, NTT-based ring multiply |
| [[trop]] | tropical semiring (min,+) | optimization: shortest path, assignment, Viterbi, transport |
| [[genies]] | isogeny field F_q | privacy: stealth addresses, VDF, blind signatures (512-bit, constant-time) |

## four tiers

traits organized by who needs them — not by abstract algebra taxonomy:

```
┌──────────────────────────────────────────────────────┐
│              TIER 1: universal (strata-core)          │
│                                                      │
│  Encode      encode, decode                    │
│  Semiring    add, mul, zero, one          ← trop     │
│  Ring        + sub, neg                   ← jali     │
│  Field       + inv, square, pow           ← nebu,    │
│                                             kuro,    │
│                                             genies   │
└───────────────────────┬──────────────────────────────┘
                        │
       ┌────────────────┼────────────────┐
       │                │                │
  TIER 2: proofs   TIER 3: compute  TIER 4: structure
  (strata-core-    (strata-core-    (strata-core-
   proof)           compute)         ext)

  Reduce       Spectral         Extension<Base>
    reduce        roots of         base, degree,
    (bytes→F)        unity, NTT       frobenius map

  Dot              Bits             Batch
    sum_of_          to_bits,         batch_inv
    products         from_bits        (Montgomery)

                                    Blind
                                      ct_eq, ct_select,
                                      ct_swap
```

### tier 1: universal

every algebra implements at least one level. hemera needs this tier only.

```rust
use strata_core::{Semiring, Ring, Field, Codec};
```

**Semiring** — add and multiply with identities. the tropical semiring (min, +) lives
here: min has no inverse, so no subtraction. trop implements Semiring and stops.

**Ring** — semiring with subtraction. polynomial ring R_q (jali) conceptually lives here.

**Field** — ring with multiplicative inverse. Goldilocks (nebu), F₂¹²⁸ (kuro), and F_q
(genies) all implement this.

**Encode** — serialize to/from bytes. every type implements this. no more ad-hoc
`to_le_bytes` scattered across crates.

### tier 2: proofs

lens (polynomial commitment) and zheng (constraint verification) need this.
hemera and nox don't.

```rust
use strata_proof::{Reduce, Dot};
```

**Reduce** — reduce hash output bytes to a field element. this is the bridge
between hemera (which produces bytes) and field operations (which need elements).
every Fiat-Shamir challenge in the stack goes through this trait.

**Dot** — fused multiply-accumulate: Σ aᵢ·bᵢ. zheng evaluates CCS constraints
as matrix-vector products over field elements. the default implementation is a
loop; algebras can override with hardware FMA or delayed modular reduction.

### tier 3: compute

nox (VM execution) and jali (ring arithmetic) need this.

```rust
use strata_compute::{Spectral, Bits};
```

**Spectral** — a field with roots of unity. the spectral domain (NTT evaluation domain)
exists, enabling O(N log N) polynomial operations instead of O(N²). Goldilocks has
two-adicity 32 — the multiplicative group F_p* has a subgroup of order 2^32 generated
by the 2^32-th root of unity. F₂¹²⁸ and F_q lack this structure.

the name comes from spectral methods in numerical analysis: transform between coefficient
and evaluation domains, compute in whichever is cheaper, transform back.

**Bits** — decompose field elements into bits and reconstruct. nox uses this for
comparison (lt), shifts, and masks. Binius (lens) uses it for binary constraint encoding.

### tier 4: structure

specific algebraic structures that not every algebra needs.

```rust
use strata_ext::{Extension, Batch, Blind};
```

**Extension\<Base\>** — tower fields. Fp2, Fp3, Fp4 over Goldilocks (nebu extensions).
F₂ → F₂² → F₂⁴ → ... → F₂¹²⁸ (kuro tower). an extension field has a base field,
a degree, and a Frobenius endomorphism.

**Batch** — Montgomery's trick: invert N elements with 1 inversion + 3(N-1)
multiplications. nebu, kuro, and genies all implement this.

**Blind** — timing-safe operations. genies (CSIDH) requires this: isogeny walks
on secret exponents must not leak timing information. ct_eq, ct_select, ct_swap —
no branches on secret data.

## what each algebra implements

| algebra | Codec | Semiring | Ring | Field | Reduce | Dot | Spectral | Bits | Extension | Batch | Blind |
|---------|--------|----------|------|-------|------------|-----|----------|------|-----------|-------|-------------|
| nebu | yes | yes | yes | yes | yes | yes | yes | yes | — | yes | — |
| kuro | yes | yes | yes | yes | yes | yes | — | — | — | yes | — |
| jali | — | — | — | — | — | — | — | — | — | — | — |
| trop | yes | yes | — | — | — | — | — | — | — | — | — |
| genies | yes | yes | yes | yes | yes | yes | — | — | — | yes | yes |

jali's RingElement (32 KiB fixed array) is too large for Copy, so it doesn't implement
the scalar traits. Ikat (lens) operates on its NTT slots — which are Goldilocks scalars.

## the five algebras

### nebu — Goldilocks field

p = 2^64 - 2^32 + 1. reduction is two shifts and an add. 4-5 cycle multiply.
2^32 roots of unity for NTT. the workhorse of the stack.

```rust
use nebu::Goldilocks;
use strata_core::Field;

let a = Goldilocks::new(42);
let b = a.inv();
assert_eq!(a * b, Goldilocks::ONE);
```

73 tests. extensions: Fp2, Fp3, Fp4.

### kuro — binary tower

F₂ → F₂² → F₂⁴ → F₂⁸ → F₂¹⁶ → F₂³² → F₂⁶⁴ → F₂¹²⁸. each level defined by
x² + x + α (Wiedemann tower). addition = XOR. multiplication = Karatsuba over
tower levels. 128 elements packed in one u128.

```rust
use kuro::F2_128;

let a = F2_128(0xDEAD_BEEF);
assert_eq!(a + a, F2_128::ZERO);  // char 2
assert_eq!(-a, a);                 // negation is identity
```

77 tests. Packed128: 128 F₂ elements for SIMD-style operations.

### jali — polynomial ring

R_q = F_p[x]/(x^n+1) with n up to 4096. negacyclic NTT for O(n log n) ring multiply.
noise tracking for FHE correctness. Galois automorphisms for key switching.

```rust
use jali::ring::RingElement;
let a = RingElement::new(1024);
let b = a.mul(&a);  // NTT-based polynomial multiplication
```

70 tests.

### trop — tropical semiring

addition = min. multiplication = saturating add. identity: zero = +inf, one = 0.
no subtraction — you cannot un-min. proves optimization: Dijkstra, Hungarian,
Viterbi, Kantorovich.

```rust
use trop::Tropical;
use strata_core::Semiring;

let a = Tropical::from_u64(3);
let b = Tropical::from_u64(7);
assert_eq!(a + b, Tropical::from_u64(3));   // min(3, 7) = 3
assert_eq!(a * b, Tropical::from_u64(10));   // 3 + 7 = 10
```

77 tests. Kleene star, determinant, eigenvalue.

### genies — isogeny curves

512-bit prime q = 4·3·5·7·...·587 - 1 (CSIDH-512). the one module with a foreign
prime — Goldilocks p+1 has no small odd factors, making CSIDH impossible over F_p.
eight u64 limbs, schoolbook multiplication, Barrett reduction. all arithmetic is
constant-time.

```rust
use genies::Fq;
use strata_core::Field;

let a = Fq::from_u64(42);
assert_eq!((a * a.inv()), Fq::ONE);
```

55 tests. Montgomery curves, isogeny walks, class group action.

## crates

| crate | what |
|-------|------|
| [strata-core](core/) | tier 1: Codec, Semiring, Ring, Field |
| [strata-proof](proof/) | tier 2: Reduce, Dot |
| [strata-compute](compute/) | tier 3: Spectral, Bits |
| [strata-ext](ext/) | tier 4: Extension, Batch, Blind |
| [cyb-nebu](nebu/rs/) | Goldilocks F_p (73 tests) |
| [cyb-kuro](kuro/rs/) | F₂ binary tower (77 tests) |
| [cyb-jali](jali/rs/) | polynomial ring R_q (70 tests) |
| [cyb-trop](trop/rs/) | tropical semiring (77 tests) |
| [cyb-genies](genies/rs/) | isogeny curves F_q (55 tests) |
| [strata](src/) | facade: re-exports everything |

```toml
# everything
[dependencies]
strata = "0.1"

# just the traits (for libraries generic over Field)
[dependencies]
strata-core = "0.1"

# traits + proof tier (for lens, zheng)
[dependencies]
strata-core = "0.1"
strata-proof = "0.1"

# one specific algebra
[dependencies]
cyb-nebu = "0.1"
```

## who uses this

| consumer | tiers needed | why |
|----------|-------------|-----|
| [[hemera]] | 1 (Field) | Poseidon2 hash: add, mul, pow7, inv over Goldilocks |
| [[lens]] | 1 + 2 (Field + Reduce) | commit: encode + hash. open: Fiat-Shamir challenges |
| [[nox]] | 1 + 3 (Field + Spectral + Bits) | VM registers, NTT jets, comparison, bit ops |
| [[zheng]] | 1 + 2 (Field + Reduce + Dot) | constraint evaluation, Fiat-Shamir, matrix products |
| [[bbg]] | 1 (Field + Encode) | state polynomial serialization |
| [[mudra]] | 1 + 4 (Field + Blind) | CSIDH key exchange, threshold protocols |

## workspace

```
algebra/
├── core/           strata-core           Semiring → Ring → Field + Encode
├── proof/          strata-proof     Reduce + Dot
├── compute/        strata-compute   Spectral + Bits
├── ext/            strata-ext       Extension + Batch + Blind
├── src/            strata         facade
├── nebu/           Goldilocks F_p
│   ├── rs/         core (73 tests)
│   ├── wgsl/       GPU compute shaders
│   ├── cli/        command-line tool
│   ├── tri/        Trident ZK circuits
│   └── specs/      canonical specs
├── kuro/           F₂ binary tower (77 tests)
├── jali/           polynomial ring R_q (70 tests)
├── trop/           tropical semiring (77 tests)
└── genies/         isogeny curves F_q (55 tests)
```

## 352 tests

```bash
cargo test -p cyb-nebu -p cyb-kuro -p cyb-jali -p cyb-trop -p cyb-genies
```

## license

cyber license: don't trust. don't fear. don't beg.
