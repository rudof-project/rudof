use thiserror::Error;
use crate::error::SrdfError;

#[derive(Debug, Error)]
pub enum ReportError {

    #[error("Obtaining objects for subject {subject} with predicate {predicate}: {error}")]
    ObjectsFor {
        subject: String,
        predicate: String,
        error: String
    },

    #[error("Error parsing the Validation Report: {error}")]
    Srdf {
        #[from]
        error: SrdfError,
    },

    #[error(transparent)]
    Result (#[from] ResultError),

    #[error("Error generating Validation Report: {msg}")]
    ValidationError {
        msg: String
    },

}

#[derive(Debug, Error)]
pub enum ResultError {
    #[error("Obtaining path for subject {subject}: {error}")]
    PathFor {
        subject: String,
        error: String,
    },

    #[error("Obtaining objects for subject {subject} with predicate {predicate}: {error}")]
    ObjectFor {
        subject: String,
        predicate: String,
        error: String,
    },

    #[error("Error parsing the Validation Result, the {field} field is missing")]
    MissingRequiredField {
        field: String
    },

    #[error("Error parsing the Validation Result: {err}")]
    Srdf {
        err: SrdfError,
    },

    #[error("Error parsing the Validation Result, the field '{field}' has an invalid IRI value: {value}")]
    WrongIriForSeverity {
        field: String,
        value: String,
    },

    #[error("Error parsing the Validation Result, the field '{field}' has an invalid IRI value: {value}")]
    WrongNodeForSeverity {
        field: String,
        value: String
    },
}