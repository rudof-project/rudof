//! Parsing of the ShExPath shortcuts `@`, `~` and `$` as their long forms.

use super::parser_helpers::*;
use crate::{Attribute, Axis, Comparator, Filter, Operand, RValue, ShExPath, ShExPathParser, Step};

#[test]
fn at_is_the_shape_declared_with_that_label() {
    assert_eq!(
        resolve("@<#IssueShape>"),
        resolve("/shapes/*[id = <#IssueShape>][assert count() = 1]/shapeExpr")
    );
}

#[test]
fn tilde_is_the_triple_constraint_on_that_predicate() {
    assert_eq!(
        resolve("~:category"),
        resolve("/thisShapeExpr::Shape/expression/thisTripleExpr::TripleConstraint[predicate = :category]")
    );
}

#[test]
fn dollar_is_the_triple_expression_declared_with_that_label() {
    let label = iri("http://ex.example/schema#reportedBy");
    assert_eq!(
        resolve("$<#reportedBy>"),
        ShExPath::path([Step::axis(Axis::Descendant)
            .with_filter(Filter::compare(path_of(Attribute::Id), Comparator::Equal, label))
            .with_filter(Filter::compare(Operand::Count, Comparator::Equal, RValue::Integer(1)).asserted())])
    );
}

#[test]
fn a_shortcut_needs_no_slash_before_it() {
    assert_eq!(
        resolve("@<#IssueShape>~:category"),
        resolve(
            "@<#IssueShape>/thisShapeExpr::Shape/expression\n  \
             /thisTripleExpr::TripleConstraint[predicate = :category]"
        )
    );
}

#[test]
fn a_shortcut_cannot_follow_a_slash() {
    ShExPathParser::parse_syntax("@<S>/~<p>").unwrap_err();
    ShExPathParser::parse_syntax("/shapes/@<S>").unwrap_err();
}

#[test]
fn a_shortcut_takes_no_filters() {
    ShExPathParser::parse_syntax("@<S>~<p>[2]").unwrap_err();
    ShExPathParser::parse_syntax("@<S>[0]").unwrap_err();
}

#[test]
fn a_shortcut_may_be_followed_by_steps() {
    assert_eq!(
        resolve("$<#reportedBy>/valueExpr"),
        ShExPath::path(
            steps_of(resolve("$<#reportedBy>"))
                .into_iter()
                .chain([child(Attribute::ValueExpr)])
        )
    );
}

#[test]
fn the_sigil_and_the_iri_are_separate_tokens() {
    assert_eq!(resolve("@ <#IssueShape>"), resolve("@<#IssueShape>"));
}

#[test]
fn at_context_is_still_an_attribute() {
    assert_eq!(parse("@context"), ShExPath::path([child(Attribute::Context)]));
    assert_eq!(parse("/@context"), ShExPath::path([child(Attribute::Context)]));
    assert_eq!(
        resolve("@:context"),
        resolve("/shapes/*[id = :context][assert count() = 1]/shapeExpr")
    );
}

#[test]
fn a_blank_node_label_cannot_follow_a_sigil() {
    ShExPathParser::parse_syntax("@_:b1").unwrap_err();
}
