use srdf::SRDFGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPARQLError {
    #[error("Query could not be performed")]
    Query { query: String, error: String },
}

#[derive(Error, Debug)]
pub enum SRDFError {
    #[error("Error during the SRDF operation: {error}")]
    Srdf { error: String },

    #[error("Error during the creation of the SRDFGraph: {0}")]
    SRDFGraph(#[from] SRDFGraphError),

    #[error("RDFError: {0}")]
    RDFError(#[from] RDFError),

    #[error("Error during the creation of the SRDFGraph")]
    SRDFGraph(#[from] SRDFGraphError),

    #[error("Converting term {subject} to subject")]
    SRDFTermAsSubject { subject: String },

    #[error("Error finding values for subject {subject} with predicate {predicate}: {error}")]
    ObjectsWithSubjectPredicate {
        subject: String,
        predicate: String,
        error: String,
    },

    #[error("Error finding values for object {object} with predicate {predicate}: {error}")]
    SubjectsWithPredicateObject {
        object: String,
        predicate: String,
        error: String,
    },

    #[error("Unexpected literal {lit} as a SHACL path")]
    SHACLUnexpectedLiteral { lit: String },
}
