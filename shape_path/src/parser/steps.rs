//! A step: an axis, a term type and filters.
//!
//! ```text
//! step           ::= "child::"? termType filter*
//!                  | "child::"? attributeOrAny termType? filter*
//!                  | nonChildAxis termType? filter*
//!                  | "(" shapePath ")" termType? filter*
//! nonChildAxis   ::= "thisShapeExpr::" | "thisTripleExpr::" | "self::" | "parent::" | "ancestor::"
//! attributeOrAny ::= "*" | attribute
//! ```

use super::filters::filter;
use super::junctions::shape_path;
use super::lexical::{token, word};
use super::span::map_error;
use super::span::{IRes, Span};
use crate::error::ShExPathParseError as ParseError;
use crate::{Attribute, AttributeOrAny, Axis, Filter, Selector, Step, TermType};
use nom::{
    Parser,
    branch::alt,
    combinator::{map, map_opt, opt, value},
    multi::many0,
    sequence::{delimited, preceded},
};

/// ```text
/// step ::= "child::"? termType filter*
///        | "child::"? attributeOrAny termType? filter*
///        | nonChildAxis termType? filter*
///        | "(" shapePath ")" termType? filter*
/// ```
pub(super) fn step(i: Span) -> IRes<Step> {
    alt((
        map(
            (
                delimited(token("("), shape_path, token(")")),
                opt(term_type),
                many0(filter),
            ),
            |(path, term_type, filters)| with_filters(Step::new(Selector::Path(Box::new(path))), term_type, filters),
        ),
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
