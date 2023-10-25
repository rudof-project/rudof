use std::io;
use iri_s::IriSError;
use shex_ast::DerefError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Nom Parsing error: {err:?}")]
    NomError { err: nom::error::ErrorKind },

    #[error(transparent)]
    IOError { #[from] err: io::Error },

    #[error("{msg}")]
    Custom { msg: String },

    #[error(transparent)]
    IRISError { #[from] err: IriSError },

    #[error(transparent)]
    DerefError { #[from] err: DerefError },

}
