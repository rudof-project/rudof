use std::io;
use oxrdf::TryFromTermError;
use thiserror::Error;
use shacl::error::{IRError, ShaclParserError};
use shacl::validation::ReportError;
use sparql_service::RdfDataError;

#[derive(Debug, Error)]
pub(crate) enum TestSuiteError {
    #[error("Error compiling shapes: {0}")]
    TestShapesCompilation(String),

    #[error(transparent)]
    ReportParsing(#[from] ReportError),

    #[error(transparent)]
    InputOutput(#[from] io::Error),

    #[error(transparent)]
    RdfData(#[from] RdfDataError),

    // TODO - Maybe remove TestShapesCompilation variant?
    #[error(transparent)]
    CompilingShapes(#[from] IRError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    ParsingShape(#[from] ShaclParserError),

    #[error("The actual and expected ValidationReports are not equals")]
    NotEquals,

    #[error(transparent)]
    TryFromTerm(#[from] TryFromTermError),
}