use thiserror::Error;

/// Errors that occur when attempting unsupported operations in WASM environments.
#[derive(Error, Debug)]
pub enum WASMError {
    /// File path operations are not available in WASM.
    #[error("File path operations are not supported in WASM environment")]
    FilePathNotSupported,

    /// HTTP client operations are not implemented in WASM.
    #[error("HTTP client operations are not supported in WASM environment")]
    HttpClientNotSupported,

    /// HTTP response reading is not implemented in WASM.
    #[error("HTTP response reading is not supported in WASM environment")]
    HttpResponseNotSupported,
}