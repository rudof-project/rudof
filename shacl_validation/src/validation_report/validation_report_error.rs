use thiserror::Error;

use crate::helper::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ValidationReportError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("Error during the creation of the Validation Result")]
    ValidationResult(#[from] ValidationResultError),
}

#[derive(Error, Debug)]
pub enum ValidationResultError {}
