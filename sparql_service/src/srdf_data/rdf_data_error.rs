use std::io;

use oxigraph::sparql::{QueryEvaluationError, SparqlSyntaxError};
use thiserror::Error;

use rudof_rdf::{
    rdf_core::RDFFormat,
    rdf_impl::{OxigraphEndpointError, RdfBackendError},
};

#[derive(Debug, Error)]
pub enum RdfDataError {
    #[error("Error extending query solutions for query '{query}': {error}")]
    ExtendingQuerySolutionsError { query: String, error: String },

    #[error("Error extending query solutions for query '{query} for endpoint {endpoint}': {error}")]
    ExtendingQuerySolutionsErrorEndpoint {
        query: String,
        error: String,
        endpoint: String,
    },

    #[error(transparent)]
    SRDFSparqlError {
        #[from]
        err: OxigraphEndpointError,
    },

    #[error("Failed to create SPARQL endpoint {name} with {url}: {err}")]
    SRDFSparqlFromEndpointDescriptionError {
        name: String,
        url: String,
        #[source]
        err: Box<OxigraphEndpointError>,
    },

    /// Any failure surfaced through the `RdfBackend` strategy enum. Wraps
    /// the structured per-backend error so callers can downcast if needed.
    #[error(transparent)]
    Backend {
        #[from]
        err: Box<RdfBackendError>,
    },

    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },

    #[error("Serializing RDF Data as {format}: {error}")]
    Serializing { error: String, format: RDFFormat },

    #[error(transparent)]
    SparqlParseError {
        #[from]
        err: SparqlSyntaxError,
    },

    #[error(transparent)]
    SparqlEvaluationError {
        #[from]
        err: QueryEvaluationError,
    },

    #[error("Trying to create a BNode on RDF data without a graph")]
    BNodeNoGraph,

    #[error("RDF data backend is not in-memory; cannot add triples to {backend}")]
    NotInMemoryBackend { backend: &'static str },
}

impl From<RdfBackendError> for RdfDataError {
    fn from(e: RdfBackendError) -> Self {
        RdfDataError::Backend { err: Box::new(e) }
    }
}
