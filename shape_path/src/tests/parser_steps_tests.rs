//! Parsing of ShExPath steps: child steps, attributes, axes and term types, and the tokens between them.

use super::parser_helpers::*;
use crate::{Attribute, Axis, ShExPath, ShExPathParser, Step, TermType};

#[test]
fn every_non_child_axis_is_a_step() {
    for (axis, spelling) in Axis::SPELLED {
        let src = format!("/shapes/{spelling}");
        assert_eq!(
            parse(&src),
            ShExPath::path([child(Attribute::Shapes), Step::axis(*axis)]),
            "{src}"
        );
    }
}

#[test]
fn an_axis_may_start_a_path() {
    assert_eq!(parse("self::"), ShExPath::path([Step::axis(Axis::Itself)]));
    assert_eq!(parse("/self::"), ShExPath::path([Step::axis(Axis::Itself)]));
}

#[test]
fn an_axis_needs_its_double_colon() {
    ShExPathParser::parse_syntax("/self").unwrap_err();
    ShExPathParser::parse_syntax("/parent").unwrap_err();
    ShExPathParser::parse_syntax("/thisShapeExpr:").unwrap_err();
}

#[test]
fn descendant_has_no_spelling() {
    ShExPathParser::parse_syntax("/descendant::").unwrap_err();
}

#[test]
fn an_axis_may_have_a_term_type() {
    assert_eq!(
        parse("thisShapeExpr::Shape"),
        ShExPath::path([of_type(Step::axis(Axis::ThisShapeExpr), TermType::Shape)])
    );
    assert_eq!(
        parse("/expression/thisTripleExpr::TripleConstraint"),
        ShExPath::path([
            child(Attribute::Expression),
            of_type(Step::axis(Axis::ThisTripleExpr), TermType::TripleConstraint)
        ])
    );
}

#[test]
fn a_term_type_alone_is_a_child_step_over_any_attribute() {
    let expected = ShExPath::path([
        child(Attribute::Expressions),
        of_type(any(), TermType::TripleConstraint),
    ]);
    assert_eq!(parse("/expressions/TripleConstraint"), expected);
    assert_eq!(parse("/expressions/child::TripleConstraint"), expected);
    assert_eq!(parse("/expressions/*TripleConstraint"), expected);
}

#[test]
fn an_attribute_may_have_a_term_type() {
    assert_eq!(
        parse("/valueExpr Shape"),
        ShExPath::path([of_type(child(Attribute::ValueExpr), TermType::Shape)])
    );
}

#[test]
fn every_term_type_of_the_grammar_is_a_step() {
    for term_type in TermType::ALL {
        let src = format!("/{term_type}");
        assert_eq!(parse(&src), ShExPath::path([of_type(any(), *term_type)]), "{src}");
    }
}

#[test]
fn the_grammar_has_every_term_type_of_the_spec() {
    let spec = "ShapeAnd ShapeOr ShapeNot NodeConstraint Shape ShapeExternal EachOf OneOf TripleConstraint \
                IriStem IriStemRange LiteralStem LiteralStemRange Language LanguageStem LanguageStemRange \
                Wildcard Schema SemAct Annotation";
    let names: Vec<&str> = spec.split_whitespace().collect();
    assert_eq!(names.len(), TermType::ALL.len(), "number of term types");
    for name in names {
        assert!(TermType::from_keyword(name).is_some(), "{name} is not a term type");
    }
}

#[test]
fn a_term_type_is_the_whole_word() {
    for (src, term_type) in [
        ("/ShapeExternal", TermType::ShapeExternal),
        ("/Shape", TermType::Shape),
        ("/IriStemRange", TermType::IriStemRange),
        ("/LanguageStemRange", TermType::LanguageStemRange),
        ("/LiteralStemRange", TermType::LiteralStemRange),
    ] {
        assert_eq!(parse(src), ShExPath::path([of_type(any(), term_type)]), "{src}");
    }
}

#[test]
fn term_types_and_attributes_differ_in_case() {
    assert_eq!(parse("/language"), ShExPath::path([child(Attribute::Language)]));
    assert_eq!(parse("/Language"), ShExPath::path([of_type(any(), TermType::Language)]));
}

