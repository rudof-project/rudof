use thiserror::Error;

/// Errors that can occur when working with ShEx schemas.
#[derive(Error, Debug)]
pub enum ShExError {
    /// The ShEx format specified is not supported by Rudof.
    #[error(
        "Unsupported ShEx format: '{format}'. Valid formats are: 'internal', 'simple', 'shexc', 'shexj', 'json', 'jsonld', 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads'"
    )]
    UnsupportedShExFormat { format: String },

    /// Errors related to specifying the schema source.
    #[error("Schema source specification error: {message}")]
    DataSourceSpec { message: String },

    /// Failed to parse a ShEx schema in the specified format.
    #[error("Failed to parse ShEx schema from '{source_name}' in format '{format}': {error}")]
    FailedParsingShExSchema {
        error: String,
        source_name: String,
        format: String,
    },

    /// Failed to compile a ShEx schema to intermediate representation.
    #[error("Failed to compile ShEx schema: {error}")]
    FailedCompilingShExSchema { error: String },

    /// No ShEx schema loaded.
    #[error("No ShEx schema loaded")]
    NoShExSchemaLoaded,

    /// No shapemap loaded.
    #[error("No shapemap loaded")]
    NoShapemapLoaded,

    /// No ShEx validation results available.
    #[error("No ShEx validation results available")]
    NoShexValidationResultsAvailable,

    /// Failed to write node info to an output stream.
    #[error("Failed I/O operation: {error}")]
    FailedIoOperation { error: String },

    /// Failed to parse a node selector.
    #[error("Failed to parse node selector '{node_selector}': {error}")]
    NodeSelectorParseError { node_selector: String, error: String },

    /// Failed to parse a shape selector.
    #[error("Failed to parse shape selector '{shape_selector}': {error}")]
    ShapeSelectorParseError { shape_selector: String, error: String },

    /// Invalid shape label.
    #[error("Invalid shape label '{label}': {error}")]
    InvalidShapeLabel { label: String, error: String },

    /// Failed to serialize ShEx schema to the specified format.
    #[error("Failed to serialize ShEx schema to {format}: {error}")]
    FailedSerializingShExSchema { format: String, error: String },

    /// Failed to parse a ShapeMap.
    #[error("Failed to parse ShapeMap from '{source_name}': {error}")]
    FailedParsingShapeMap { source_name: String, error: String },

    /// Failed to serialize ShapeMap to the specified format.
    #[error("Failed to serialize ShapeMap to {format}: {error}")]
    FailedSerializingShapemap { format: String, error: String },

    /// Failed during ShEx validation.
    #[error("ShEx validation failed: {error}")]
    FailedShExValidation { error: String },

    /// Failed to serialize ShEx validation results to the specified format.
    #[error("Failed to serialize ShEx validation results to {format}: {error}")]
    FailedSerializingShExValidationResults { format: String, error: String },
}
