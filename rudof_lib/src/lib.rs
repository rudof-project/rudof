//! # rudof_lib
//!
//! `rudof_lib` provides a single, stable entry point for working with RDF data,
//! ShEx and SHACL schemas, SPARQL queries, and related semantic web technologies.
//!
//! It acts as a facade over multiple specialized crates, orchestrating operations
//! across the broader rudof ecosystem.
//!
//! ## Overview
//!
//! The main entry point to this library is the [`Rudof`] struct, which maintains
//! context across multiple semantic operations.
//!
//! Configuration is handled via the [`RudofConfig`] struct, which offers
//! fine-grained settings across RDF formats, SHACL validation, schema conversion,
//! and more.

#[cfg(not(target_family = "wasm"))]
pub(crate) mod api;

#[cfg(not(target_family = "wasm"))]
pub mod display;

#[cfg(not(target_family = "wasm"))]
pub mod errors;

#[cfg(not(target_family = "wasm"))]
pub mod formats;

#[cfg(not(target_family = "wasm"))]
mod rudof;

#[cfg(not(target_family = "wasm"))]
mod rudof_config;

#[cfg(not(target_family = "wasm"))]
pub mod types;

#[cfg(not(target_family = "wasm"))]
pub(crate) mod utils;

#[cfg(not(target_family = "wasm"))]
pub use rudof::*;
#[cfg(not(target_family = "wasm"))]
pub use rudof_config::RudofConfig;
