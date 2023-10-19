use std::{fs, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Nom Parsing error: {err:?}")]
    NomError { err: nom::error::ErrorKind },

    #[error("IO Error {err}")]
    IOError { #[from] err: io::Error },

    #[error("{msg}")]
    Custom { msg: String },
}
