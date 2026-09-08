# strata 0.1.0 release plan

> Standard structure reference: ~/cyber/hemera
> lens depends on strata — strata must publish first.

---

## gap analysis vs hemera template

hemera structure:
```
repo/
├── bench/          separate benchmark crate (publish=false)
├── cli/            binary crate
├── docs/           Diataxis: explanation/, guides/, tutorials/
├── roadmap/        future proposals (markdown files)
├── rs/             core implementation crate
├── specs/          canonical specification
├── vectors/        JSON test vector pinning
├── wgsl/           GPU backend crate
├── CHANGELOG.md
├── CLAUDE.md       comprehensive, project-specific
└── Cargo.toml      workspace with lints + panic=abort profiles
```

strata current structure:
```
repo/
├── core/ proof/ compute/ ext/   tier trait crates
├── src/                         facade crate
├── nebu/ kuro/ jali/ trop/ genies/   each has: rs/, wgsl/, cli/, specs/
└── Cargo.toml
```

**present** (matches hemera per-algebra pattern):
- `<algebra>/rs/` — core implementation ✓
- `<algebra>/wgsl/` — GPU backend ✓
- `<algebra>/cli/` — CLI binary ✓
- `<algebra>/specs/` — canonical specs ✓ (nebu has tri/ for ZK circuits)

**missing at workspace level:**
- `bench/` — no separate benchmark crate (benchmarks are embedded in rs/ crates)
- `docs/` — no Diataxis documentation
- `roadmap/` — no future proposals
- `vectors/` — no JSON test vector pinning
- `CHANGELOG.md`
- `CLAUDE.md` — no workspace-level agent instructions
- `.github/workflows/ci.yml` — only notify-cyber.yml exists
- workspace `Cargo.toml` lints + `panic = "abort"` profiles

---

## Phase 1: workspace structure

### 1.1 Cargo.toml — add lints and profiles

**File:** `Cargo.toml`

Add to existing workspace manifest:
```toml
[workspace.lints.rust]
missing_debug_implementations = "warn"

[workspace.lints.clippy]
unused-async = "warn"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

Each crate's Cargo.toml must add `[lints] workspace = true`.

### 1.2 CI pipeline

**File to create:** `.github/workflows/ci.yml`

Four jobs matching hemera's ci.yml:
```yaml
name: CI
on:
  push:
    branches: [master]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test -p strata-nebu -p strata-kuro -p strata-jali -p strata-trop -p strata-genies

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace -- -D warnings

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt
      - run: cargo +nightly fmt --check

  doc:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --workspace --no-deps
```

Note: tier trait crates (core, proof, compute, ext) and WGSL/CLI subcrates are in
the workspace — `cargo doc --workspace` covers all of them. Exclude no-std crates
if doc build requires feature flags; add `[package.metadata.docs.rs]` per crate.

### 1.3 bench/ crate

**File to create:** `bench/Cargo.toml`
```toml
[package]
name = "strata-bench"
version.workspace = true
edition.workspace = true
publish = false

[dev-dependencies]
criterion = "0.5"
nebu = { workspace = true }
kuro = { workspace = true }
jali = { workspace = true }
trop = { workspace = true }
genies = { workspace = true }

[[bench]]
name = "field_ops"
harness = false

[[bench]]
name = "ntt"
harness = false

