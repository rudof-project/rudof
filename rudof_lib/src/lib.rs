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

pub(crate) mod api;

pub mod display;

pub mod errors;

pub mod formats;

mod rudof;

pub mod types;

pub(crate) mod utils;

pub mod config;

pub use crate::config::RudofConfig;
pub use rudof::*;

// Reexport config trait and error
pub use rudof_config::{ConfigError, TomlConfig};
