use thiserror::Error;

/// Errors that can occur when working with SHACL schemas.
#[derive(Error, Debug)]
pub enum ShaclError {
    /// The SHACL schema format specified is not supported by Rudof.
    #[error("Unsupported SHACL schema format: '{format}'. Valid formats are: 'internal', 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'jsonld'")]
    UnsupportedShaclSchemaFormat { format: String },

    /// The SHACL schema format 'internal' is not readable. It can only be used for writing SHACL schemas.
    #[error("SHACL schema format 'internal' is not readable. It can only be used for writing SHACL schemas.")]
    InternalSHACLFormatNonReadable,

    /// Errors related to specifying the data source.
    #[error("SHACL source specification error: {message}")]
    DataSourceSpec { message: String },

    #[error(
        "Failed to parse SHACL schema from '{source_name}' with format '{format}': {error}"
    )]
    FailedParsingShaclSchema {
        source_name: String,
        format: String,
        error: String,
    },

    /// No SHACL shapes loaded.
    #[error("No SHACL shapes loaded")]
    NoShaclShapesLoaded,

    /// Failed to compile a SHACL schema to intermediate representation.
    #[error("Failed to compile SHACL schema: {error}")]
    FailedCompilingShaclSchema { error: String },

    /// Failed to write info to an output stream.
    #[error("Failed I/O operation: {error}")]
    FailedIoOperation { error: String },

    /// Failed during SHACL validation.
    #[error("SHACL validation failed: {error}")]
    FailedShaclValidation { error: String },

    /// No SHACL validation results available.
    #[error("No SHACL validation results available")]
    NoShaclValidationResultsAvailable,
}