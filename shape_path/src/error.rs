use nom::error::{ErrorKind, FromExternalError};
use nom_locate::LocatedSpan;
use prefixmap::error::DerefError;
use std::num::ParseIntError;
use thiserror::Error;

/// An error evaluating a ShExPath.
#[derive(Error, Debug)]
pub enum ShExPathError {
    /// The path is not a ShExPath.
    #[error("Error parsing the ShExPath: {0}")]
    Parse(#[from] ShExPathParseError),

    /// The schema could not be written as ShExJ, the document a path is evaluated over.
    ///
    /// # Fields
    /// - `msg`: Description of the serialization failure
    #[error("Error writing the schema as ShExJ: {msg}")]
    Serialization { msg: String },

    /// A filter `[assert expr]` whose `expr` does not hold: the path is in error.
    ///
    /// # Fields
    /// - `location`: Where the item the assertion failed on is
    #[error("ShExPath assertion failed on the item at \"{location}\"")]
    AssertionFailed { location: String },

    /// The path uses something the evaluator does not evaluate.
    ///
    /// # Fields
    /// - `feature`: What is not evaluated
    #[error("ShExPath evaluation of {feature} is not implemented")]
    NotImplemented { feature: String },
}

/// An error reading a ShExPath.
///
/// The messages are those of the ShExC parser, whose terminals ShExPath shares.
#[derive(Error, Debug)]
pub enum ShExPathParseError {
    /// The parser failed at a position of the path.
    #[error("Parsing error: {err}")]
    NomError { err: Box<LocatedParseError> },

    #[error("{msg}")]
    Custom { msg: String },

    /// A prefixed name whose prefix is not declared, or an IRI that cannot be resolved.
    #[error(transparent)]
    DerefError {
        #[from]
        err: DerefError,
    },

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Unexpected trailing input: {0}")]
    TrailingInput(String),

    #[error(r#"Expected "{0}""#)]
    ExpectedToken(String),

    #[error("Expected prefixed name")]
    ExpectedPrefixedName,

    #[error("Expected integer")]
    Integer,

    #[error("Parse int error for str {str}: {err} ")]
    ParseIntError { str: String, err: ParseIntError },
}

impl ShExPathParseError {
    /// This error at `position`.
    pub(crate) fn at(self, position: LocatedSpan<&str>) -> LocatedParseError {
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
                String::from_utf8(position.get_line_beginning().to_vec()).expect("input is valid UTF-8")
            } else {
                String::new()
            };
            format!("\"{line}\"\n{}^", "-".repeat(3 + column))
        };
        LocatedParseError {
            source: Box::new(self),
            line: position.location_line(),
            column,
            fragment,
            context: Vec::new(),
        }
    }
}

/// A [`ShExPathParseError`] at a position of the path, with the errors that led to it.
#[derive(Debug, Error)]
#[error("Parse error on line {}, column {}: {}\nat {}{}",
  .line, .column,
  .source,
  .fragment,
  format_parse_error_context(.context))]
pub struct LocatedParseError {
    #[source]
    pub source: Box<ShExPathParseError>,
    pub line: u32,
    pub column: usize,
    pub fragment: String,
    pub context: Vec<LocatedParseError>,
}

impl LocatedParseError {
    /// Adds `other` as context of this error.
    pub(crate) fn append(&mut self, other: LocatedParseError) {
        self.context.push(other);
    }
}

fn format_parse_error_context(context: &[LocatedParseError]) -> String {
    let fragments: Vec<String> = context
        .iter()
        .flat_map(|error| {
            error
                .to_string()
                .split('\n')
                .map(|line| format!("  {line}"))
                .collect::<Vec<_>>()
        })
        .collect();
    if fragments.is_empty() {
        String::new()
    } else {
        format!("\nContext:\n{}", fragments.join("\n"))
    }
}

impl nom::error::ParseError<LocatedSpan<&str>> for LocatedParseError {
    fn from_error_kind(input: LocatedSpan<&str>, kind: ErrorKind) -> Self {
        ShExPathParseError::SyntaxError(kind.description().to_string()).at(input)
    }

    fn append(input: LocatedSpan<&str>, kind: ErrorKind, other: Self) -> Self {
        let mut error = ShExPathParseError::SyntaxError(kind.description().to_string()).at(input);
        error.append(other);
        error
    }
}

impl FromExternalError<LocatedSpan<&str>, ParseIntError> for LocatedParseError {
    fn from_external_error(input: LocatedSpan<&str>, _kind: ErrorKind, e: ParseIntError) -> Self {
        ShExPathParseError::ParseIntError {
            str: input.fragment().to_string(),
            err: e,
        }
        .at(input)
    }
}
