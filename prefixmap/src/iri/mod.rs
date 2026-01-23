// ```
// let mut pm = PrefixMap::new();
// let binding = ;
// pm.insert("schema", &IriS::from_str("http://schema.org/"))
// pm.insert("", &IriS::from_str("http://example.org/")?);
// ```
pub mod iri_ref;
pub mod deref;
mod visitor;

pub use deref::{Deref, DerefError};
pub use iri_ref::{IriRef, IriRefError};
