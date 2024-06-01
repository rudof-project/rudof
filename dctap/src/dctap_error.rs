use thiserror::Error;

use crate::IriS;

#[derive(Error, Debug)]
pub enum IriSError {
    #[error("IRI parse error: {err}")]
    IriParseError { err: String },

    #[error("Error resolving IRI `{other}` with base IRI `{base}`: {err}")]
    IriResolveError {
        err: String,
        base: IriS,
        other: IriS,
    },
}

impl Clone for IriSError {
    fn clone(&self) -> Self {
        match self {
            IriSError::IriParseError { err } => Self::IriParseError { err: err.clone() },
            IriSError::IriResolveError { err, base, other } => IriSError::IriResolveError {
                err: err.clone(),
                base: base.clone(),
                other: other.clone(),
            },
        }
    }
}
