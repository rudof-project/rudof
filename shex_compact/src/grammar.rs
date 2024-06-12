use crate::{shex_parser_error::ParseError as ShExParseError, IRes, Span};
use colored::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::multispace1,
    combinator::value,
    multi::many0,
    sequence::{delimited, pair},
    Err,
};
use std::fmt::Debug;

// Create a [`Span`][nom_locate::LocatedSpan] over the input.
/* fn span_from_str(input: &str) -> Span<'_> {
    Span::new(input)
}*/

// The result of a parse
// type ParseResult<'a, T> = Result<T, LocatedParseError>;

/// A combinator that modifies the associated error.
pub(crate) fn map_error<'a, T: 'a>(
    mut parser: impl FnMut(Span<'a>) -> IRes<'a, T> + 'a,
    mut error: impl FnMut() -> ShExParseError + 'a,
) -> impl FnMut(Span<'a>) -> IRes<'a, T> + 'a {
    move |input| {
        parser(input).map_err(|e| match e {
            Err::Incomplete(_) => e,
            Err::Error(context) => {
                let mut err = error().at(input);
                err.append(context);
                Err::Error(err)
            }
            Err::Failure(context) => {
                let mut err = error().at(input);
                err.append(context);
                Err::Failure(err)
            }
        })
    }
}

/// A combinator to add tracing to the parser.
/// [fun] is an identifier for the parser and [parser] is the actual parser.
#[inline(always)]
pub(crate) fn traced<'a, T, P>(
    fun: &'static str,
    mut parser: P,
) -> impl FnMut(Span<'a>) -> IRes<'a, T>
where
    T: Debug,
    P: FnMut(Span<'a>) -> IRes<'a, T>,
{
    move |input| {
        tracing::trace!(target: "parser", "{fun}({input:?})");
        let result = parser(input);
        match &result {
            Ok(res) => {
                tracing::trace!(target: "parser", "{}", format!("{fun}({input:?}) -> {res:?}").green());
            }
            Err(e) => {
                tracing::trace!(target: "parser", "{}", format!("{fun}({input:?}) -> {e:?}").red());
            }
        }
        result
    }
}

/// A combinator that recognises a comment, starting at a `#`
/// character and ending at the end of the line.
fn comment(input: Span) -> IRes<()> {
    alt((
        value((), pair(tag("#"), is_not("\n\r"))),
        // a comment that immediately precedes the end of the line –
        // this must come after the normal line comment above
        value((), tag("#")),
        value((), multi_comment),
    ))(input)
}

fn multi_comment(i: Span) -> IRes<()> {
    value((), delimited(tag("/*"), is_not("*/"), tag("*/")))(i)
}

/// A combinator that recognises an arbitrary amount of whitespace and
/// comments.
pub(crate) fn tws0(input: Span) -> IRes<()> {
    value((), many0(alt((value((), multispace1), comment))))(input)
}

/*
/// A combinator that recognises any non-empty amount of whitespace
/// and comments.
pub(crate) fn tws1(input: Span) -> IRes<()> {
    value((), many1(alt((value((), multispace1), comment))))(input)
}
*/

/// A combinator that creates a parser for a specific token.
pub(crate) fn token<'a>(token: &'a str) -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    map_error(tag(token), || {
        ShExParseError::ExpectedToken(token.to_string())
    })
}

/// A combinator that creates a parser for a specific token,
/// surrounded by trailing whitespace or comments.
pub(crate) fn token_tws<'a>(token: &'a str) -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    map_error(delimited(tws0, tag(token), tws0), || {
        ShExParseError::ExpectedToken(token.to_string())
    })
}

/// A combinator that creates a parser for a case insensitive tag,
/// surrounded by trailing whitespace or comments.
pub(crate) fn tag_no_case_tws<'a>(token: &'a str) -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    map_error(delimited(tws0, tag_no_case(token), tws0), || {
        ShExParseError::ExpectedToken(token.to_string())
    })
}
