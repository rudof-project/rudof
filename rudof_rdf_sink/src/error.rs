/// What can go wrong while pushing triples into a sink.
#[derive(Debug, thiserror::Error)]
pub enum RdfSinkError {
    #[error("writing RDF: {0}")]
    Io(#[from] std::io::Error),

    /// The graph, store or endpoint behind the sink rejected the triples.
    #[error("the sink's destination rejected the triples: {error}")]
    Store { error: String },

    #[error("the sink is finished and takes no more triples")]
    Finished,
}
