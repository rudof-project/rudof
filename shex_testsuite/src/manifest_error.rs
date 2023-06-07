use std::{ffi::OsStr, io, path::Path};

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

    #[error("not found entry: {name:?}")]
    NotFoundEntry { name: String },

    #[error("Turtle error: {path_name:?}. Error: {turtle_err:?}")]
    ErrorReadingTurtle {
        path_name: String,
        turtle_err: String,
    },

    #[error("Unknown error")]
    Unknown,
}
