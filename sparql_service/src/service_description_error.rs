use rdf::{rdf_core::RDFError, rdf_impl::InMemoryGraphError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceDescriptionError {
    #[error(transparent)]
    RDFParseError {
        #[from]
        error: RDFError,
    },

    #[error("Expected IRI as value for property: {property} but got {term}")]
    ExpectedIRIAsValueForProperty { property: String, term: String },

    #[error(transparent)]
    SRDFGraphError {
        #[from]
        error: InMemoryGraphError,
    },
}
