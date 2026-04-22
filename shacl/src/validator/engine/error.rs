use rudof_rdf::rdf_core::RDFError;
use rudof_rdf::rdf_impl::InMemoryGraphError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SparqlError {
    #[error("Query could not be performed")]
    Query { query: String, err: String },
}

#[derive(Debug, Error)]
pub enum SrdfError {
    #[error("Error during the SRDF operation: {err}")]
    Srdf { err: String },

    #[error("Error during the creation of the SRDFGraph: {err}")]
    SrdfGraph {
        #[from]
        err: InMemoryGraphError,
    },

    #[error("RDFError: {err}")]
    RdfError {
        #[from]
        err: RDFError,
    },

    #[error("Converting term {subj} to subject")]
    SrdfTermAsSubject { subj: String },

    #[error("Error finding values for subject {subject} with predicate {predicate}: {err}")]
    ObjectsWithSubjectPredicate {
        subject: String,
        predicate: String,
        err: String,
    },

    #[error("Error finding values for object {object} with predicate {predicate}: {err}")]
    SubjectsWithPredicateObject {
        object: String,
        predicate: String,
        err: String,
    },

    #[error("Unexpected literal {lit} as a SHACL path")]
    ShaclUnexpectedLiteral { lit: String },
}
