use srdf::{RdfParseError, SRDFGraphError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceDescriptionError {
    #[error(transparent)]
    RDFParseError {
        #[from]
        error: RdfParseError,
    },

    #[error("Expected IRI as value for property: {property} but got {term}")]
    ExpectedIRIAsValueForProperty { property: String, term: String },

    #[error(transparent)]
    SRDFGraphError {
        #[from]
        error: SRDFGraphError,
    },
}
