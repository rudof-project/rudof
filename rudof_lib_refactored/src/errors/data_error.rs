use thiserror::Error;

/// Errors that can occur during data handling operations in Rudof.
#[derive(Error, Debug)]
pub enum DataError {
    /// Error applying RDF data configuration.
    #[error("Error applying RDF data configuration: {error}")]
    RdfDataConfig { error: String },

    /// Attempted to use a non-RDF format in an RDF-only context.
    #[error("Non-RDF format: {format}")]
    NonRdfFormat { format: String },

    /// The data format specified is not supported by Rudof.
    #[error("Unsupported data format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'jsonld', 'pg'")]
    UnsupportedDataFormat { format: String },

    /// The data reader mode specified is not supported by Rudof.
    #[error("Unsupported reader mode: '{mode}'. Valid modes are 'strict' or 'lax'")]
    UnsupportedReaderMode {
        mode: String
    },

    /// The result data format specified is not supported by Rudof.
    #[error("Unsupported result data format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'compact', 'json', 'plantuml', 'svg', 'png'")]
    UnsupportedResultDataFormat { format: String },
}