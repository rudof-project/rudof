use rudof_rdf::rdf_core::RDFFormat;

/// What can go wrong while pushing triples into a sink.
#[derive(Debug, thiserror::Error)]
pub enum RdfSinkError {
    #[error("writing RDF: {0}")]
    Io(#[from] std::io::Error),

    /// The graph, store or endpoint behind the sink rejected the triples.
    #[error("the sink's destination rejected the triples: {error}")]
    Store { error: String },

    /// The format asked of a streaming sink has no serializer that takes one
    /// triple at a time.
    #[error("{format} cannot be written one triple at a time")]
    NotStreamable { format: RDFFormat },

    #[error("the sink is finished and takes no more triples")]
    Finished,
}