#[test]
fn shape_decl_is_not_a_term_type() {
    ShExPathParser::parse_syntax("/ShapeDecl").unwrap_err();
}

#[test]
fn child_steps_are_separated_by_slashes() {
    let expected = ShExPath::path([child(Attribute::Shapes), any()]);
    assert_eq!(parse("/shapes/*"), expected);
}

#[test]
fn the_leading_slash_is_optional() {
    let expected = ShExPath::path([child(Attribute::Shapes), any()]);
    assert_eq!(parse("shapes/*"), expected);
}

#[test]
fn double_slash_is_accepted_wherever_slash_is() {
    let expected = ShExPath::path([child(Attribute::Shapes), any()]);
    assert_eq!(parse("//shapes//*"), expected);
}

#[test]
fn the_child_axis_may_be_written_out() {
    let expected = ShExPath::path([child(Attribute::Shapes), any()]);
    assert_eq!(parse("/child::shapes/child::*"), expected);
}

#[test]
fn every_attribute_of_the_grammar_is_a_child_step() {
    for attribute in Attribute::ALL {
        let src = format!("/{attribute}");
        assert_eq!(parse(&src), ShExPath::path([child(*attribute)]), "{src}");
    }
}

#[test]
fn the_grammar_has_every_attribute_of_the_spec() {
    let spec = "type id semActs annotations predicate @context startActs start imports shapes \
                shapeExprs shapeExpr nodeKind datatype values length minlength maxlength pattern flags \
                mininclusive minexclusive maxinclusive maxexclusive totaldigits fractiondigits \
                value language stem exclusions languageTag closed extra extends expression \
                expressions min max inverse valueExpr name code object";
    let names: Vec<&str> = spec.split_whitespace().collect();
    assert_eq!(names.len(), Attribute::ALL.len(), "number of attributes");
    for name in names {
        assert!(Attribute::from_keyword(name).is_some(), "{name} is not an attribute");
    }
}

#[test]
fn an_attribute_is_the_whole_word() {
    for (src, attribute) in [
        ("/valueExpr", Attribute::ValueExpr),
        ("/value", Attribute::Value),
        ("/expressions", Attribute::Expressions),
        ("/expression", Attribute::Expression),
        ("/languageTag", Attribute::LanguageTag),
        ("/shapeExprs", Attribute::ShapeExprs),
        ("/startActs", Attribute::StartActs),
        ("/minlength", Attribute::MinLength),
    ] {
        assert_eq!(parse(src), ShExPath::path([child(attribute)]), "{src}");
    }
}

#[test]
fn attributes_are_case_sensitive() {
    ShExPathParser::parse_syntax("/Shapes").unwrap_err();
    ShExPathParser::parse_syntax("/VALUEEXPR").unwrap_err();
}

#[test]
fn a_word_that_is_not_an_attribute_is_an_error() {
    ShExPathParser::parse_syntax("/asdf").unwrap_err();
    ShExPathParser::parse_syntax("/abstract").unwrap_err();
}

#[test]
fn whitespace_and_comments_are_skipped_between_tokens() {
    let expected = ShExPath::path([child(Attribute::Shapes), any()]);
    for src in [
        " / shapes / * ",
        "/shapes # the declarations\n/*",
        "<!-- the declarations --> /shapes/*",
        "/shapes/* # trailing comment",
        "/shapes\n\t/*",
    ] {
        assert_eq!(parse(src), expected, "{src:?}");
    }
}

#[test]
fn slash_star_is_not_a_comment() {
    let expected = ShExPath::path([child(Attribute::Expressions), any(), any()]);
    assert_eq!(parse("expressions/*/*"), expected);
}

#[test]
fn an_empty_path_is_an_error() {
    ShExPathParser::parse_syntax("").unwrap_err();
    ShExPathParser::parse_syntax("/").unwrap_err();
}

#[test]
fn trailing_input_is_an_error() {
    let err = ShExPathParser::parse_syntax("/shapes ]").unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input: ]"), "{err}");
}
