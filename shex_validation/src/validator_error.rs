use shex_ast::ShapeLabel;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ValidatorError {

    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel{ shape: ShapeLabel }
}

