use std::io;
use shex_ast::SchemaJsonError;
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

    #[error("Unknown error")]
    Unknown,

    #[error(transparent)]
    SRDFError(#[from] SRDFGraphError)
}
