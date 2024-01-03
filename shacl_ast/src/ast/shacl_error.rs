use srdf::RDFNode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclError {
    #[error("NodeShape has an id which is not an IRI: {id}")]
    NodeShapeIdNotIri { id: RDFNode },
}
