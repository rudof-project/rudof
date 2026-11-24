//! The input of the parser and the result of each of its parsers.

use crate::error::{LocatedParseError, ShExPathParseError};
use nom::{Err, IResult, Parser};
use nom_locate::LocatedSpan;

/// A [`LocatedSpan`] over the path.
pub(crate) type Span<'a> = LocatedSpan<&'a str>;

/// What a parser returns.
pub(crate) type IRes<'a, T> = IResult<Span<'a>, T, LocatedParseError>;

/// `parser`, with the error it fails with replaced by `error`, which keeps the original as context.
pub(crate) fn map_error<'a, T: 'a>(
    mut parser: impl Parser<Span<'a>, Output = T, Error = LocatedParseError> + 'a,
    mut error: impl FnMut() -> ShExPathParseError + 'a,
) -> impl FnMut(Span<'a>) -> IRes<'a, T> + 'a {
    move |input| {
        parser.parse(input).map_err(|e| match e {
            Err::Incomplete(_) => e,
            Err::Error(context) => {
                let mut err = error().at(input);
                err.append(context);
                Err::Error(err)
            },
            Err::Failure(context) => {
                let mut err = error().at(input);
                err.append(context);
                Err::Failure(err)
            },
        })
    }
}
