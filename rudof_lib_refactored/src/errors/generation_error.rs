use thiserror::Error;

/// Errors that can occur when generating synthetic RDF data from schemas.
#[derive(Error, Debug)]
pub enum GenerationError {
    /// The schema format for data generation is not supported.
    #[error("Unsupported schema format for data generation: '{format}'. Valid formats are: 'auto', 'shex', 'shacl'")]
    UnsupportedGenerationSchemaFormat { format: String },
}