//! A filter of a step.
//!
//! ```text
//! filter      ::= "[" filterExpr "]"
//! filterExpr  ::= "assert"? shapePath comparison?
//!               | "assert"? function comparison?
//!               | numericExpr
//! function    ::= "index" "(" ")" | "count" "(" ")"
//! comparison  ::= comparitor rvalue
//! comparitor  ::= "=" | "<" | ">"
//! rvalue      ::= INTEGER | iri
//! numericExpr ::= INTEGER
//! ```

use super::junctions::shape_path;
use super::lexical::{integer, iri, keyword, keyword_no_case, token};
use super::span::{IRes, Span};
use crate::{Comparator, Comparison, Filter, Operand, RValue};
use nom::{
    Parser,
    branch::alt,
    combinator::{map, opt, value},
    sequence::{delimited, terminated},
};

/// `filter ::= "[" filterExpr "]"`
pub(super) fn filter(i: Span<'_>) -> IRes<'_, Filter> {
    delimited(token("["), filter_expr, token("]")).parse(i)
}

/// ```text
/// filterExpr ::= "assert"? shapePath comparison?
///              | "assert"? function comparison?
///              | numericExpr
/// ```
///
/// `numericExpr ::= INTEGER` is read as `index() = INTEGER`, of which it is shorthand.
fn filter_expr(i: Span<'_>) -> IRes<'_, Filter> {
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
fn assert(i: Span<'_>) -> IRes<'_, Span<'_>> {
    keyword_no_case("assert")(i)
}

/// `function ::= "index" "(" ")" | "count" "(" ")"`
fn function(i: Span<'_>) -> IRes<'_, Operand> {
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
fn comparison(i: Span<'_>) -> IRes<'_, Comparison> {
    map((comparator, rvalue), |(comparator, rvalue)| Comparison {
        comparator,
        rvalue,
    })
    .parse(i)
}

/// `comparitor ::= "=" | "<" | ">"`
fn comparator(i: Span<'_>) -> IRes<'_, Comparator> {
    alt((
        value(Comparator::Equal, token("=")),
        value(Comparator::LessThan, token("<")),
        value(Comparator::GreaterThan, token(">")),
    ))
    .parse(i)
}

/// `rvalue ::= INTEGER | iri`
fn rvalue(i: Span<'_>) -> IRes<'_, RValue> {
    alt((map(integer, RValue::Integer), map(iri, RValue::Iri))).parse(i)
}
