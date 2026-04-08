use crate::errors::*;
use thiserror::Error;

/// Main error type that encompasses all error categories in Rudof
#[derive(Error, Debug)]
pub enum RudofError {
    /// Configuration errors.
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Input specification and source errors.
    #[error("Input specification error: {0}")]
    InputSpec(#[from] InputSpecError),

    /// ShapeMap errors.
    #[error("ShapeMap error: {0}")]
    ShapeMap(#[from] ShapeMapError),

    /// ShEx schema errors.
    #[error("ShEx error: {0}")]
    ShEx(#[from] ShExError),

    /// SHACL schema errors.
    #[error("SHACL error: {0}")]
    Shacl(#[from] ShaclError),

    /// Property Graph schema errors.
    #[error("Property Graph schema error: {0}")]
    PgSchema(#[from] PgSchemaError),

    /// Validation errors.
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Data errors.
    #[error("Data error: {0}")]
    Data(#[from] Box<DataError>),

    /// Node inspection errors.
    #[error("Node inspection error: {0}")]
    NodeInspection(#[from] NodeInspectionError),

    /// DC-TAP (Dublin Core Tabular Application Profile) errors.
    #[error("DC-TAP error: {0}")]
    DCTap(#[from] DCTapError),

    /// Conversion errors.
    #[error("Conversion error: {0}")]
    Conversion(#[from] ConversionError),

    /// Schema comparison errors.
    #[error("Schema comparison error: {0}")]
    Comparison(#[from] ComparisonError),

    /// RDF-config errors.
    #[error("RDF-config error: {0}")]
    RdfConfig(#[from] RdfConfigError),

    /// Service description errors.
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),

    /// SPARQL query errors.
    #[error("Query error: {0}")]
    Query(#[from] QueryError),

    /// Generate synthetic RDF data errors.
    #[error("Generate error: {0}")]
    Generate(#[from] GenerationError),

    /// IRI-related errors.
    #[error("IRI error: {0}")]
    Iri(#[from] IriError),

    /// The requested operation is not yet implemented.
    #[error("Not implemented: {msg}")]
    NotImplemented { msg: String },

    /// A generic error with a message.
    #[error("Error: {error}")]
    Generic { error: String },
}
