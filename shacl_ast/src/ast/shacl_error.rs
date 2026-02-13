use rdf::rdf_core::term::Object;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclError {
    #[error("NodeShape has an id which is not an IRI: {id}")]
    NodeShapeIdNotIri { id: Box<Object> },

    #[error("Not found shape {shape}")]
    ShapeNotFound { shape: Box<Object> },
}
