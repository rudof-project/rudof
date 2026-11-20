use rudof_rdf::rdf_core::Rdf;
use thiserror::Error;

/// Errors raised while building or serializing an EARL report
#[derive(Debug, Error)]
pub enum EarlError {
    #[error("Error building the RDF graph of the EARL report: {0}")]
    GraphError(String),

    #[error("Cannot convert a {0} into a {1} while writing the EARL report")]
    CastError(String, String),

    #[error("Unknown EARL outcome: '{0}'")]
    UnknownOutcome(String),

    #[error("Unknown EARL test mode: '{0}'")]
    UnknownTestMode(String),
}

impl EarlError {
    pub fn new_graph_error_ctx<RDF: Rdf>(err: RDF::Err, ctx: &str) -> Self {
        Self::GraphError(format!("{ctx}: {err}"))
    }
}

/// Maps a [`Rdf::Err`] into an [`EarlError`]
pub(crate) fn graph_error<RDF: Rdf>(msg: &str) -> impl FnOnce(RDF::Err) -> EarlError {
    move |e| EarlError::new_graph_error_ctx::<RDF>(e, msg)
}
