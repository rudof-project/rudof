//! The terminals of the ShExPath grammar, and the whitespace and comments between tokens.
//!
//! Each parser skips the whitespace and comments before its token, so a whole path is
//! `terminated(shape_path, ws)`. Keywords are whole words: a word is read and then looked up, so
//! `valueExpr` is never read as `value` followed by something else.

use crate::compact::grammar::map_error;
use crate::compact::shex_grammar;
use crate::compact::{IRes, Span};
use crate::shex_parser_error::ParseError;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while1},
    character::complete::{char, digit1, multispace1, one_of},
    combinator::{map_res, opt, recognize, value, verify},
    multi::many0,
    sequence::{pair, preceded},
};
use prefixmap::IriRef;

/// `[136s] iri ::= IRIREF | prefixedName`, as written: it is resolved once the path is parsed.
pub(super) fn iri(i: Span) -> IRes<IriRef> {
    preceded(ws, shex_grammar::iri).parse(i)
}

/// `[19t] INTEGER ::= [+-]? [0-9]+`
pub(super) fn integer(i: Span) -> IRes<i64> {
    map_error(
        map_res(preceded(ws, recognize(pair(opt(one_of("+-")), digit1))), |n: Span| {
            n.fragment().trim_start_matches('+').parse::<i64>()
        }),
        || ParseError::Integer,
    )(i)
}

/// A case sensitive keyword, which is a whole word.
pub(super) fn keyword<'a>(keyword: &'static str) -> impl FnMut(Span<'a>) -> IRes<'a, Span<'a>> {
    map_error(verify(word, move |w: &Span| *w.fragment() == keyword), move || {
        ParseError::ExpectedToken(keyword.to_string())
    })
}

/// A keyword matched without regard to case, which is a whole word.
pub(super) fn keyword_no_case<'a>(keyword: &'static str) -> impl FnMut(Span<'a>) -> IRes<'a, Span<'a>> {
    map_error(
        verify(word, move |w: &Span| w.fragment().eq_ignore_ascii_case(keyword)),
        move || ParseError::ExpectedToken(keyword.to_string()),
    )
}

/// A word, the unit keywords are read in: letters, digits, `_` and `-`, optionally after an `@`
/// (for `@context`).
pub(super) fn word(i: Span) -> IRes<Span> {
    preceded(
        ws,
        recognize(pair(
            opt(char('@')),
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
        )),
    )
    .parse(i)
}

/// A token, after any whitespace and comments.
pub(super) fn token<'a>(token: &'static str) -> impl FnMut(Span<'a>) -> IRes<'a, Span<'a>> {
    map_error(preceded(ws, tag(token)), move || {
        ParseError::ExpectedToken(token.to_string())
    })
}

/// Whitespace and comments, skipped between tokens.
///
/// `COMMENT ::= '#' [^#x0a#x0d]* | '<!--' ([^-] | '-' [^-] | '--' [^>])* '-->'`
///
/// ShExC's `/* */` comments are not ShExPath comments: `/*` is a separator and a star.
pub(super) fn ws(i: Span) -> IRes<()> {
    value((), many0(alt((value((), multispace1), line_comment, xml_comment)))).parse(i)
}

fn line_comment(i: Span) -> IRes<()> {
    value((), pair(char('#'), opt(is_not("\n\r")))).parse(i)
}

fn xml_comment(i: Span) -> IRes<()> {
    value((), (tag("<!--"), take_until("-->"), tag("-->"))).parse(i)
}
