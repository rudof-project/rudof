use crate::ParseError;
use thiserror::Error;

/// An error evaluating a ShExPath.
#[derive(Error, Debug)]
pub enum ShExPathError {
    /// The path is not a ShExPath.
    #[error("Error parsing the ShExPath: {0}")]
    Parse(#[from] ParseError),

    /// The schema could not be written as ShExJ, the document a path is evaluated over.
    ///
    /// # Fields
    /// - `msg`: Description of the serialization failure
    #[error("Error writing the schema as ShExJ: {msg}")]
    Serialization { msg: String },

    /// The path uses something the evaluator does not evaluate.
    ///
    /// # Fields
    /// - `feature`: What is not evaluated
    #[error("ShExPath evaluation of {feature} is not implemented")]
    NotImplemented { feature: String },
}
