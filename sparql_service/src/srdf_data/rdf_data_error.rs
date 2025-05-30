use std::io;

use oxigraph::{
    sparql::{EvaluationError, SparqlSyntaxError},
    store::StorageError,
};
use thiserror::Error;

use srdf::{RDFFormat, SRDFGraphError, SRDFSparqlError};

#[derive(Debug, Error)]
pub enum RdfDataError {
    #[error(transparent)]
    SRDFSparqlError {
        #[from]
        err: SRDFSparqlError,
    },

    #[error(transparent)]
    SRDFGraphError {
        #[from]
        err: SRDFGraphError,
    },

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
        err: EvaluationError,
    },

    #[error("Trying to create a BNode on RDF data without a graph")]
    BNodeNoGraph,
}
