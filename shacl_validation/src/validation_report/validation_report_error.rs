use thiserror::Error;

use crate::helpers::helper_error::SRDFError;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Obtaining objects for subject {subject} with predicate {predicate}: {error}")]
    ObjectsFor {
        subject: String,
        predicate: String,
        error: String,
    },
    #[error("Error parsing the ValidationReport, {}", _0)]
    Srdf(#[from] SRDFError),

    #[error(transparent)]
    Result(#[from] ResultError),

    #[error("Error generating ValidationReport: {msg}")]
    ValidationError { msg: String },
}

#[derive(Error, Debug)]
pub enum ResultError {
    #[error("Obtaining path for subject {subject}: {error}")]
    PathFor {
        subject: String,
        error: String,
        path: String,
    },
    #[error("Obtaining objects for subject {subject} with predicate {predicate}: {error}")]
    ObjectFor {
        subject: String,
        predicate: String,
        error: String,
    },
    #[error("Error parsing the ValidationResult, the {} field is missing", _0)]
    MissingRequiredField(String),

    #[error("Error parsing the ValidationResult, {}", _0)]
    Srdf(#[from] SRDFError),

    #[error(
        "Error parsing the ValidationResult, the field '{}' has an invalid IRI value: '{}'",
        field,
        value
    )]
    WrongIRIForSeverity { field: String, value: String },

    #[error(
        "Error parsing the ValidationResult, the field '{}' has an invalid IRI value: '{}'",
        field,
        value
    )]
    WrongNodeForSeverity { field: String, value: String },
}
