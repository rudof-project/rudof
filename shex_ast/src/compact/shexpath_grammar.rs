//! Grammar of [ShExPath](https://shexspec.github.io/spec/ShExPath#grammar).
//!
//! Each parser skips the whitespace and comments before its token, so a whole path is
//! `terminated(shape_path, ws)`. Keywords are whole words: a word is read and then looked up, so
//! `valueExpr` is never read as `value` followed by something else.

use crate::compact::grammar::map_error;
use crate::compact::{IRes, Span};
use crate::shex_parser_error::ParseError;
use crate::shexpath::{Attribute, AttributeOrAny, ShExPath, Step};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while1},
    character::complete::{char, multispace1},
    combinator::{map, map_opt, opt, recognize, value},
    multi::many0,
    sequence::{pair, preceded},
};

/// `shapePath ::= sequenceStep ("," sequenceStep)*`
pub(crate) fn shape_path(i: Span) -> IRes<ShExPath> {
    intersection_step(i)
}

/// `intersectionStep ::= startStep nextStep*`
fn intersection_step(i: Span) -> IRes<ShExPath> {
    map((start_step, many0(next_step)), |(first, rest)| {
        ShExPath::path(first.into_iter().chain(rest.into_iter().flatten()))
    })
    .parse(i)
}

/// `startStep ::= ("/" | "//")? step`
///
/// The separator has no effect on where the path begins.
fn start_step(i: Span) -> IRes<Vec<Step>> {
    map(preceded(opt(separator), step), |step| vec![step]).parse(i)
}

/// `nextStep ::= ("/" | "//") step`
fn next_step(i: Span) -> IRes<Vec<Step>> {
    map(preceded(separator, step), |step| vec![step]).parse(i)
}

/// `"/" | "//"`, which currently mean the same thing.
fn separator(i: Span) -> IRes<Span> {
    alt((token("//"), token("/"))).parse(i)
}

/// `step ::= "child::"? attributeOrAny`
fn step(i: Span) -> IRes<Step> {
    map(preceded(opt(token("child::")), attribute_or_any), Step::child).parse(i)
}

/// `attributeOrAny ::= "*" | attribute`
fn attribute_or_any(i: Span) -> IRes<AttributeOrAny> {
    alt((
        value(AttributeOrAny::Any, token("*")),
        map(attribute, AttributeOrAny::Attribute),
    ))
    .parse(i)
}

/// `attribute ::= "type" | "id" | ...`, see [`Attribute`].
fn attribute(i: Span) -> IRes<Attribute> {
    map_error(map_opt(word, |w| Attribute::from_keyword(w.fragment())), || {
        ParseError::Custom {
            msg: "Expected a ShExPath attribute".to_string(),
        }
    })(i)
}

/// A word, the unit keywords are read in: letters, digits, `_` and `-`, optionally after an `@`
/// (for `@context`).
fn word(i: Span) -> IRes<Span> {
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
fn token<'a>(token: &'static str) -> impl FnMut(Span<'a>) -> IRes<'a, Span<'a>> {
    map_error(preceded(ws, tag(token)), move || {
        ParseError::ExpectedToken(token.to_string())
    })
}

/// Whitespace and comments, skipped between tokens.
///
/// `COMMENT ::= '#' [^#x0a#x0d]* | '<!--' ([^-] | '-' [^-] | '--' [^>])* '-->'`
///
/// ShExC's `/* */` comments are not ShExPath comments: `/*` is a separator and a star.
pub(crate) fn ws(i: Span) -> IRes<()> {
    value((), many0(alt((value((), multispace1), line_comment, xml_comment)))).parse(i)
}

fn line_comment(i: Span) -> IRes<()> {
    value((), pair(char('#'), opt(is_not("\n\r")))).parse(i)
}

fn xml_comment(i: Span) -> IRes<()> {
    value((), (tag("<!--"), take_until("-->"), tag("-->"))).parse(i)
}
