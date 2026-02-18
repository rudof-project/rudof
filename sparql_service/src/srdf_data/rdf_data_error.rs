use std::io;

use oxigraph::{
    sparql::{QueryEvaluationError, SparqlSyntaxError},
    store::StorageError,
};
use thiserror::Error;

use rudof_rdf::{
    rdf_core::RDFFormat,
    rdf_impl::InMemoryGraphError,
};
#[cfg(feature = "network")]
use rudof_rdf::rdf_impl::SparqlEndpointError;

#[derive(Debug, Error)]
pub enum RdfDataError {
    #[error("Error extending query solutions for query '{query}': {error}")]
    ExtendingQuerySolutionsError { query: String, error: String },

    #[cfg(feature = "network")]
    #[error("Error extending query solutions for query '{query} for endpoint {endpoint}': {error}")]
    ExtendingQuerySolutionsErrorEndpoint {
        query: String,
        error: String,
        endpoint: String,
    },

    #[cfg(feature = "network")]
    #[error(transparent)]
    SRDFSparqlError {
        #[from]
        err: SparqlEndpointError,
    },

    #[cfg(feature = "network")]
    #[error("Failed to create SPARQL endpoint {name} with {url}: {err}")]
    SRDFSparqlFromEndpointDescriptionError {
        name: String,
        url: String,
        #[source]
        err: Box<SparqlEndpointError>,
    },

    #[error("RDF graph error: {err}")]
    SRDFGraphError { err: Box<InMemoryGraphError> },

    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },

    #[error("Serializing RDF Data as {format}: {error}")]
    Serializing { error: String, format: RDFFormat },

    #[error(transparent)]
    StorageError {
        #[from]
        err: StorageError,
    },

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

    #[error("Store not initialized")]
    StoreNotInitialized,
}
