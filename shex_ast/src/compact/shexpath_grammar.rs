//! Grammar of [ShExPath](https://shexspec.github.io/spec/ShExPath#grammar).
//!
//! Each parser skips the whitespace and comments before its token, so a whole path is
//! `terminated(shape_path, ws)`. Keywords are whole words: a word is read and then looked up, so
//! `valueExpr` is never read as `value` followed by something else.

use crate::compact::grammar::map_error;
use crate::compact::shex_grammar::iri;
use crate::compact::{IRes, Span};
use crate::shex_parser_error::ParseError;
use crate::shexpath::{
    Attribute, AttributeOrAny, Axis, Comparator, Comparison, Filter, Operand, RValue, ShExPath, Step, TermType,
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while1},
    character::complete::{char, digit1, multispace1, one_of},
    combinator::{map, map_opt, map_res, opt, recognize, value, verify},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
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

/// ```text
/// step ::= "child::"? termType filter*
///        | "child::"? attributeOrAny termType? filter*
///        | nonChildAxis termType? filter*
/// ```
fn step(i: Span) -> IRes<Step> {
    alt((
        map(
            (non_child_axis, opt(term_type), many0(filter)),
            |(axis, term_type, filters)| with_filters(Step::axis(axis), term_type, filters),
        ),
        map(
            (preceded(opt(token("child::")), term_type), many0(filter)),
            |(term_type, filters)| with_filters(Step::child(AttributeOrAny::Any), Some(term_type), filters),
        ),
        map(
            (
                preceded(opt(token("child::")), attribute_or_any),
                opt(term_type),
                many0(filter),
            ),
            |(attribute, term_type, filters)| with_filters(Step::child(attribute), term_type, filters),
        ),
    ))
    .parse(i)
}

/// A term type is the step's first filter, before those in brackets.
fn with_filters(step: Step, term_type: Option<TermType>, filters: Vec<Filter>) -> Step {
    term_type
        .map(Filter::TermType)
        .into_iter()
        .chain(filters)
        .fold(step, Step::with_filter)
}

/// `filter ::= "[" filterExpr "]"`
fn filter(i: Span) -> IRes<Filter> {
    delimited(token("["), filter_expr, token("]")).parse(i)
}

/// ```text
/// filterExpr ::= "assert"? shapePath comparison?
///              | "assert"? function comparison?
///              | numericExpr
/// ```
///
/// `numericExpr ::= INTEGER` is read as `index() = INTEGER`, of which it is shorthand.
fn filter_expr(i: Span) -> IRes<Filter> {
    alt((
        map(
            (opt(assert), function, opt(comparison)),
            |(assert, operand, comparison)| Filter::Test {
                assert: assert.is_some(),
                operand,
                comparison,
            },
        ),
        map(
            (opt(assert), shape_path, opt(comparison)),
            |(assert, path, comparison)| Filter::Test {
                assert: assert.is_some(),
                operand: Operand::Path(path),
                comparison,
            },
        ),
        map(integer, Filter::index),
    ))
    .parse(i)
}

/// `"assert"`, matched without regard to case.
fn assert(i: Span) -> IRes<Span> {
    keyword_no_case("assert")(i)
}

/// `function ::= "index" "(" ")" | "count" "(" ")"`
fn function(i: Span) -> IRes<Operand> {
    terminated(
        alt((
            value(Operand::Index, keyword("index")),
            value(Operand::Count, keyword("count")),
        )),
        (token("("), token(")")),
    )
    .parse(i)
}

/// `comparison ::= comparitor rvalue`
fn comparison(i: Span) -> IRes<Comparison> {
    map((comparator, rvalue), |(comparator, rvalue)| Comparison {
        comparator,
        rvalue,
    })
    .parse(i)
}

/// `comparitor ::= "=" | "<" | ">"`
fn comparator(i: Span) -> IRes<Comparator> {
    alt((
        value(Comparator::Equal, token("=")),
        value(Comparator::LessThan, token("<")),
        value(Comparator::GreaterThan, token(">")),
    ))
    .parse(i)
}

/// `rvalue ::= INTEGER | iri`
fn rvalue(i: Span) -> IRes<RValue> {
    alt((map(integer, RValue::Integer), map(preceded(ws, iri), RValue::Iri))).parse(i)
}

/// `[19t] INTEGER ::= [+-]? [0-9]+`
fn integer(i: Span) -> IRes<i64> {
    map_error(
        map_res(preceded(ws, recognize(pair(opt(one_of("+-")), digit1))), |n: Span| {
            n.fragment().trim_start_matches('+').parse::<i64>()
        }),
        || ParseError::Integer,
    )(i)
}

/// A case sensitive keyword, which is a whole word.
fn keyword<'a>(keyword: &'static str) -> impl FnMut(Span<'a>) -> IRes<'a, Span<'a>> {
    map_error(verify(word, move |w: &Span| *w.fragment() == keyword), move || {
        ParseError::ExpectedToken(keyword.to_string())
    })
}

/// A keyword matched without regard to case, which is a whole word.
fn keyword_no_case<'a>(keyword: &'static str) -> impl FnMut(Span<'a>) -> IRes<'a, Span<'a>> {
    map_error(
        verify(word, move |w: &Span| w.fragment().eq_ignore_ascii_case(keyword)),
        move || ParseError::ExpectedToken(keyword.to_string()),
    )
}

/// `nonChildAxis ::= "thisShapeExpr::" | "thisTripleExpr::" | "self::" | "parent::" | "ancestor::"`
fn non_child_axis(i: Span) -> IRes<Axis> {
    alt((
        value(Axis::ThisShapeExpr, token("thisShapeExpr::")),
        value(Axis::ThisTripleExpr, token("thisTripleExpr::")),
        value(Axis::Itself, token("self::")),
        value(Axis::Parent, token("parent::")),
        value(Axis::Ancestor, token("ancestor::")),
    ))
    .parse(i)
}

/// `termType ::= shapeExprType | tripleExprType | valueType | "Schema" | "SemAct" | "Annotation"`,
/// see [`TermType`].
fn term_type(i: Span) -> IRes<TermType> {
    map_error(map_opt(word, |w| TermType::from_keyword(w.fragment())), || {
        ParseError::Custom {
            msg: "Expected a ShExPath term type".to_string(),
        }
    })(i)
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
