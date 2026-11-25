//! IRI simple wrapper
//!
//! This module contains a simple wrapper to work with IRIs
//! The main goal is that we can use a simple interface to work with IRIs
//! which could be adapted to different implementations in the future if needed.
//!
//! The library provides the macro [`iri`] to create [`IriS`] from strings.
//!
// getrandom 0.4 and 0.3 are dependencies only to turn on their browser
// backend on wasm (see Cargo.toml); nothing calls them directly.
#[cfg(target_family = "wasm")]
use getrandom as _;
#[cfg(target_family = "wasm")]
use getrandom03 as _;

pub mod error;
mod iri;
mod mime_type;
pub mod registry;

#[cfg(target_family = "wasm")]
pub use crate::iri::dereferencer::{Dereferencer, set_dereferencer};
pub use crate::iri::{Iri, IriS};
pub use crate::mime_type::MimeType;
