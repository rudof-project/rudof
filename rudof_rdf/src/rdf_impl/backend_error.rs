//! Error type returned by the [`RdfBackend`](super::RdfBackend) strategy enum.
//!
//! Each variant wraps the concrete error type produced by one of theunderlying backends.

use thiserror::Error;

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
use super::OxigraphEndpointError;
use super::OxigraphInMemoryError;
#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
use super::QleverError;

#[derive(Debug, Error)]
pub enum RdfBackendError {
    #[error(transparent)]
    InMemory(#[from] Box<OxigraphInMemoryError>),

    #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
    #[error(transparent)]
    Endpoint(#[from] Box<OxigraphEndpointError>),

    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    #[error(transparent)]
    Qlever(#[from] Box<QleverError>),

    #[error("backend {backend} does not support operation `{op}` (read-only)")]
    ReadOnly { op: &'static str, backend: &'static str },
}

impl From<OxigraphInMemoryError> for RdfBackendError {
    fn from(e: OxigraphInMemoryError) -> Self {
        RdfBackendError::InMemory(Box::new(e))
    }
}

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
impl From<OxigraphEndpointError> for RdfBackendError {
    fn from(e: OxigraphEndpointError) -> Self {
        RdfBackendError::Endpoint(Box::new(e))
    }
}

#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
impl From<QleverError> for RdfBackendError {
    fn from(e: QleverError) -> Self {
        RdfBackendError::Qlever(Box::new(e))
    }
}
