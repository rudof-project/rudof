use std::path::PathBuf;

use iri_s::{IriS, IriSError};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SchemaJsonError {
    #[error("Node {node} is not a valid RDF Node in an ObjectValue: {error}")]
    InvalidNodeInObjectValue {
        node: String, // We need to clone so we use String instead of Object
        error: String,
    },
    #[error("Error checking literal: {error}")]
    LiteralError { error: String },

    #[error("Error parsing label as IriRef, label: {label}: {error}")]
    InvalidIriRef {
        label: String,
        error: String, // We need to clone errors so we use String instead of IriSError
    },
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

    #[error("Reading JSON from reader. Error: {error:?}")]
    JsonErrorFromReader {
        error: String, // We need to clone errors so we use String instead of serde_json::Error,
    },

    #[error("Shape Decl with prefixed shape {prefix:}:{local} but no prefix map declaration")]
    ShapeDeclPrefixNoPrefixMap { prefix: String, local: String },

    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: prefixmap::PrefixMapError,
    },

    #[error("Obtaining current dir: {error:?}")]
    CurrentDir { error: String },

    #[error("Obtaining Url from local dir: {path}")]
    LocalFolderIriError { path: PathBuf },

    #[error("Trying to dereference IRI: {iri}: {error}")]
    DereferencingIri { iri: IriS, error: IriSError },

    #[error("Obtaining schema from IRI {iri}. Error: {error}")]
    SchemaFromIri { iri: IriS, error: String },
}
