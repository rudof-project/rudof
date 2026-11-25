//! Parsing of ShExPath filters.

use super::parser_helpers::*;
use crate::{Attribute, Axis, Comparator, Filter, Operand, RValue, ShExPath, ShExPathParser, Step, TermType};

#[test]
fn a_number_selects_by_index() {
    assert_eq!(
        parse("/shapes/*[0]"),
        ShExPath::path([child(Attribute::Shapes), any().with_filter(Filter::index(0))])
    );
}

#[test]
fn a_number_is_shorthand_for_index() {
    assert_eq!(parse("/expressions/*[1]"), parse("/expressions/*[index() = 1]"));
    assert_eq!(
        parse("/expressions/*[ index ( ) = 1 ]"),
        parse("/expressions/*[index()=1]")
    );
}

#[test]
fn a_number_may_have_a_sign() {
    assert_eq!(parse("/*[-1]"), ShExPath::path([any().with_filter(Filter::index(-1))]));
    assert_eq!(parse("/*[+2]"), ShExPath::path([any().with_filter(Filter::index(2))]));
}

#[test]
fn functions_may_be_tested_alone_or_compared() {
    assert_eq!(
        parse("/*[count() = 3]"),
        ShExPath::path([any().with_filter(Filter::compare(Operand::Count, Comparator::Equal, RValue::Integer(3)))])
    );
    assert_eq!(
        parse("/*[count()]"),
        ShExPath::path([any().with_filter(Filter::test(Operand::Count))])
    );
    assert_eq!(
        parse("/*[index()]"),
        ShExPath::path([any().with_filter(Filter::test(Operand::Index))])
    );
}

#[test]
fn functions_are_case_sensitive() {
    ShExPathParser::parse_syntax("/*[INDEX() = 1]").unwrap_err();
    ShExPathParser::parse_syntax("/*[Count()]").unwrap_err();
}

#[test]
fn assert_is_matched_without_regard_to_case() {
    let expected = ShExPath::path([
        any().with_filter(Filter::compare(Operand::Count, Comparator::Equal, RValue::Integer(4)).asserted())
    ]);
    assert_eq!(parse("/*[assert count() = 4]"), expected);
    assert_eq!(parse("/*[ASSERT count() = 4]"), expected);
    assert_eq!(parse("/*[Assert count() = 4]"), expected);
}

#[test]
fn a_path_may_be_compared_with_an_iri() {
    assert_eq!(
        parse("/*[predicate = :name]"),
        ShExPath::path([any().with_filter(Filter::compare(
            path_of(Attribute::Predicate),
            Comparator::Equal,
            pname("", "name")
        ))])
    );
    assert_eq!(
        parse("/*[id = <http://a.example/S1>]"),
        ShExPath::path([any().with_filter(Filter::compare(
            path_of(Attribute::Id),
            Comparator::Equal,
            iri("http://a.example/S1")
        ))])
    );
}

#[test]
fn a_path_may_be_tested_alone() {
    assert_eq!(
        parse("/*[valueExpr]"),
        ShExPath::path([any().with_filter(Filter::test(path_of(Attribute::ValueExpr)))])
    );
    assert_eq!(
        parse("/*[assert valueExpr]"),
        ShExPath::path([any().with_filter(Filter::test(path_of(Attribute::ValueExpr)).asserted())])
    );
    assert_eq!(
        parse("/*[valueExpr/expression]"),
        ShExPath::path([any().with_filter(Filter::test(Operand::Path(ShExPath::path([
            child(Attribute::ValueExpr),
            child(Attribute::Expression)
        ]))))])
    );
}

#[test]
fn every_comparator_is_parsed() {
    assert_eq!(
        parse("/*[min < 3]"),
        ShExPath::path([any().with_filter(Filter::compare(
            path_of(Attribute::Min),
            Comparator::LessThan,
            RValue::Integer(3)
        ))])
    );
    assert_eq!(
        parse("/*[max > 1]"),
        ShExPath::path([any().with_filter(Filter::compare(
            path_of(Attribute::Max),
            Comparator::GreaterThan,
            RValue::Integer(1)
        ))])
    );
}

#[test]
fn filters_apply_in_order() {
    assert_eq!(
        parse("/*[1][0]"),
        ShExPath::path([any().with_filter(Filter::index(1)).with_filter(Filter::index(0))])
    );
}

#[test]
fn filters_follow_the_term_type() {
    assert_eq!(
        parse("thisTripleExpr::TripleConstraint[predicate = <http://a.example/p>]"),
        ShExPath::path([
            of_type(Step::axis(Axis::ThisTripleExpr), TermType::TripleConstraint).with_filter(Filter::compare(
                path_of(Attribute::Predicate),
                Comparator::Equal,
                iri("http://a.example/p")
            ))
        ])
    );
    assert_eq!(
        parse("/expressions/TripleConstraint[0]"),
        ShExPath::path([
            child(Attribute::Expressions),
            of_type(any(), TermType::TripleConstraint).with_filter(Filter::index(0))
        ])
    );
}

#[test]
fn an_axis_may_have_filters() {
    assert_eq!(
        parse("/self::[2]"),
        ShExPath::path([Step::axis(Axis::Itself).with_filter(Filter::index(2))])
    );
}

#[test]
fn a_malformed_filter_is_an_error() {
    for src in [
        "/*[",
        "/*[]",
        "/*[0",
        "/*[assert]",
        "/*[count() =]",
        "/*[id = ]",
        "/*[=1]",
    ] {
        assert!(ShExPathParser::parse_syntax(src).is_err(), "{src}");
    }
}
