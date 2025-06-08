use thiserror::Error;

#[derive(Error, Debug)]
pub enum RDFError {
    #[error("Converting Object to RDF term")]
    ConversionError(String),

    #[error("Comparison error: {term1} with {term2}")]
    ComparisonError { term1: String, term2: String },
}
