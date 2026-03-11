use thiserror::Error;

/// Errors that can occur when working with Property Graph schemas.
#[derive(Error, Debug)]
pub enum PgSchemaError {
    /// The Property Graph schema format specified is not supported by Rudof.
    #[error("Unsupported Property Graph schema format: '{format}'. Valid formats are: compact, pgschemac")]
    UnsupportedPgSchemaFormat { format: String },
}