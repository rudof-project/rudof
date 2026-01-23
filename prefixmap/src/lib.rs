//! Prefix map implementation
//!
//! Implements prefix maps, which are used in Turtle and ShEx
//!
//! A prefix map is a list of alias declarations associated with IRIs
//!
//! ```turtle
//! prefix schema: <http://schema.org/>
//! prefix :       <http://example.org/>
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
//! let schema_iri  = IriS::from_str("http://schema.org/")?;
//! let example_iri = IriS::from_str("http://example.org/")?;
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
