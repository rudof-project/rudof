//! Parsing of ShExPath junctions and parenthesized steps.

use super::parser_helpers::*;
use crate::{Attribute, Filter, Operand, Selector, ShExPath, ShExPathParser, Step, TermType};

#[test]
fn a_comma_makes_a_sequence() {
    assert_eq!(parse("shapes , start"), ShExPath::Sequence(vec![a(), b()]));
    assert_eq!(parse("shapes,start,imports"), ShExPath::Sequence(vec![a(), b(), c()]));
}

#[test]
fn union_and_intersection_are_matched_without_regard_to_case() {
    for src in ["shapes union start", "shapes UNION start", "shapes Union start"] {
        assert_eq!(parse(src), ShExPath::Union(vec![a(), b()]), "{src}");
    }
    for src in ["shapes intersection start", "shapes INTERSECTION start"] {
        assert_eq!(parse(src), ShExPath::Intersection(vec![a(), b()]), "{src}");
    }
}

#[test]
fn junctions_take_any_number_of_operands() {
    assert_eq!(
        parse("shapes union start union imports"),
        ShExPath::Union(vec![a(), b(), c()])
    );
}

#[test]
fn intersection_binds_tighter_than_union_and_union_than_comma() {
    assert_eq!(
        parse("shapes intersection start union imports intersection startActs"),
        ShExPath::Union(vec![
            ShExPath::Intersection(vec![a(), b()]),
            ShExPath::Intersection(vec![c(), d()])
        ])
    );
    assert_eq!(
        parse("shapes , start union imports intersection startActs"),
        ShExPath::Sequence(vec![
            a(),
            ShExPath::Union(vec![b(), ShExPath::Intersection(vec![c(), d()])])
        ])
    );
}

#[test]
fn junctions_of_shortcuts() {
    assert_eq!(
        resolve("@<#IssueShape>~:name , @<#IssueShape>~:category"),
        ShExPath::Sequence(vec![
            resolve("@<#IssueShape>~:name"),
            resolve("@<#IssueShape>~:category")
        ])
    );
    assert_eq!(
        resolve("@<#IssueShape>~:name union @<#IssueShape>~:name"),
        ShExPath::Union(vec![resolve("@<#IssueShape>~:name"), resolve("@<#IssueShape>~:name")])
    );
}

#[test]
fn a_filter_may_hold_a_junction() {
    assert_eq!(
        parse("/*[shapes , start]"),
        ShExPath::path([any().with_filter(Filter::test(Operand::Path(ShExPath::Sequence(vec![a(), b()]))))])
    );
}

#[test]
fn a_union_keyword_is_a_whole_word() {
    assert!(ShExPathParser::parse_syntax("shapes unionstart").is_err());
}

#[test]
fn a_parenthesized_path_is_a_step() {
    assert_eq!(
        parse("(shapes/*)"),
        ShExPath::path([Step::new(Selector::Path(Box::new(ShExPath::path([
            child(Attribute::Shapes),
            any()
        ]))))])
    );
    assert_eq!(
        parse("/shapes/(* union shapes)Shape[0]"),
        ShExPath::path([
            child(Attribute::Shapes),
            Step::new(Selector::Path(Box::new(ShExPath::Union(vec![
                ShExPath::path([any()]),
                a()
            ]))))
            .with_filter(Filter::TermType(TermType::Shape))
            .with_filter(Filter::index(0))
        ])
    );
}

#[test]
fn iris_in_parenthesized_paths_and_junctions_are_resolved() {
    let expected = compares(Attribute::Id, iri("http://ex.example/schema#S"));
    assert_eq!(
        resolve("(/*[id = <#S>])"),
        ShExPath::path([Step::new(Selector::Path(Box::new(expected.clone())))])
    );
    assert_eq!(
        resolve("/*[id = <#S>] union /*[id = <#S>]"),
        ShExPath::Union(vec![expected.clone(), expected])
    );
}

#[test]
fn a_malformed_junction_is_an_error() {
    for src in ["shapes union", ", shapes", "shapes ,", "(shapes", "shapes)", "()"] {
        assert!(ShExPathParser::parse_syntax(src).is_err(), "{src}");
    }
}

#[test]
fn errors_name_the_unexpected_input() {
    let err = ShExPathParser::parse_syntax("/shapes asdf").unwrap_err();
    assert!(err.to_string().contains("asdf"), "{err}");
    let err = ShExPathParser::parse_syntax("/shapes %").unwrap_err();
    assert!(err.to_string().contains('%'), "{err}");
}
