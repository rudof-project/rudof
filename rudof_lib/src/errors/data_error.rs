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

    /// Attempted to use a non-image visualization format in an image-only context.
    #[error("Non-image visualization format: {format}")]
    NonImageVisualizationFormat { format: String },

    /// The data format specified is not supported by Rudof.
    #[error(
        "Unsupported data format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'jsonld', 'pg'"
    )]
    UnsupportedDataFormat { format: String },

    /// The data reader mode specified is not supported by Rudof.
    #[error("Unsupported reader mode: '{mode}'. Valid modes are 'strict' or 'lax'")]
    UnsupportedReaderMode { mode: String },

    /// Errors related to specifying the data source (e.g. both or neither data and endpoint provided).
    #[error("Data source specification error: {message}")]
    DataSourceSpec { message: String },

    /// The result data format specified is not supported by Rudof.
    #[error(
        "Unsupported result data format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'compact', 'json', 'plantuml', 'svg', 'png'"
    )]
    UnsupportedResultDataFormat { format: String },

    /// Failed to create a SPARQL endpoint from the given IRI.
    #[error("Invalid endpoint '{endpoint}': {error}")]
    InvalidEndpoint { endpoint: String, error: String },

    /// Failed to parse RDF data.
    #[error(
        "Failed to parse RDF data from '{source_name}' with format '{format}', base '{base}', reader mode '{reader_mode}': {error}"
    )]
    FailedParsingRdfData {
        source_name: String,
        format: String,
        base: String,
        reader_mode: String,
        error: String,
    },

    /// Failed to parse Property Graph data.
    #[error("Failed to parse PG data from '{source_name}': {error}")]
    FailedParsingPgData { source_name: String, error: String },

    /// Failed to parse Service Description data.
    #[error(
        "Failed to parse Service Description data from '{source_name}' with format '{format}', base '{base}', reader mode '{reader_mode}': {error}"
    )]
    FailedParsingServiceDescriptionData {
        source_name: String,
        format: String,
        base: String,
        reader_mode: String,
        error: String,
    },

    /// Failed to serialize Service Description.
    #[error("Failed to serialize Service Description with format '{result_service_format}': {error}")]
    FailedSerializingServiceDescription {
        result_service_format: String,
        error: String,
    },

    /// Failed to serialize Rdf Data.
    #[error("Failed to serialize Rdf Data with format '{result_data_format}': {error}")]
    FailedSerializingRdfData { result_data_format: String, error: String },

    /// Failed to serialize Property Graph Data.
    #[error("Failed to serialize Property Graph Data with format '{result_pg_format}': {error}")]
    FailedSerializingPgData { result_pg_format: String, error: String },

    /// No Service Description loaded.
    #[error("No Service Description loaded")]
    NoServiceDescription,

    /// No RDF data loaded.
    #[error("No RDF data loaded")]
    NoRdfDataLoaded,

    /// No Pg data loaded.
    #[error("No PG data loaded")]
    NoPgDataLoaded,

    /// No data loaded.
    #[error("No data loaded")]
    NoDataLoaded,

    /// Failed to serialize data to the specified format.
    #[error("Failed to serialize data to {format}: {error}")]
    FailedSerializingData { format: String, error: String },

    /// Failed to parse a node selector.
    #[error("Failed parsing node selector '{node}': {error}")]
    FailedNodeSelectorParse { node: String, error: String },

    /// Failed to parse an IRI reference.
    #[error("Failed parsing Iri ref '{iri}': {error}")]
    FailedIriRefParse { iri: String, error: String },

    /// Failed to resolve a prefix in a prefixed IRI reference.
    #[error("Failed resolving prefix '{prefix}': {error}")]
    FailedPrefixResolution { prefix: String, error: String },

    /// Failed to retrieve arcs for a node.
    #[error("Failed retrieving arcs for node: {error}")]
    FailedArcRetrieval { error: String },

    /// Failed to qualify a node or term for display.
    #[error("Failed qualifying node or term: {error}")]
    FailedQualification { error: String },

    /// Failed to write node info to an output stream.
    #[error("Failed I/O operation while writing node info: {error}")]
    FailedIoOperation { error: String },
}
