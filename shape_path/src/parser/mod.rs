//! Parser of [ShExPath](https://shexspec.github.io/spec/ShExPath) expressions.

mod filters;
mod iri;
mod junctions;
mod lexical;
pub(crate) mod shortcuts;
mod span;
mod steps;

use crate::ShExPath;
use crate::error::ShExPathParseError as ParseError;
use junctions::shape_path;
use lexical::ws;
use nom::{Err, Parser, sequence::terminated};
use prefixmap::{DerefIri, PrefixMap};
use rudof_iri::IriS;
use span::Span;

type Result<A> = std::result::Result<A, ParseError>;

/// Reads ShExPath expressions.
pub struct ShExPathParser;

impl ShExPathParser {
    /// Parses `src`, which must be a whole ShExPath, with whitespace and comments allowed around it.
    ///
    /// The IRIs in the path are resolved: an `IRIREF` against `base` and a prefixed name with
    /// `prefixmap`. A prefix that `prefixmap` does not declare is an error.
    pub fn parse(src: &str, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<ShExPath> {
        Ok(Self::parse_syntax(src)?.deref_iri(base, prefixmap)?)
    }

    /// Parses `src`, leaving its IRIs as they are written.
    pub(crate) fn parse_syntax(src: &str) -> Result<ShExPath> {
        let (rest, path) = terminated(shape_path, ws).parse(Span::new(src)).map_err(|e| match e {
            Err::Incomplete(needed) => ParseError::Custom {
                msg: format!("Incomplete ShExPath: needed {needed:?}"),
            },
            Err::Error(e) | Err::Failure(e) => ParseError::NomError { err: Box::new(e) },
        })?;
        if rest.is_empty() {
            Ok(path)
        } else {
            Err(ParseError::TrailingInput(rest.fragment().to_string()))
        }
    }
}
