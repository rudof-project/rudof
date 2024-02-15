use iri_s::IriSError;
use prefixmap::DerefError;
use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;



#[derive(Error, Debug)]
pub enum ParseError {

    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },

    #[error("{msg}")]
    Custom { msg: String },

    #[error(transparent)]
    IRISError {
        #[from]
        err: IriSError,
    },

    #[error(transparent)]
    DerefError {
        #[from]
        err: DerefError,
    },

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Expected further input: {0}")]
    MissingInput(String),

    #[error(r#"Expected "{0}""#)]
    ExpectedToken(String),

    #[error("Expected shape definition of shape reference")]
    ExpectedShapeOrRef,

    #[error("Expected shape expression declaration")]
    ExpectedShapeExprDecl,

    #[error("Expected exclusion that starts by .")]
    ExclusionPlus,

    #[error("Expected exclusion")]
    Exclusion,

    #[error("Expected set of values between [ and ]")]
    ValueSet,

    #[error("Expected value set value")]
    ValueSetValue,

    #[error("Expected value set")]
    ValueSetFacets,

    #[error("Expected literal node constraint")]
    LitNodeConstraint,

    #[error("Expected shape expression definition or external ")]
    ShapeExprOrExternal,

    #[error("Expected non literal node constraint followed by optional shape or shape reference")]
    NonLitNodeConstraintOptShapeOrRef,

    #[error("Expected non literal inline node constraint followed by optional shape or shape reference")]
    NonLitInlineNodeConstraintOptShapeOrRef,

    #[error("Expected inline shape atom")]
    ExpectedInlineShapeAtom,

    #[error("Expected datatype with optional xs_facets")]
    DatatypeFacets,

    #[error("Expected prefixed name")]
    ExpectedPrefixedName,

    #[error("Expected extends followed by shape references")]
    Extension,

    #[error("Expected Start declaration")]
    ExpectedStart,

    #[error("Expected cardinality")]
    ExpectedCardinality,

    #[error("Expected triple constraint")]
    ExpectedTripleConstraint,

    #[error("Expected literal range")]
    ExpectedLiteralRange,

    #[error("Expected prefix declaration")]
    ExpectedPrefixDecl,

    #[error("Expected cardinality declaration starting by {{")]
    ExpectedRepeatRange,

    #[error("Expected rest of cardinality declaration after comma")]
    ExpectedRestRepeatRange,

    #[error("Expected shape expr")]
    ExpectedShapeExpr,

    #[error("Expected inline shape expr")]
    ExpectedInlineShapeExpr,

    #[error("Expected IRI or Literal")]
    ExpectedIriOrLiteral,

    #[error("Expected language range")]
    LanguageRange,

    #[error("Expected Literal")]
    Literal,

    #[error("Expected Shape Atom")]
    ShapeAtom,

    #[error("Expected annotation")]
    ExpectedAnnotation,

    #[error("Expected triple expression")]
    TripleExpression,

    #[error("Expected string literal between single quotes")]
    StringLiteralQuote,

    #[error("Expected RDF Literal")]
    RDFLiteral,

    #[error("Expected triple expression between parenthesis")]
    BracketedTripleExpr,

    #[error("Expected OneOf triple expression")]
    OneOfTripleExpr,

    #[error("Expected code in semantic action")]
    Code,

    #[error("Expected code declaration")]
    CodeDeclaration,

    #[error("Expected unary triple expression")]
    UnaryTripleExpr,

    #[error("Expected include")]
    Include,

    #[error("Expected base declaration")]
    ExpectedBaseDecl,

    #[error("Expected import declaration")]
    ExpectedImportDecl,

    #[error("Expected string literal")]
    ExpectedStringLiteral,

    #[error("Expected shape definition")]
    ExpectedShapeDefinition,

    #[error("Expected EXTRA property set")]
    ExpectedEXTRAPropertySet,

    #[error("Expected CLOSED")]
    ExpectedClosed,

    #[error("Expected CLOSED or EXTRA followed by list of predicates")]
    ExpectedQualifier,

    #[error("Expected list of CLOSED or EXTRA followed by list of predicates")]
    ExpectedQualifiers,

    #[error("Parse int error for str {str}: {err} ")]
    ParseIntError { str: String, err: ParseIntError },

    #[error("Parse f64 error for str {str}: {err}")]
    ParseFloatError { str: String, err: ParseFloatError },

    #[error("Expected numeric literal")]
    NumericLiteral,

    #[error("Expected integer literal")]
    IntegerLiteral,

    #[error("Expected integer")]
    Integer,

    #[error("Expected ShapeSpec: IRIREF, BNode or START")]
    ExpectedShapeSpec,

    #[error("Expected ShEx statement")]
    ExpectedStatement,

    #[error("Expected ShapeMap association")]
    ExpectedShapeMapAssociation,

    #[error("Expected node selector specification")]
    ExpectedNodeSpec,


}

