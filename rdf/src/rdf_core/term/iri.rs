use std::{hash::Hash, fmt::{Debug, Display}};

/// Represents an IRI (Internationalized Resource Identifier) in RDF.
pub trait Iri: Debug + Display + Hash + Eq + Ord + Clone {
    /// Returns the IRI as a string slice.
    fn as_str(&self) -> &str;
}
