use dctap::{DatatypeId, PropertyId, ShapeId, TapShape};
use prefixmap::error::PrefixMapError;
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
        err: iri_s::error::IriSError,
    },

    #[error("PrefixMap error: {err} at line: {line}, field: {field}")]
    ResolvingPrefixError {
        err: PrefixMapError,
        line: u64,
        field: String,
    },

    #[error("No base IRI trying to resolve IRI for {str}")]
    NoBaseIRI { str: String },

    #[error(
        "Multiple value expressions in statement: value_datatype: {value_datatype:?}, value_shape: {value_shape} "
    )]
    MultipleValueExprInStatement {
        value_datatype: DatatypeId,
        value_shape: ShapeId,
    },

    #[error("Converting value datatype to IRI, no prefix declaration: {datatype_id:?}")]
    DatatypeId2IriNoPrefix { datatype_id: DatatypeId },

    #[error("Converting shape to IRI, no prefix declaration: {shape_id:?}")]
    ShapeId2IriNoPrefix { shape_id: ShapeId },

    #[error("No base IRI converting {str} to IRI. Line: {line}")]
    IriNoPrefix { str: String, line: u64 },

    #[error("Converting property to IRI, no prefix declaration: {property_id:?}")]
    PropertyId2IriNoPrefix { property_id: PropertyId },

    #[error("Parsing ValueShape at line: {line}: {error}")]
    ParsingValueShape {
        line: u64,
        error: Box<Tap2ShExError>,
    },
}

impl Tap2ShExError {
    pub fn not_implemented(msg: &str) -> Tap2ShExError {
        Tap2ShExError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
