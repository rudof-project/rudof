use thiserror::Error;

/// An error evaluating a ShExPath.
#[derive(Error, Debug)]
pub enum ShExPathError {
    /// The schema could not be written as ShExJ, the document a path is evaluated over.
    ///
    /// # Fields
    /// - `msg`: Description of the serialization failure
    #[error("Error writing the schema as ShExJ: {msg}")]
    Serialization { msg: String },
}
