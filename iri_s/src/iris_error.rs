// use oxiri::Iri;
// use oxrdf::IriParseError;
use thiserror::Error;

use crate::IriS;

#[derive(Error, Debug, Clone)]
pub enum IriSError {
    #[error("Error parsing {str} as IRI: {err}")]
    IriParseError { str: String, err: String },

    #[error("Error resolving IRI `{other}` with base IRI `{base}`: {err}")]
    IriResolveError {
        err: String,
        base: IriS,
        other: IriS,
    },

    #[error("Creating reqwest http client: {error}")]
    ReqwestClientCreation { error: String },

    #[error("Error parsing Iri as Url: {error}")]
    UrlParseError { error: String },

    #[error("Http request error: {error}")]
    ReqwestError { error: String },

    #[error("Http request error as String: {error}")]
    ReqwestTextError { error: String },
}
