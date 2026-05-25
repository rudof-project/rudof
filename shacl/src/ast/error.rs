use rudof_rdf::rdf_core::term::Object;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ASTError {
    #[error("A shape of type {expected} was expected, but found {shape}")]
    UnexpectedShapeType { expected: String, shape: Box<Object> },

    #[error("Not found shape {0}")]
    ShapeNotFound(Box<Object>),
}

impl From<Object> for ASTError {
    fn from(value: Object) -> Self {
        Self::ShapeNotFound(Box::new(value))
    }
}
