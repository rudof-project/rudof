//! The `api` module provides operation-specific submodules that constitute the Rudof builder pattern API.
//!
//! Each submodule contains traits and builders for a specific domain (like RDF data, SHACL, ShEx, SPARQL queries),
//! separating concerns while keeping the main [`crate::Rudof`] struct cohesive.

pub mod comparison;
pub mod conversion;
pub mod core;
pub mod data;
pub mod dctap;
pub mod generation;
pub mod pgschema;
pub mod query;
pub mod rdf_config;
pub mod shacl;
pub mod shex;
