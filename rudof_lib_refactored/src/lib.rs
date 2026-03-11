//! # rudof_lib
//!
//! Public API for RDF data validation using ShEx and SHACL.
//!
//! This library provides a facade over the internal Rudof implementation,
//! exposing a curated, stable API that shields users from internal changes.
//!
//! ## Structure
//!
//! - `formats`: All format enums 
//! - `types`: All domain types 
//! - `errors`: Error handling 
//! - `Rudof`: Main entry point
//! ```

pub(crate) mod api;
pub mod errors;
pub mod formats;
mod rudof;
pub mod types;
pub(crate) mod utils;

pub use rudof::*;