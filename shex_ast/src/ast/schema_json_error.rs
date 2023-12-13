use std::io;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SchemaJsonError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError {
        path_name: String,
        error: String, // We need to clone so we use String instead of io::Error
    },

    #[error("Reading JSON from {path_name:?}. Error: {error:?}")]
    JsonError {
        path_name: String,
        error: String, // We need to clone errors so we use String instead of serde_json::Error,
    },
}
