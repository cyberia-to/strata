//! cyber-algebra — five algebraic backends for verifiable computation.
//!
//! re-exports the four-tier trait hierarchy and all five algebras.
//!
//! ## tiers
//!
//! - tier 1 (`cyb-algebra`): Encode, Semiring, Ring, Field
//! - tier 2 (`cyb-algebra-proof`): Hash2Field, InnerProduct
//! - tier 3 (`cyb-algebra-compute`): Spectral, Bits
//! - tier 4 (`cyb-algebra-ext`): Extension, Batch, ConstantTime
//!
//! ## algebras
//!
//! - `nebu`: Goldilocks F_p (all tiers)
//! - `kuro`: F₂ tower (tiers 1, 2, 4)
//! - `jali`: polynomial ring R_q (tier 1)
//! - `trop`: tropical semiring (tier 1 — Semiring only)
//! - `genies`: isogeny curves F_q (tiers 1, 2, 4)

// tier 1: universal
pub use cyb_algebra::*;

// tier 2: proof system
pub use cyb_algebra_proof::{InnerProduct, Hash2Field};

// tier 3: computation
pub use cyb_algebra_compute::{Bits, Spectral};

// tier 4: structure
pub use cyb_algebra_ext::{Batch, ConstantTime, Extension};

// algebras
pub use genies;
pub use jali;
pub use kuro;
pub use nebu;
pub use trop;
