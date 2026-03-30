use oxrdf::TryFromTermError;
use shacl::error::{IRError, ShaclParserError};
use shacl_validation::validation_report::validation_report_error::ReportError;
use sparql_service::RdfDataError;
use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestSuiteError {
    #[error("Error compiling shapes: {error}")]
    TestShapesCompilation { error: String },

    #[error(transparent)]
    ReportParsing(#[from] ReportError),

    #[error(transparent)]
    InputOutput(#[from] Error),

    #[error(transparent)]
    RdfData(#[from] RdfDataError),

    #[error(transparent)]
    CompilingShapes(#[from] IRError),

    #[error("Validation error: {error}")]
    Validation { error: String },

    #[error(transparent)]
    ParsingShape(#[from] ShaclParserError),

    #[error("The actual and expected ValidationReports are not equals")]
    NotEquals,

    #[error(transparent)]
    TryFromTerm(#[from] TryFromTermError),
}
