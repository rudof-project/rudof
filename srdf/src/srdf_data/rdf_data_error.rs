use thiserror::Error;

use crate::SRDFSparqlError;

#[derive(Debug, Error)]
pub enum RdfDataError {
    #[error(transparent)]
    SRDFSparqlError {
        #[from]
        err: SRDFSparqlError,
    },
}
