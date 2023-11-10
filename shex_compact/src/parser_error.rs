use iri_s::IriSError;
use shex_ast::DerefError;
use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

use crate::{LocatedParseError, Span};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Nom Parsing error: {err:?}")]
    NomError { err: Box<LocatedParseError> },

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

    #[error("Expected prefixed name")]
    ExpectedPrefixedName,

    #[error("Expected Start declaration")]
    ExpectedStart,

    #[error("Expected cardinality")]
    ExpectedCardinality,

    #[error("Expected triple constraint")]
    ExpectedTripleConstraint,

    #[error("Expected prefix declaration")]
    ExpectedPrefixDecl,

    #[error("Expected cardinality declaration starting by {{")]
    ExpectedRepeatRange,

    #[error("Expected rest of cardinality declaration after comma")]
    ExpectedRestRepeatRange,

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
    ParseFloatError {
        str: String, 
        
        err: ParseFloatError,
    },
}

impl ParseError {
    /// Locate this error by adding a position.
    pub fn at(self, position: Span) -> LocatedParseError {
        // miri doesn't like nom_locate, cf. https://github.com/fflorent/nom_locate/issues/88
        let column = if cfg!(not(miri)) {
            position.naive_get_utf8_column()
        } else {
            0
        };
        let fragment = if position.is_empty() {
            String::new()
        } else {
            let line = if cfg!(not(miri)) {
                String::from_utf8(position.get_line_beginning().to_vec())
                    .expect("input is valid UTF-8")
            } else {
                String::new()
            };
            format!("\"{line}\"\n{}^", "-".repeat(3 + column))
        };

        LocatedParseError {
            source: self,
            line: position.location_line(),
            column,
            fragment,
            context: Vec::new(),
        }
    }
}
