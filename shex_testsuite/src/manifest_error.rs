use std::io;
use iri_s::IriSError;
use shex_ast::{SchemaJsonError, CompiledSchemaError};
use shex_validation::{ValidatorError, ResultValue};
use srdf_graph::SRDFGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error("Reading JSON from {path_name:?}. Error: {error:?}")]
    JsonError {
        path_name: String,
        error: serde_json::Error,
    },

    #[error("Error parsing Schema as Json: errror {error:?}, entry: {entry_name:?}")]
    SchemaJsonError {
        error: SchemaJsonError, 
        entry_name: String
    },

    #[error("not found entry: {name:?}")]
    NotFoundEntry { name: String },

    #[error("No focus node in validation action: {entry}")]
    NoFocusNode { entry: String },

    #[error("Unknown error")]
    Unknown,

    #[error(transparent)]
    SRDFError(#[from] SRDFGraphError),

    #[error(transparent)]
    CompiledSchemaError(#[from] CompiledSchemaError),

    #[error(transparent)]
    IriError(#[from] IriSError),

    #[error(transparent)]
    ValidationError(#[from] ValidatorError),

    #[error("Parsing validation type: Unknown value: {value}")]
    ParsingValidationType{ value: String },

    #[error("Expected faiure but obtained {value} for {entry}")]
    ExpectedFailureButObtained { value: ResultValue, entry: String },

    #[error("Expected OK but obtained {value} for {entry}")]
    ExpectedOkButObtained { value: ResultValue, entry: String }

}
