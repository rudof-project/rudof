use prefixmap::PrefixMapError;
use rbe::RbeError;
use shex_ast::{CompiledSchemaError, Node, Pred, compiled::shape_label::ShapeLabel, ShapeLabelIdx, ShapeExprLabel};
use shex_ast::compiled::preds::Preds;
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

    #[error(transparent)]
    CompiledSchemaError(#[from] CompiledSchemaError),

    #[error("Failed regular expression")]
    RbeFailed(),

    #[error("Closed shape but found properties {remainder:?} which are not part of shape declared properties: {declared:?}")]
    ClosedShapeWithRemainderPreds { remainder: Preds, declared: Preds },

    #[error(transparent)]
    RbeError(#[from] RbeError<Pred, Node, ShapeLabelIdx>),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),

    #[error("ShapeLabel not found {shape_label:?}: {err}")]
    ShapeLabelNotFoundError {
        shape_label: ShapeExprLabel,
        err: CompiledSchemaError
    },

}
