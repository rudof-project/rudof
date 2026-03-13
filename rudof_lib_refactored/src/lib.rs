//! # rudof_lib
//!
//! This library provides a single, stable entry point for working with RDF data,
//! ShEx and SHACL schemas, SPARQL queries, and related semantic web technologies.
//! 
//! It acts as a facade over multiple specialized crates.

pub(crate) mod api;
pub mod errors;
pub mod formats;
mod rudof_config;
mod rudof;
pub mod types;
pub(crate) mod utils;

pub use rudof::*;
pub use rudof_config::RudofConfig;

// ============================================================================
// Re-exported types from underlying crates
// ============================================================================
// These are re-exported rather than wrapped to maintain full API access and avoid unnecessary indirection. 
// Errors are handled through our hierarchical error system in the `errors` module.



