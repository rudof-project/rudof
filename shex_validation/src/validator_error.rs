use iri_s::IriS;
use rbe::RbeError;
use shex_ast::{CompiledSchemaError, ShapeLabel, ShapeLabelIdx};
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

    #[error("NodeKind IRI but found {object}")]
    NodeKindIri { object: Object },

    #[error("NodeKind BNode but found {object}")]
    NodeKindBNode { object: Object },

    #[error("NodeKind Literal but found {object}")]
    NodeKindLiteral { object: Object },

    #[error("NodeKind NonLiteral but found {object}")]
    NodeKindNonLiteral { object: Object },

    #[error("Datatype expected {expected} but found {found} for literal with lexical form {lexical_form}")]
    DatatypeDontMatch {
        found: IriS,
        expected: IriS,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found no literal {object}")]
    DatatypeNoLiteral { expected: IriS, object: Object },

    #[error("Failed regular expression")]
    RbeFailed(),

    #[error(transparent)]
    RbeError(#[from] RbeError<IriS, Object, ShapeLabelIdx>),
}
