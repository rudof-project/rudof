use iri_s::{IriSError, IriS};
use srdf::Object;
use thiserror::Error;

use crate::schema_json;
use crate::schema_json::TripleExprLabel;

#[derive(Error, Debug)]
pub enum CompiledSchemaError {
    #[error("Parsing {str:?} as IRI")]
    Str2IriError { str: String },

    #[error("Parsing as IRI: {err:?}")]
    IriParseError {
        #[from]
        err: IriSError,
    },

    #[error("SchemaJson Error")]
    SchemaJsonError(#[from] schema_json::SchemaJsonError),

    #[error("Duplicated triple expression label in schema: {label:?}")]
    DuplicatedTripleExprLabel { label: TripleExprLabel },

    #[error("Converting min value {min} must be positive")]
    MinLessZero { min: i32 },

    #[error("Converting max value {max} must be > -1")]
    MaxIncorrect { max: i32 },

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
}
