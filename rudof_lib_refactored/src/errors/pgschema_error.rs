use thiserror::Error;

/// Errors that can occur when working with Property Graph schemas.
#[derive(Error, Debug)]
pub enum PgSchemaError {
    /// The Property Graph schema format specified is not supported by Rudof.
    #[error("Unsupported Property Graph schema format: '{format}'. Valid formats are: compact, pgschemac")]
    UnsupportedPgSchemaFormat { format: String },

    /// Errors related to specifying the data source.
    #[error("Property Graph schema source specification error: {message}")]
    DataSourceSpec { message: String },

    /// Errors related to parsing the Property Graph schema.
    #[error("Failed to parse Property Graph schema: {error}")]
    FailedParsingPgSchema { error: String },

    /// Failed to write Property Graph schema to an output stream.
    #[error("Failed I/O operation: {error}")]
    FailedIoOperation { error: String },

    /// No Property Graph schema loaded.
    #[error("No Property Graph schema loaded")]
    NoPgschemaLoaded,

    /// No typemap loaded.
    #[error("No typemap loaded")]
    NoTypemapLoaded,

    /// Failed during Property Graph schema validation.
    #[error("Property Graph schema validation failed: {error}")]
    FailedPgschemaValidation { error: String },

    /// Errors related to parsing the typemap.
    #[error("Failed to parse typemap: {error}")]
    FailedParsingTypemap { error: String },

    /// No Property Graph schema validation results available.
    #[error("No Property Graph schema validation results available")]
    NoValidationResultsAvailable,

}