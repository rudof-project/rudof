use thiserror::Error;

use crate::helpers::helper_error::SRDFError;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Error parsing the ValidationReport, {}", _0)]
    Srdf(#[from] SRDFError),

    #[error(transparent)]
    Result(#[from] ResultError),

    #[error("Error generating ValidationReport: {msg}")]
    ValidationReportError { msg: String },
}

#[derive(Error, Debug)]
pub enum ResultError {
    #[error("Error parsing the ValidationResult, the {} field is missing", _0)]
    MissingRequiredField(String),

    #[error("Error parsing the ValidationResult, {}", _0)]
    Srdf(#[from] SRDFError),
}
