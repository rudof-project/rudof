use oxrdf::TryFromTermError;
use shacl::error::{IRError, ShaclParserError};
use sparql_service::RdfDataError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TestSuiteError {
    #[error("Error compiling shapes: {0}")]
    TestShapesCompilation(String),

    #[error(transparent)]
    InputOutput(#[from] io::Error),

    #[error(transparent)]
    RdfData(#[from] Box<RdfDataError>),

    // TODO - Maybe remove TestShapesCompilation variant?
    #[error(transparent)]
    CompilingShapes(#[from] Box<IRError>),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    ParsingShape(#[from] Box<ShaclParserError>),

    #[error("The actual and expected ValidationReports are not equals")]
    NotEquals,

    #[error(transparent)]
    TryFromTerm(#[from] Box<TryFromTermError>),
}

impl From<RdfDataError> for TestSuiteError {
    fn from(value: RdfDataError) -> Self {
        Self::RdfData(Box::new(value))
    }
}

impl From<IRError> for TestSuiteError {
    fn from(value: IRError) -> Self {
        Self::CompilingShapes(Box::new(value))
    }
}

impl From<ShaclParserError> for TestSuiteError {
    fn from(value: ShaclParserError) -> Self {
        Self::ParsingShape(Box::new(value))
    }
}

impl From<TryFromTermError> for TestSuiteError {
    fn from(value: TryFromTermError) -> Self {
        Self::TryFromTerm(Box::new(value))
    }
}
