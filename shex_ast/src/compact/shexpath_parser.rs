//! Parser of [ShExPath](https://shexspec.github.io/spec/ShExPath) expressions.

use crate::compact::shexpath_grammar::{shape_path, ws};
use crate::shexpath::ShExPath;
use crate::{ParseError, Span};
use nom::{Err, Parser, sequence::terminated};
use prefixmap::{DerefIri, PrefixMap};
use rudof_iri::IriS;

type Result<A> = std::result::Result<A, ParseError>;

/// Reads ShExPath expressions.
pub struct ShExPathParser;

impl ShExPathParser {
    /// Parses `src`, which must be a whole ShExPath, with whitespace and comments allowed around it.
    ///
    /// The IRIs in the path are resolved: an `IRIREF` against `base` and a prefixed name with
    /// `prefixmap`. A prefix that `prefixmap` does not declare is an error.
    pub fn parse(src: &str, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<ShExPath> {
        Ok(Self::parse_syntax(src)?.deref_iri(base, prefixmap)?)
    }

    /// Parses `src`, leaving its IRIs as they are written.
    pub(crate) fn parse_syntax(src: &str) -> Result<ShExPath> {
        let (rest, path) = terminated(shape_path, ws).parse(Span::new(src)).map_err(|e| match e {
            Err::Incomplete(needed) => ParseError::Custom {
                msg: format!("Incomplete ShExPath: needed {needed:?}"),
            },
            Err::Error(e) | Err::Failure(e) => ParseError::NomError { err: Box::new(e) },
        })?;
        if rest.is_empty() {
            Ok(path)
        } else {
            Err(ParseError::TrailingInput(rest.fragment().to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shexpath::{Attribute, AttributeOrAny, Axis, Comparator, Filter, Operand, RValue, Step, TermType};
    use prefixmap::IriRef;
    use rudof_iri::IriS;

    fn child(attribute: Attribute) -> Step {
        Step::child(AttributeOrAny::Attribute(attribute))
    }

    fn any() -> Step {
        Step::child(AttributeOrAny::Any)
    }

    fn parse(src: &str) -> ShExPath {
        match ShExPathParser::parse_syntax(src) {
            Ok(path) => path,
            Err(e) => panic!("{src} is not parsed: {e}"),
        }
    }

    fn of_type(step: Step, term_type: TermType) -> Step {
        step.with_filter(Filter::TermType(term_type))
    }

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
        assert!(ShExPathParser::parse_syntax("/self").is_err());
        assert!(ShExPathParser::parse_syntax("/parent").is_err());
        assert!(ShExPathParser::parse_syntax("/thisShapeExpr:").is_err());
    }

    #[test]
    fn descendant_has_no_spelling() {
        assert!(ShExPathParser::parse_syntax("/descendant::").is_err());
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

    fn path_of(attribute: Attribute) -> Operand {
        Operand::Path(ShExPath::path([child(attribute)]))
    }

    fn iri(s: &str) -> RValue {
        RValue::Iri(IriRef::iri(IriS::new_unchecked(s)))
    }

    fn pname(prefix: &str, local: &str) -> RValue {
        RValue::Iri(IriRef::prefixed(prefix, local))
    }

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
        assert!(ShExPathParser::parse_syntax("/*[INDEX() = 1]").is_err());
        assert!(ShExPathParser::parse_syntax("/*[Count()]").is_err());
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

    fn example_prefixmap() -> PrefixMap {
        let mut prefixmap = PrefixMap::new();
        prefixmap.add_prefix("", IriS::new_unchecked("http://ex.example/ns#"));
        prefixmap.add_prefix("foaf", IriS::new_unchecked("http://xmlns.com/foaf/0.1/"));
        prefixmap
    }

    fn example_base() -> IriS {
        IriS::new_unchecked("http://ex.example/schema")
    }

    fn resolve(src: &str) -> ShExPath {
        let prefixmap = example_prefixmap();
        match ShExPathParser::parse(src, Some(&example_base()), Some(&prefixmap)) {
            Ok(path) => path,
            Err(e) => panic!("{src} is not parsed: {e}"),
        }
    }

    fn compares(attribute: Attribute, rvalue: RValue) -> ShExPath {
        ShExPath::path([any().with_filter(Filter::compare(path_of(attribute), Comparator::Equal, rvalue))])
    }

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
        assert!(ShExPathParser::parse_syntax("@<S>/~<p>").is_err());
        assert!(ShExPathParser::parse_syntax("/shapes/@<S>").is_err());
    }

    #[test]
    fn a_shortcut_takes_no_filters() {
        assert!(ShExPathParser::parse_syntax("@<S>~<p>[2]").is_err());
        assert!(ShExPathParser::parse_syntax("@<S>[0]").is_err());
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
        assert!(ShExPathParser::parse_syntax("@_:b1").is_err());
    }

    fn steps_of(path: ShExPath) -> Vec<Step> {
        match path {
            ShExPath::Path(steps) => steps,
        }
    }

    #[test]
    fn shape_decl_is_not_a_term_type() {
        assert!(ShExPathParser::parse_syntax("/ShapeDecl").is_err());
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
        assert!(ShExPathParser::parse_syntax("/Shapes").is_err());
        assert!(ShExPathParser::parse_syntax("/VALUEEXPR").is_err());
    }

    #[test]
    fn a_word_that_is_not_an_attribute_is_an_error() {
        assert!(ShExPathParser::parse_syntax("/asdf").is_err());
        assert!(ShExPathParser::parse_syntax("/abstract").is_err());
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
        assert!(ShExPathParser::parse_syntax("").is_err());
        assert!(ShExPathParser::parse_syntax("/").is_err());
    }

    #[test]
    fn trailing_input_is_an_error() {
        let err = ShExPathParser::parse_syntax("/shapes ]").unwrap_err();
        assert!(err.to_string().contains("Unexpected trailing input: ]"), "{err}");
    }
}
