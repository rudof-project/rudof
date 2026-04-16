use thiserror::Error;

/// Errors that can occur during materialization operations in Rudof.
#[derive(Error, Debug)]
pub enum MaterializeError {
    /// No ShEx schema is currently loaded.
    #[error("No ShEx schema loaded. Load a ShEx schema first.")]
    NoShExSchemaLoaded,

    /// No MapState is currently available.
    #[error("No MapState available. Run ShEx validation with Map semantic actions first.")]
    NoMapStateLoaded,

    /// The provided IRI string could not be parsed.
    #[error("Failed to parse IRI '{iri}': {error}")]
    InvalidIri { iri: String, error: String },

    /// The materialization itself failed.
    #[error("Materialization failed: {error}")]
    FailedMaterialization { error: String },

    /// Serializing the materialized RDF graph failed.
    #[error("Failed to serialize materialized graph as '{format}': {error}")]
    FailedSerializingGraph { format: String, error: String },
}
