use shex_ast::ShapeLabel;
use srdf::Object;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel { shape: ShapeLabel },

    #[error("Error converting object to iri: {object}")]
    ConversionObjectIri { object: Object },
}
