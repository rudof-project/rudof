use std::{io, result};

use thiserror::Error;

pub type Result<T> = result::Result<T, ConverterError>;

#[derive(Error, Debug)]
pub enum ConverterError {
    #[error("Error reading config file from path {path}: {error}")]
    ConverterConfigFromPathError { path: String, error: io::Error },

    #[error("Error reading config file from path {path}: {error}")]
    ConverterConfigFromYAMLError {
        path: String,
        error: serde_yml::Error,
    },
}
