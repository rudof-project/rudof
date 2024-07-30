use thiserror::Error;

use crate::helper::helper_error::{SPARQLError, SRDFError};

#[derive(Error, Debug)]
pub enum ValidationReportError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("Error during the creation of the Validation Result")]
    ValidationResult(#[from] ValidationResultError),
    #[error("Error related to SRDF")]
    SRDF(#[from] SRDFError),
    #[error("Error querying")]
    Query,
    #[error("Cannot parse Literal to Subject")]
    LiteralToSubject,
    #[error("Invalid kind of term")]
    InvalidTerm,
}

#[derive(Error, Debug)]
pub enum ValidationResultError {
    #[error("Error related to SRDF")]
    SRDF(#[from] SRDFError),
    #[error("Cannot parse Literal to Subject")]
    LiteralToSubject,
}
