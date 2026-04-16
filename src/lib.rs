//! strata — five algebraic backends for verifiable computation.
//!
//! re-exports the four-tier trait hierarchy and all five algebras.
//!
//! ## tiers
//!
//! - tier 1 (`strata-core`): Codec, Semiring, Ring, Field
//! - tier 2 (`strata-proof`): Reduce, Dot
//! - tier 3 (`strata-compute`): Spectral, Bits
//! - tier 4 (`strata-ext`): Extension, Batch, Blind
//!
//! ## algebras
//!
//! - `nebu`: Goldilocks F_p (all tiers)
//! - `kuro`: F₂ tower (tiers 1, 2, 4)
//! - `jali`: polynomial ring R_q (tier 1)
//! - `trop`: tropical semiring (tier 1 — Semiring only)
//! - `genies`: isogeny curves F_q (tiers 1, 2, 4)

// tier 1: universal
pub use strata_core::*;

// tier 2: proof system
pub use strata_proof::{Dot, Reduce};

// tier 3: computation
pub use strata_compute::{Bits, Spectral};

// tier 4: structure
pub use strata_ext::{Batch, Blind, Extension};

// algebras
pub use genies;
pub use jali;
pub use kuro;
pub use nebu;
pub use trop;