[[bench]]
name = "isogeny"
harness = false
```

Add `"bench"` to workspace `members`.

**Benchmarks to write:**
- `bench/benches/field_ops.rs` — mul, add, inv for Goldilocks and F₂¹²⁸
- `bench/benches/ntt.rs` — forward/inverse NTT at sizes 2^10, 2^16, 2^20
- `bench/benches/isogeny.rs` — single CSIDH isogeny walk

Move existing embedded benches from `nebu/rs/benches/` etc. into `bench/`.

### 1.4 vectors/ — JSON test vector pinning

**File to create:** `vectors/strata.json`

Cross-implementation verification anchors. Format mirrors `hemera/vectors/hemera.json`:
```json
{
  "nebu": {
    "add": { "zero_plus_one": "0000000000000001", "p_minus_1_plus_1": "0000000000000000" },
    "mul": { "two_times_two": "0000000000000004" },
    "inv": { "two": "8000000000000001" },
    "ntt_4": { "identity": [...] }
  },
  "kuro": {
    "mul": { "one_times_one": "00000000000000000000000000000001" },
    "inv": { "generator": "..." }
  },
  "trop": {
    "add_min": { "three_and_seven": "0000000000000003" },
    "mul_plus": { "three_and_seven": "000000000000000a" }
  },
  "genies": {
    "mul": { "one_times_one": "..." },
    "legendre": { "one": "01", "p_minus_1": "ff" }
  }
}
```

**Test file to create:** `<algebra>/rs/tests/vectors.rs` for each algebra — reads
JSON and asserts outputs match. This cross-verifies CPU and (eventually) GPU results.

### 1.5 docs/ — Diataxis structure

**Files to create:**
```
docs/
├── README.md                          index: what lives where
└── explanation/
    ├── README.md
    ├── why-five-algebras.md           the field selection rationale (content from README)
    ├── tier-hierarchy.md              why four tiers, what each tier enables
    ├── goldilocks.md                  why p = 2^64 - 2^32 + 1
    ├── binary-tower.md                why F₂ and the Wiedemann construction
    ├── tropical-semiring.md           why (min,+) and what it proves
    ├── isogeny-field.md               why CSIDH-512 for privacy
    └── performance.md                 NTT benchmarks, field op costs
```

guides/ and tutorials/ created as stubs — content deferred post-release.

### 1.6 roadmap/

**Files to create:**
```
roadmap/
├── README.md
├── nebu-avx512.md       AVX-512 packed Goldilocks
├── kuro-clmul.md        CLMUL hardware acceleration for F₂¹²⁸
├── genies-ct.md         constant-time action() via dummy isogenies
├── jali-ntt-gpu.md      NTT on GPU for ring encryption
└── new-algebra.md       process for adding a sixth algebra
```

### 1.7 CHANGELOG.md

**File to create:** `CHANGELOG.md`
```markdown
# Changelog

## [0.1.0] - 2026-05-14

### Added
- `strata-core`: Codec, Semiring, Ring, Field trait hierarchy
  with `test_semiring_axioms!`, `test_ring_axioms!`, `test_field_axioms!` macros
- `strata-proof`: Reduce, Dot traits
- `strata-compute`: Spectral, Packed, Bits traits
- `strata-ext`: Extension, Batch, Blind traits
- `strata-nebu`: Goldilocks F_p, NTT, Fp2/Fp3/Fp4, AVX2
- `strata-kuro`: F₂¹²⁸ binary tower
- `strata-jali`: polynomial ring R_q
- `strata-trop`: tropical semiring (min,+)
- `strata-genies`: F_q CSIDH-512, constant-time Blind ops
- `strata`: facade
- GPU backends (WGSL) and CLI tools for each algebra
```

### 1.8 CLAUDE.md

**File to create:** `CLAUDE.md` at workspace root

Model on hemera's CLAUDE.md (629 lines). Content:
1. agent collaboration (from cyber/midao/dev.md)
2. engineering patterns (from cyber/midao/engineering.md)
3. quality control: 12 passes, severity tiers (from cyber/midao/quality.md)
4. project structure (from cyber/midao/projects.md)
5. documentation methodology (from cyber/midao/documentation.md)
6. strata-specific: algebra contracts, tier dependencies, dual-stream optimization,
   companion repos (lens, nox, zheng, bbg)
7. do-not-touch zones: Cargo.toml versions, specs/ canonical, Lens/Field trait interfaces

---

## Phase 2: per-algebra specs/ audit

Each algebra has `<algebra>/specs/` directory. Audit each for:

- [ ] `nebu/specs/`: verify goldilocks.md, ntt.md, extensions.md are present and complete
- [ ] `kuro/specs/`: tower construction, Karatsuba
- [ ] `jali/specs/`: ring arithmetic, NTT, noise budget
- [ ] `trop/specs/`: semiring axioms, dual certificate, Kleene star
- [ ] `genies/specs/`: CSIDH-512 prime, isogeny walk, constant-time requirements

Each specs/ needs a README.md decision record (format: hemera/specs/README.md).

---

## Phase 3: critical bugs (release-blocking)

### Barrett reduction escape bug
**File:** `genies/rs/src/fq.rs` ~line 187

`count > 5` escape exits with an unreduced value. Must fix before release.
- Add `debug_assert!(count <= 3)` inside loop
- Add proptest: `Fq::mul(&a, &b).limbs < PRIME` for 1000 random pairs

### action() constant-time
**File:** `genies/rs/src/action.rs`

`while e != 0` and `if e > 0 { e -= 1 } else { e += 1 }` are secret-dependent
branches. CSIDH requires constant-time exponent processing.

Options:
1. Implement dummy isogenies (constant iterations per prime regardless of exponent)
2. Explicitly document `action()` is public-exponent-only; create `action_ct()` stub

Decision must be made and documented in `genies/specs/` before release.

### Zeroize secret key
**File:** `genies/rs/src/action.rs` + `genies/rs/Cargo.toml`

Add `zeroize = "1"` and `#[derive(Zeroize, ZeroizeOnDrop)]` on `Ideal`.

