use thiserror::Error;
use crate::errors::*;

/// Main error type that encompasses all error categories in Rudof
#[derive(Error, Debug)]
pub enum RudofError {
    /// Input specification and source errors.
    #[error("Input specification error: {0}")]
    InputSpec(#[from] InputSpecError),

    /// WebAssembly environment limitation errors.
    #[error("WASM error: {0}")]
    WASM(#[from] WASMError),

    /// ShapeMap errors.
    #[error("ShapeMap error: {0}")]
    ShapeMap(#[from] ShapeMapError),

    /// ShEx schema errors.
    #[error("ShEx error: {0}")]
    ShEx(#[from] ShExError),

    /// Property Graph schema errors.
    #[error("Property Graph schema error: {0}")]
    PgSchema(#[from] PgSchemaError),

    /// Validation errors.
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Data errors.
    #[error("Data error: {0}")]
    Data(#[from] DataError),

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

    /// The requested operation is not yet implemented.
    #[error("Not implemented: {msg}")]
    NotImplemented { msg: String },
}