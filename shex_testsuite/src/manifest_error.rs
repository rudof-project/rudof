use iri_s::IriSError;
use shex_ast::{ast::SchemaJsonError, CompiledSchemaError, Schema};
use shex_compact::ParseError;
use shex_validation::{ResultValue, ValidatorError};
use srdf::srdf_graph::SRDFGraphError;
use std::{ffi::OsString, io, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Obtaining absolute path for {base:?}: {error:?}")]
    AbsolutePathError { base: OsString, error: io::Error },

    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error("Base path {base:?} can't be converted to Url")]
    BasePathError { base: OsString },

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
    ParseError(#[from] ParseError),

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

    #[error("Schema parsed is different to schema serialized after parsing\nSchema parsed from JSON\n{schema_parsed:?}\nSchema serialized after parsing:\n{schema_parsed_after_serialization:?}\nSchema serialized: {schema_serialized}\nSchema serialized after: {schema_serialized_after}")]
    SchemasDifferent {
        schema_parsed: Schema,
        schema_serialized: String,
        schema_parsed_after_serialization: Schema,
        schema_serialized_after: String
    },

    #[error(
        "ShEx Schema after serialization is different from schema parsed\nSchema JSON Parsed:\n{json_schema_parsed:?}\nShEx schema parsed:\n{shex_schema_parsed:?}\nInput schema serialized: {schema_serialized}"
    )]
    ShExSchemaDifferent {
        json_schema_parsed: Schema,
        schema_serialized: String, 
        shex_schema_parsed: Schema,
    },

    #[error("Schema parsed could not be serialized\n{schema_parsed:?}\n{error:?}")]
    SchemaSerializationError {
        schema_parsed: Schema,
        error: serde_json::Error,
    },

    #[error("Schema parsed after serialization could not be serialized\n{schema_parsed:?}\n{error:?}")]
    SchemaSerializationError2nd {
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
