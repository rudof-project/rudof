//! Resolution of the IRIs a ShExPath is written with.

use super::parser_helpers::*;
use crate::{Attribute, Comparator, Filter, Operand, ShExPath, ShExPathParser};

#[test]
fn a_relative_iri_is_resolved_against_the_base() {
    assert_eq!(
        resolve("/*[id = <#IssueShape>]"),
        compares(Attribute::Id, iri("http://ex.example/schema#IssueShape"))
    );
}

#[test]
fn an_absolute_iri_is_kept() {
    assert_eq!(
        resolve("/*[id = <http://a.example/S1>]"),
        compares(Attribute::Id, iri("http://a.example/S1"))
    );
}

#[test]
fn a_prefixed_name_is_resolved_with_the_prefixes() {
    assert_eq!(
        resolve("/*[predicate = :name]"),
        compares(Attribute::Predicate, iri("http://ex.example/ns#name"))
    );
    assert_eq!(
        resolve("/*[predicate = foaf:mbox]"),
        compares(Attribute::Predicate, iri("http://xmlns.com/foaf/0.1/mbox"))
    );
    assert_eq!(
        resolve("/*[predicate = foaf:]"),
        compares(Attribute::Predicate, iri("http://xmlns.com/foaf/0.1/"))
    );
}

#[test]
fn iris_in_nested_filters_are_resolved() {
    assert_eq!(
        resolve("/*[expressions/*[predicate = :name]]"),
        ShExPath::path([any().with_filter(Filter::test(Operand::Path(ShExPath::path([
            child(Attribute::Expressions),
            any().with_filter(Filter::compare(
                path_of(Attribute::Predicate),
                Comparator::Equal,
                iri("http://ex.example/ns#name")
            ))
        ]))))])
    );
}

#[test]
fn an_unknown_prefix_is_an_error() {
    let prefixmap = example_prefixmap();
    let err = ShExPathParser::parse("/*[predicate = pre999:lname]", None, Some(&prefixmap)).unwrap_err();
    assert!(err.to_string().contains("pre999"), "{err}");
}

#[test]
fn a_prefixed_name_without_prefixes_is_an_error() {
    assert!(ShExPathParser::parse("/*[predicate = :name]", None, None).is_err());
}

#[test]
fn a_relative_iri_without_a_base_is_kept() {
    assert_eq!(
        ShExPathParser::parse("/*[id = <#S>]", None, None).expect("parses"),
        compares(Attribute::Id, iri("#S"))
    );
}
