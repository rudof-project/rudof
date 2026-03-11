use thiserror::Error;

/// Errors that can occur when working with IRIs.
#[derive(Error, Debug)]
pub enum IriError {
    /// Failed to parse a string as an IRI.
    #[error("Failed to parse IRI '{iri}': {error}")]
    ParseError { iri: String, error: String },

    /// Failed to extend an IRI with a segment.
    #[error("Failed to extend IRI '{base}' with segment '{segment}': {error}")]
    ExtendError {
        base: String,
        segment: String,
        error: String,
    },

    /// Failed to resolve a relative IRI.
    #[error("Failed to resolve '{relative}' against base '{base}': {error}")]
    ResolveError {
        base: String,
        relative: String,
        error: String,
    },

    /// Failed to join a path to an IRI.
    #[error("Failed to join '{path}' to base '{base}': {error}")]
    JoinError {
        base: String,
        path: String,
        error: String,
    },

    /// Failed to dereference an IRI.
    #[error("Failed to dereference IRI '{iri}': {error}")]
    DereferenceError { iri: String, error: String },

    /// Failed to convert a file path to an IRI.
    #[error("Failed to convert path '{path}' to IRI: {error}")]
    PathConversionError { path: String, error: String },

    /// Operation not supported in WASM environment.
    #[error("Operation '{operation}' is not supported in WASM environment")]
    WasmNotSupported { operation: String },
}