//! Implementation of [`SRDF`] traits based on SPARQL endpoints
//!
//! This crate implements the [`SRDF`] traits using a SPARQL endpoint to obtain the RDF data
pub mod srdf_sparql_error;
pub mod srdfsparql;

pub use crate::srdf_sparql_error::*;
pub use crate::srdfsparql::*;