---

## Phase 4: twelve quality passes

Run at release tier (all 12). Key items not covered above:

**Pass 3 — arithmetic correctness:**
- [ ] `nebu`: `add(P-1, P-1) == P-2` test; `reduce128` boundary test
- [ ] `genies`: `Fq::legendre()` const test: `2 * PRIME_MINUS_1_HALF + 1 == PRIME`
- [ ] `jali`: NTT roundtrip `intt(ntt(v)) == v` parameterized over all power-of-2 sizes

**Pass 4 — crypto hygiene:**
- [ ] `grep -r "sha2\|sha3\|blake2\|blake3\|md5" .` returns nothing
- [ ] No secret-dependent branches except genies (resolved in Phase 3)
- [ ] `genies` `neg()` branches on `is_zero()` — audit all callers for secret values

**Pass 12 — testability:**
- [ ] All five algebras call axiom macros from strata-core testing module
- [ ] `vectors/strata.json` loaded and asserted in `tests/vectors.rs` per algebra
- [ ] `batch_inv()` consistency proptest: result matches element-wise `inv()` for nebu, genies

---

## Phase 5: publishing

### Publishing order

```bash
# Layer 1
cargo publish -p strata-core

# Layer 2
cargo publish -p strata-proof
cargo publish -p strata-compute
cargo publish -p strata-ext

# Layer 3 (strata-jali depends on nebu; publish it last)
cargo publish -p strata-trop
cargo publish -p strata-nebu
cargo publish -p strata-kuro
cargo publish -p strata-genies
cargo publish -p strata-jali

# Layer 4
cargo publish -p strata
```

WGSL/CLI crates: publish alongside their algebra (strata-nebu-wgsl, strata-nebu-cli, etc.)
after confirming they compile cleanly and docs build.

### Pre-publish check per crate
```bash
cargo publish --dry-run -p <crate>
cargo package --list -p <crate>   # verify no stray path refs
cargo doc -p <crate> --no-deps    # verify doc build
```

---

## Phase 6: post-release

- [ ] Verify crates.io pages and docs.rs builds
- [ ] `git tag -a v0.1.0 -m "strata 0.1.0" && git push origin v0.1.0`
- [ ] Update lens Cargo.toml: add `[patch.crates-io]` for local dev, change
  workspace deps to version deps only
- [ ] Open roadmap issues for genies constant-time (action_ct)

---

## critical file table

| file | issue | priority |
|------|-------|----------|
| `genies/rs/src/fq.rs` ~187 | Barrett escape = unreduced value | release-blocking |
| `genies/rs/src/action.rs` | secret-dependent branches | release-blocking |
| `Cargo.toml` | missing lints + profiles | Phase 1 |
| `.github/workflows/ci.yml` | missing entirely | Phase 1 |
| `bench/` | missing crate | Phase 1 |
| `vectors/strata.json` | missing cross-impl anchors | Phase 1 |
| `docs/` | missing Diataxis tree | Phase 1 |
| `roadmap/` | missing proposals | Phase 1 |
| `CLAUDE.md` | missing workspace-level | Phase 1 |
| `CHANGELOG.md` | missing | Phase 1 |
