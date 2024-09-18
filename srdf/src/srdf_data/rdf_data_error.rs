use std::io;

use thiserror::Error;

use crate::{SRDFGraphError, SRDFSparqlError};

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
}
