use rudof_rdf::rdf_core::RDFError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConstraintError {
    #[error("Constraint not yet implemented: {err}")]
    NotImplemented { err: String },

    #[error("Query error: {err}")]
    Query { err: String },

    #[error("Expected IRI but found {term}")]
    ExpectedIri { term: String },

    #[error(transparent)]
    Rdf(#[from] RDFError),

    #[error("Internal error: {err}")]
    Internal { err: String },
}
