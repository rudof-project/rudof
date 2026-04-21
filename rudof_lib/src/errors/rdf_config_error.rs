use thiserror::Error;

/// Errors that can occur when working with RDF-config specifications.
#[derive(Error, Debug)]
pub enum RdfConfigError {
    /// The RDF-config specification format is not supported by Rudof.
    #[error("Unsupported RDF-config format: '{format}'. Valid formats are: 'yaml'")]
    UnsupportedRdfConfigFormat { format: String },

    /// The RDF-config result format is not supported by Rudof.
    #[error("Unsupported RDF-config result format: '{format}'. Valid formats are: 'internal', 'yaml'")]
    UnsupportedResultRdfConfigFormat { format: String },

    /// Errors related to specifying the data source.
    #[error("RDF-config source specification error: {message}")]
    DataSourceSpec { message: String },

    /// Failed to write RDF config to an output stream.
    #[error("Failed I/O operation: {error}")]
    FailedIoOperation { error: String },

    /// No RDF config loaded.
    #[error("No RDF config loaded")]
    NoRdfConfigLoaded,
}
