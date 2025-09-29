use iri_s::IriSError;
use prefixmap::{IriRef, PrefixMapError};
use srdf::lang::Lang;
use thiserror::Error;

use super::shape_label::ShapeLabel;
use crate::ast::TripleExprLabel;
use crate::{Node, ast};
use srdf::numeric_literal::NumericLiteral;

#[derive(Error, Debug, Clone)]
pub enum SchemaIRError {
    #[error("Pattern  /{regex}/{} not found in {node}", flags.as_deref().unwrap_or(""))]
    PatternNodeNotLiteral {
        node: String,
        regex: String,
        flags: Option<String>,
    },

    #[error("Invalid regex /{regex}")]
    InvalidRegex { regex: String },

    #[error("Error matching /{regex}/{flags} with {lexical_form}")]
    PatternError {
        regex: String,
        flags: String,
        lexical_form: String,
    },

    #[error("Error creating language tag: {lang}: {err}")]
    LangTagError { lang: String, err: String },

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

    #[error(
        "Datatype expected {expected} but found {found} for literal with lexical form {lexical_form}"
    )]
    DatatypeDontMatch {
        found: IriRef,
        expected: IriRef,
        lexical_form: String,
    },

    #[error(
        "Datatype expected {expected} but found a wrong datatype with lexical form {lexical_form} and declared datatype {datatype}: {error}"
    )]
    WrongDatatypeLiteralMatch {
        lexical_form: String,
        datatype: IriRef,
        error: String,
        expected: IriRef,
    },

    #[error("Datatype expected {expected} but found literal {node} which has datatype: {}", (*node).datatype().map(|d| d.to_string()).unwrap_or("None".to_string()))]
    DatatypeNoLiteral {
        expected: Box<IriRef>,
        node: Box<Node>,
    },

    #[error("Datatype expected {expected} but found String literal {lexical_form}")]
    DatatypeDontMatchString {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found Integer literal {lexical_form}")]
    DatatypeDontMatchInteger {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found decimal literal {lexical_form}")]
    DatatypeDontMatchDecimal {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found float literal {lexical_form}")]
    DatatypeDontMatchFloat {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found long literal {lexical_form}")]
    DatatypeDontMatchLong {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Datatype expected {expected} but found double literal {lexical_form}")]
    DatatypeDontMatchDouble {
        expected: IriRef,
        lexical_form: String,
    },

    #[error("Expected language tag {lang} for StringLiteral with lexical form {lexical_form}")]
    DatatypeDontMatchLangString {
        lexical_form: String,
        lang: Box<Lang>,
    },

    #[error("Length of {node} = {found} doesn't match {expected}")]
    LengthError {
        expected: usize,
        found: usize,
        node: String,
    },

    #[error("MinLength of {node} = {found} doesn't match {expected}")]
    MinLengthError {
        expected: usize,
        found: usize,
        node: String,
    },

    #[error("MaxLength of {node} = {found} doesn't match {expected}")]
    MaxLengthError {
        expected: usize,
        found: usize,
        node: String,
    },

    #[error("NumericValue of {node} = {found} doesn't match minInclusive of {expected}")]
    MinInclusiveError {
        expected: NumericLiteral,
        found: NumericLiteral,
        node: String,
    },

    #[error("Node {node} is not a numeric literal")]
    NonNumeric { node: String },

    #[error("Shape label not found {shape_label}")]
    ShapeLabelNotFound { shape_label: ShapeLabel },

    #[error("Not implemented yet: {msg}")]
    Todo { msg: String },

    #[error("Can't convert prefixed name {prefix}:{local} to shape label")]
    IriRef2ShapeLabelError { prefix: String, local: String },

    #[error("Can't find prefixed name {prefix}:{local} in prefixmap: {err}")]
    PrefixedNotFound {
        prefix: String,
        local: String,
        err: Box<PrefixMapError>,
    },

    #[error("Label not found: {shape_label}")]
    LabelNotFound { shape_label: ShapeLabel },

    #[error("Internal: {msg}")]
    Internal { msg: String },
}
