use dctap::{DatatypeId, ShapeId, TapShape};
use prefixmap::PrefixMapError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Tap2ShExError {
    #[error("Tap2ShEx converter, not implemented: {msg}")]
    NotImplemented { msg: String },

    #[error("No shape id in tap_shape: {tap_shape:?}")]
    NoShapeId { tap_shape: TapShape },

    #[error(transparent)]
    IriSError {
        #[from]
        err: iri_s::IriSError,
    },

    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: PrefixMapError,
    },

    #[error("No base IRI trying to resolve IRI for {str}")]
    NoBaseIRI { str: String },

    #[error("Multiple value expressions in statement: value_datatype: {value_datatype:?}, value_shape: {value_shape} ")]
    MultipleValueExprInStatement {
        value_datatype: DatatypeId,
        value_shape: ShapeId,
    },

    #[error("Converting value datatype to IRI, no prefix declaration: {datatype_id:?}")]
    DatatypeId2IriNoPrefix { datatype_id: DatatypeId },
}

impl Tap2ShExError {
    pub fn not_implemented(msg: &str) -> Tap2ShExError {
        Tap2ShExError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
