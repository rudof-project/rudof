use iri_s::error::IriSError;
use shex_ast::compact::ParseError;
use shex_ast::shapemap::ValidationStatus;
use shex_ast::{Schema, SchemaIRError, ast::SchemaJsonError};
use shex_validation::ValidatorError;
use rdf::{rdf_impl::InMemoryGraphError, rdf_core::RDFError};
use std::{ffi::OsString, io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Error converting path {path} to file URL")]
    FromFilePath { path: PathBuf },

    #[error("Error parsing ShExC for entry {entry_name} from file {shex_path}. Error: {error:?}")]
    ShExCParsingError {
        error: Box<ParseError>,
        entry_name: String,
        shex_path: Box<PathBuf>,
    },
    #[error("Reading Manifest Map from {map:?} for entry {entry:?}. Error: {error}")]
    ReadingShapeMap { entry: String, map: PathBuf, error: String },

    #[error("Parsing ShapeLabel: {value}. Error: {error:?}")]
    ParsingShapeLabel { value: String, error: String },

    #[error("Reading manifest map for entry {entry:?}. Error: {error}")]
    ParsingManifestMap { entry: String, error: String },

    #[error("Parsing focus node: {value}. Error: {error:?}")]
    ParsingFocusNode { value: String, error: Box<RDFError> },

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
        error: Box<SchemaJsonError>,
        entry_name: String,
    },

    #[error("not found entry: {name:?}")]
    NotFoundEntry { name: String },

    #[error("No focus node in validation action: {entry}")]
    NoFocusNode { entry: String },

    #[error("Unknown error")]
    Unknown,

    #[error(transparent)]
    SRDFError(#[from] InMemoryGraphError),

    #[error(transparent)]
    ParseError(#[from] ParseError),

    #[error(transparent)]
    SchemaIRError(#[from] Box<SchemaIRError>),

    #[error(transparent)]
    IriError(#[from] IriSError),

    #[error(transparent)]
    ValidationError(#[from] ValidatorError),

    #[error("Parsing validation type: Unknown value: {value}")]
    ParsingValidationType { value: String },

    #[error(
        "Expected Failure for entry {entry} but obtained passed status [{}]\nFailure status: [{}]",
       passed_status.iter().map(|s| s.code()).collect::<Vec<_>>().join(", "),
       failed_status.iter().map(|s| s.code()).collect::<Vec<_>>().join(", "))]
    ExpectedFailureButObtained {
        failed_status: Vec<ValidationStatus>,
        passed_status: Vec<ValidationStatus>,
        entry: String,
    },

    #[error("Expected OK for {entry} but failed. Failed status are: [{}]\nPassed status are: [{}]",
       failed_status.iter().map(|s| s.code()).collect::<Vec<_>>().join(", "),
       passed_status.iter().map(|s| s.code()).collect::<Vec<_>>().join(", "))]
    ExpectedOkButObtained {
        failed_status: Vec<ValidationStatus>,
        passed_status: Vec<ValidationStatus>,
        entry: String,
    },

    #[error(
        "Schema parsed is different to schema serialized after parsing\nSchema parsed from JSON\n{schema_parsed:?}\nSchema serialized after parsing:\n{schema_parsed_after_serialization:?}\nSchema serialized: {schema_serialized}\nSchema serialized after: {schema_serialized_after}"
    )]
    SchemasDifferent {
        schema_parsed: Box<Schema>,
        schema_serialized: String,
        schema_parsed_after_serialization: Box<Schema>,
        schema_serialized_after: String,
    },

    #[error(
        "Error |ShExSchemaDifferent| ShEx Schema after serialization is different from schema parsed\nSchema JSON Parsed:\n{json_schema_parsed:?}\nSchema parsed from ShExC:\n{shexc_schema_parsed:?}\nInput schema serialized: {schema_serialized}"
    )]
    ShExSchemaDifferent {
        json_schema_parsed: Box<Schema>,
        schema_serialized: String,
        shexc_schema_parsed: Box<Schema>,
    },

    #[error("Schema parsed could not be serialized\n{schema_parsed:?}\n{error:?}")]
    SchemaSerializationError {
        schema_parsed: Box<Schema>,
        error: serde_json::Error,
    },

    #[error("Schema parsed after serialization could not be serialized\n{schema_parsed:?}\n{error:?}")]
    SchemaSerializationError2nd {
        schema_parsed: Box<Schema>,
        error: serde_json::Error,
    },

    #[error(
        "Parsing schema serialized with name: {schema_name}\nSchema serialized:\n{schema_serialized}\nError: {error}"
    )]
    SchemaParsingAfterSerialization {
        schema_name: String,
        schema_parsed: Box<Schema>,
        schema_serialized: String,
        error: serde_json::Error,
    },

    #[error("Error converting ShapeExprLabel to ShapeLabel for entry {entry}. Error: {error}")]
    IriRefError { error: String, entry: String },

    #[error("Unable to perform operation in WASM: {0}")]
    WASMError(String),
}
