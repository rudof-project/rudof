use iri_s::{IriS, IriSError};
use prefixmap::IriRef;
use srdf::lang::Lang;
//use srdf::Object;
use thiserror::Error;

use crate::ast::TripleExprLabel;
use crate::{ast, Node, ShapeLabel};

#[derive(Error, Debug)]
pub enum CompiledSchemaError {
    #[error("Parsing {str:?} as IRI")]
    Str2IriError { str: String },

    #[error("Parsing {str} as IRI: {err:?}")]
    IriParseError { str: String, err: IriSError },

    #[error("SchemaJson Error")]
    SchemaJsonError(#[from] ast::SchemaJsonError),

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
        found: IriRef,
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found no literal {node}")]
    DatatypeNoLiteral { expected: IriRef, node: Node },

    #[error("Datatype expected {expected} but found String literal {lexical_form}")]
    DatatypeDontMatchString {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found String literal {lexical_form}")]
    DatatypeDontMatchLangString {
        expected: IriRef,
        lexical_form: String,
        lang: Lang,
    },

    #[error("Shape label not found {shape_label}")]
    ShapeLabelNotFound { shape_label: ShapeLabel },

    #[error("Not implemented yet: {msg}")]
    Todo { msg: String },

    #[error("Can't convert prefixed name {prefix}:{local} to shape label")]
    IriRef2ShapeLabelError { prefix: String, local: String },

    #[error("Internal: {msg}")]
    Internal { msg: String },
}
