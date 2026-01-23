//! Prefix map implementation
//!
//! Implements prefix maps, which are used in TURTLE, SPARQL and ShEx
//!
//! A prefix map is a list of alias declarations associated with IRIs:
//!
//! ```turtle
//! prefix schema: <https://schema.org/>
//! prefix :       <https://example.org/>
//! ```
//!
//! Example
//!
//! ```
//! # use std::str::FromStr;
//! # use iri_s::{IriS, IriSError};
//! # use prefixmap::PrefixMap;
//!
//! # fn main() -> Result<(), IriSError> {
//! let schema_iri  = IriS::from_str("https://schema.org/")?;
//! let example_iri = IriS::from_str("https://example.org/")?;
//! let mut pm = PrefixMap::new();
//! pm.add_prefix("schema", schema_iri);
//! pm.add_prefix("", example_iri);
//! # Ok(())
//! # }
//! ```

mod test;
pub mod iri;
pub mod error;
pub mod map;

pub use crate::error::*;
pub use crate::iri::{Deref, DerefError};
pub use crate::iri::{IriRef, IriRefError};
pub use crate::map::PrefixMap;
