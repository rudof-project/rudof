//! The shortcuts, read as the steps the spec says they abbreviate.
//!
//! `shortcut ::= ("@" | "~" | "$") iri`

use super::lexical::{iri, token};
use crate::compact::{IRes, Span};
use crate::shexpath::{Attribute, AttributeOrAny, Axis, Comparator, Filter, Operand, RValue, ShExPath, Step, TermType};
use nom::{Parser, branch::alt, combinator::map, sequence::preceded};
use prefixmap::IriRef;

/// `shortcut ::= ("@" | "~" | "$") iri`
pub(super) fn shortcut(i: Span) -> IRes<Vec<Step>> {
    alt((
        map(preceded(token("@"), iri), shape_label_shortcut),
        map(preceded(token("~"), iri), predicate_shortcut),
        map(preceded(token("$"), iri), triple_expr_label_shortcut),
    ))
    .parse(i)
}

/// `@label` is `/shapes/*[id = label][assert count() = 1]/shapeExpr`.
fn shape_label_shortcut(label: IriRef) -> Vec<Step> {
    vec![
        Step::child(AttributeOrAny::Attribute(Attribute::Shapes)),
        Step::child(AttributeOrAny::Any)
            .with_filter(id_is(label))
            .with_filter(exactly_one()),
        Step::child(AttributeOrAny::Attribute(Attribute::ShapeExpr)),
    ]
}

/// `~iri` is `/thisShapeExpr::Shape/expression/thisTripleExpr::TripleConstraint[predicate = iri]`.
fn predicate_shortcut(predicate: IriRef) -> Vec<Step> {
    vec![
        Step::axis(Axis::ThisShapeExpr).with_filter(Filter::TermType(TermType::Shape)),
        Step::child(AttributeOrAny::Attribute(Attribute::Expression)),
        Step::axis(Axis::ThisTripleExpr)
            .with_filter(Filter::TermType(TermType::TripleConstraint))
            .with_filter(attribute_is(Attribute::Predicate, predicate)),
    ]
}

/// `$label` is `/descendant::[id = label][assert count() = 1]`.
fn triple_expr_label_shortcut(label: IriRef) -> Vec<Step> {
    vec![
        Step::axis(Axis::Descendant)
            .with_filter(id_is(label))
            .with_filter(exactly_one()),
    ]
}

/// `[id = label]`
fn id_is(label: IriRef) -> Filter {
    attribute_is(Attribute::Id, label)
}

/// `[attribute = iri]`
fn attribute_is(attribute: Attribute, iri: IriRef) -> Filter {
    Filter::compare(
        Operand::Path(ShExPath::path([Step::child(AttributeOrAny::Attribute(attribute))])),
        Comparator::Equal,
        RValue::Iri(iri),
    )
}

/// `[assert count() = 1]`
fn exactly_one() -> Filter {
    Filter::compare(Operand::Count, Comparator::Equal, RValue::Integer(1)).asserted()
}
