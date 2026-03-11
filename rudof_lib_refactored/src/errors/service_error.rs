use thiserror::Error;

/// Errors that can occur when working with SPARQL endpoint service descriptions.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// The service result format specified is not supported by Rudof.
    #[error("Unsupported service result format: '{format}'. Valid formats are: 'internal', 'mie', 'json'")]
    UnsupportedResultServiceFormat { format: String },
}