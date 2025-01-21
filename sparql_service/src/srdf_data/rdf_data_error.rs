use std::io;

use oxigraph::sparql::EvaluationError;
use oxigraph::sparql::SparqlSyntaxError;
use oxigraph::store::StorageError;
use srdf::oxgraph_error::GraphError;
use srdf::oxsparql_error::SparqlError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RdfDataError {
    #[error(transparent)]
    Sparql {
        #[from]
        err: SparqlError,
    },

    #[error(transparent)]
    Graph {
        #[from]
        err: GraphError,
    },

    #[error(transparent)]
    IO {
        #[from]
        err: io::Error,
    },

    #[error(transparent)]
    Storage {
        #[from]
        err: StorageError,
    },

    #[error(transparent)]
    SparqlParse {
        #[from]
        err: SparqlSyntaxError,
    },

    #[error(transparent)]
    SparqlEvaluation {
        #[from]
        err: EvaluationError,
    },
}
