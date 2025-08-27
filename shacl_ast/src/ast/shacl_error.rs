use srdf::{Object, RDFNode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclError {
    #[error("NodeShape has an id which is not an IRI: {id}")]
    NodeShapeIdNotIri { id: RDFNode },

    #[error("Not found shape {shape}")]
    ShapeNotFound { shape: Object },
}
