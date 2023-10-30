use iri_s::IriSError;
use shex_ast::{ast::SchemaJsonError, CompiledSchemaError, Schema};
use shex_validation::{ResultValue, ValidatorError};
use srdf_graph::SRDFGraphError;
use std::io;
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
        entry_name: String,
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
    ParsingValidationType { value: String },

    #[error("Expected faiure but obtained {value} for {entry}")]
    ExpectedFailureButObtained { value: ResultValue, entry: String },

    #[error("Expected OK but obtained {value} for {entry}")]
    ExpectedOkButObtained { value: ResultValue, entry: String },

    #[error("Schema parsed is different to schema serialized after parsing\n{schema_parsed:?}\n{schema_parsed_after_serialization:?}")]
    SchemasDifferent {
        schema_parsed: Schema,
        schema_parsed_after_serialization: Schema,
    },

    #[error(
        "ShEx Schema is different from schema parsed\n{json_schema_parsed:?}\n{shex_schema_parsed:?}"
    )]
    ShExSchemaDifferent {
        json_schema_parsed: Schema,
        shex_schema_parsed: Schema,
    },

    #[error("Schema parsed could not be serialized\n{schema_parsed:?}\n{error:?}")]
    SchemaSerializationError {
        schema_parsed: Schema,
        error: serde_json::Error,
    },

    #[error("Parsing schema serialized with name: {schema_name}\nSchema serialized:\n{schema_serialized}\nError: {error}")]
    SchemaParsingAfterSerialization {
        schema_name: String,
        schema_parsed: Schema,
        schema_serialized: String,
        error: serde_json::Error,
    },
}
