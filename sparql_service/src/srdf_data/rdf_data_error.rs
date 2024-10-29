use std::io;

use oxigraph::{
    sparql::{EvaluationError, SparqlSyntaxError},
    store::StorageError,
};
use thiserror::Error;

use srdf::{SRDFGraphError, SRDFSparqlError};

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

    #[error("the Store has not been materialized, no query operation can be performed. Try setting the proper LoadMode when creating the Graph")]
    MaterializationError,
}
