use std::{ffi::OsStr, io, path::Path};

use shex_ast::SchemaJsonError;
use srdf_oxgraph::srdf_graph_error::SRDFError;
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

    #[error("SRDFError error: {error:?}")]
    SRDFError { error: SRDFError },

    #[error("Unknown error")]
    Unknown,
}
