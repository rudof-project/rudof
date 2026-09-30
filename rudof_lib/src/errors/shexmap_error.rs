use thiserror::Error;

/// Errors of the ShExMap operations (`shexmap_bind`, `shexmap_materialize`, `shexmap_check`).
#[derive(Error, Debug)]
pub enum ShExMapError {
    /// No RDF data is currently loaded.
    #[error("No RDF data loaded. Load the input graph first.")]
    NoRdfDataLoaded,

    /// No ShEx schema is currently loaded.
    #[error("No ShEx schema loaded. Load the {which} schema first.")]
    NoShExSchemaLoaded { which: String },

    /// No bindings are available to materialize from.
    #[error("No ShExMap bindings available. Bind an input graph or load a bindings file first.")]
    NoBindings,

    /// A node could not be parsed.
    #[error("Invalid node '{node}': {error}")]
    InvalidNode { node: String, error: String },

    /// A shape label could not be parsed.
    #[error("Invalid shape label '{label}': {error}")]
    InvalidShapeLabel { label: String, error: String },

    /// A static variable's value could not be parsed.
    #[error("Invalid value for static variable {variable}: {error}")]
    InvalidStaticVariable { variable: String, error: String },

    /// The validator rejected the focus node before binding.
    #[error("{node} does not conform to {shape}: {status}")]
    Validation {
        node: String,
        shape: String,
        status: String,
    },

    /// Binding, materialization or analysis failed (see `shex_ast::shexmap`).
    #[error("{error}")]
    Failed { error: String },

    /// Reading or writing a bindings file failed.
    #[error("Failed reading or writing ShExMap bindings: {error}")]
    Bindings { error: String },

    /// Reading the graph to update in place failed.
    #[error("Failed reading the graph to update from '{target}': {error}")]
    FailedReadingTarget { target: String, error: String },

    /// Serializing the materialized graph failed.
    #[error("Failed to serialize the materialized graph as '{format}': {error}")]
    FailedSerializingGraph { format: String, error: String },
}
