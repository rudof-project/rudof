use crate::{shex_parser_error::ParseError as ShExParseError, Span};
use colored::*;
use log;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::multispace1,
    combinator::{cut, map, map_res, opt, recognize, value},
    error::{ErrorKind, FromExternalError},
    error_position,
    multi::{count, fold_many0, many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    Err, IResult, InputTake,
};
use std::{fmt::Debug, num::{ParseIntError, ParseFloatError}};
use thiserror::Error;

use crate::grammar_structs::{
    Cardinality, NumericLength, NumericRange, Qualifier, SenseFlags, ShExStatement,
};

use nom_locate::LocatedSpan;
use prefixmap::IriRef;
use srdf::{lang::Lang, literal::Literal, numeric_literal::NumericLiteral};

/// A [`ParseError`] at a certain location
#[derive(Debug, Error)]
#[error("Parse error on line {}, column {}: {}\nat {}{}", 
  .line, .column, 
  .source, 
  .fragment, 
  format_parse_error_context(.context))]
pub struct LocatedParseError {
    #[source]
    pub source: ShExParseError,
    pub line: u32,
    pub column: usize,
    pub fragment: String,
    pub context: Vec<LocatedParseError>,
}

impl LocatedParseError {
    /// Append another [`LocatedParseError`] as context to this error.
    pub(crate) fn append(&mut self, other: LocatedParseError) {
        self.context.push(other)
    }
}

pub(crate) fn format_parse_error_context(context: &[LocatedParseError]) -> String {
    let mut fragments = Vec::new();

    for error in context {
        let error_string = format!("{error}");
        for line in error_string.split('\n') {
            fragments.push(format!("{}{line}", " ".repeat(2)));
        }
    }

    if fragments.is_empty() {
        String::new()
    } else {
        format!("\nContext:\n{}", fragments.join("\n"))
    }
}

impl nom::error::ParseError<Span<'_>> for LocatedParseError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        ShExParseError::SyntaxError(kind.description().to_string()).at(input)
    }

    fn append(input: Span, kind: ErrorKind, other: Self) -> Self {
        let mut error = ShExParseError::SyntaxError(kind.description().to_string()).at(input);
        error.append(other);
        error
    }
}

impl FromExternalError<Span<'_>, ParseIntError> for LocatedParseError {
    fn from_external_error(input: Span, _kind: ErrorKind, e: ParseIntError) -> Self {
        ShExParseError::ParseIntError {
            str: input.fragment().to_string(),
            err: e,
        }
        .at(input)
    }
}

impl FromExternalError<Span<'_>, ParseFloatError> for LocatedParseError {
    fn from_external_error(input: Span, _kind: ErrorKind, e: ParseFloatError) -> Self {
        ShExParseError::ParseFloatError {
            str: input.fragment().to_string(),
            err: e,
        }
        .at(input)
    }
}
