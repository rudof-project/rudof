//! IRI simple wrapper
//!
//! This module contains a simple wrapper to work with IRIs
//! The main goal is that we can use a simple interface to work with IRIs
//! which could be adapted to different implementations in the future if needed.
//!
//! The library provides the macro [`iri`] to create IRIs from strings.
//!
pub mod error;
pub mod iri;
pub mod mime_type;

pub use crate::error::*;
pub use crate::iri::IriS;
pub use crate::mime_type::*;
