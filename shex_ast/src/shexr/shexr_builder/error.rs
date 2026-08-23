use thiserror::Error;

/// Errors raised while serializing a ShEx [`crate::Schema`] to RDF using the
/// ShExR vocabulary (see [`ShExRBuilder`](super::ShExRBuilder)).
#[derive(Debug, Error)]
pub enum ShExRBuilderError {
    /// The underlying RDF backend failed to add a triple/blank node.
    #[error("Error building RDF: {msg}")]
    RDFBuildError { msg: String },

    /// The construct is valid ShEx but this builder doesn't serialize it yet.
    #[error("Not yet implemented in the ShExR builder: {what}")]
    Unsupported { what: String },
}
