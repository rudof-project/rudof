//! Prefix map implementation
//!
//! Implements prefix maps, which are used in Turtle and ShEx. A prefix map is a
//! list of alias declarations associated with IRIs.
//!
//! ```turtle
//! prefix schema: <http://schema.org/>
//! prefix :       <http://example.org/>
//! ```
//!
//! ## Example
//!
//! ```
//! use std::str::FromStr;
//! use iri_s::IriS;
//! use prefixmap::PrefixMap;
//!
//! let schema_iri  = IriS::from_str("http://schema.org/").unwrap();
//! let example_iri = IriS::from_str("http://example.org/").unwrap();
//! let mut pm = PrefixMap::default();
//! pm.insert("schema", &schema_iri);
//! pm.insert("", &example_iri);
//! ```
pub use crate::prefixmap::*;
pub use alias::*;
pub use deref::*;
pub use error::*;
pub use iri_ref::*;

pub mod alias;
pub mod deref;
pub mod error;
pub mod iri_ref;
pub mod prefixmap;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use iri_s::IriS;

    use super::*;

    #[test]
    fn prefixmap_test() -> Result<(), PrefixMapError> {
        let mut pm = PrefixMap::default();
        let schema_iri = IriS::from_str("http://schema.org/")?;
        pm.insert("schema", &schema_iri).unwrap();
        let resolved = pm.resolve("schema:knows")?;
        let schema_knows = IriS::from_str("http://schema.org/knows")?;
        assert_eq!(resolved, schema_knows);
        Ok(())
    }
}
