use iri_s::{IriS, IriSError};
use srdf::lang::Lang;
use srdf::Object;
use thiserror::Error;

use crate::schema_json::TripleExprLabel;
use crate::{schema_json, Node};

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

    #[error("NodeKind IRI but found {node}")]
    NodeKindIri { node: Node },

    #[error("NodeKind BNode but found {node}")]
    NodeKindBNode { node: Node },

    #[error("NodeKind Literal but found {node}")]
    NodeKindLiteral { node: Node },

    #[error("NodeKind NonLiteral but found {node}")]
    NodeKindNonLiteral { node: Node },

    #[error("Datatype expected {expected} but found {found} for literal with lexical form {lexical_form}")]
    DatatypeDontMatch {
        found: IriS,
        expected: IriS,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found no literal {node}")]
    DatatypeNoLiteral { expected: IriS, node: Node },

    #[error("Datatype expected {expected} but found String literal {lexical_form}")]
    DatatypeDontMatchString {
        expected: IriS,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found String literal {lexical_form}")]
    DatatypeDontMatchLangString {
        expected: IriS,
        lexical_form: String,
        lang: Lang,
    },
}
