//! Implementation of [`SRDF`] traits based on SPARQL endpoints
//!
//! This crate implements the [`SRDF`] traits using a SPARQL endpoint to obtain the RDF data
pub mod oxsparql;
pub mod oxsparql_error;

pub use crate::error::*;
pub use crate::oxsparql::*;
