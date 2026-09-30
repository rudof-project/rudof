//! The `api` module provides operation-specific submodules that constitute the Rudof builder pattern API.
//!
//! Each submodule contains traits and builders for a specific domain (like RDF data, SHACL, ShEx, SPARQL queries),
//! separating concerns while keeping the main [`crate::Rudof`] struct cohesive.

#[cfg(feature = "comparison")]
pub mod comparison;
#[cfg(feature = "conversion")]
pub mod conversion;
pub mod core;
pub mod data;
#[cfg(feature = "dctap")]
pub mod dctap;
// Data generation and property graph databases rely on native-only crates
// (`rudof_generate`, `lbug`).
#[cfg(not(target_family = "wasm"))]
pub mod generation;
pub mod map_state;
pub mod materialize;
// The property graph database needs the `pg-db` feature.
#[cfg(all(feature = "pg-db", not(target_family = "wasm")))]
pub mod pg_db;
#[cfg(feature = "pgschema")]
pub mod pgschema;
pub mod prefixes;
pub mod query;
#[cfg(feature = "rdf-config")]
pub mod rdf_config;
pub mod shacl;
pub mod shex;
pub mod shexmap;
