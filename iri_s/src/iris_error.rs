// use oxiri::Iri;
// use oxrdf::IriParseError;
use thiserror::Error;

use crate::IriS;

#[derive(Error, Debug)]
pub enum IriSError {
    #[error("Error parsing {str} as IRI: {err}")]
    IriParseError { str: String, err: String },

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
            IriSError::IriParseError { str, err } => Self::IriParseError {
                str: str.clone(),
                err: err.clone(),
            },
            IriSError::IriResolveError { err, base, other } => IriSError::IriResolveError {
                err: err.clone(),
                base: base.clone(),
                other: other.clone(),
            },
        }
    }
}
