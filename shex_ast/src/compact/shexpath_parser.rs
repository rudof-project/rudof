//! Parser of [ShExPath](https://shexspec.github.io/spec/ShExPath) expressions.

use crate::compact::shexpath_grammar::{shape_path, ws};
use crate::shexpath::ShExPath;
use crate::{ParseError, Span};
use nom::{Err, Parser, sequence::terminated};

type Result<A> = std::result::Result<A, ParseError>;

/// Reads ShExPath expressions.
pub struct ShExPathParser;

impl ShExPathParser {
    /// Parses `src`, which must be a whole ShExPath, with whitespace and comments allowed around it.
    pub fn parse(src: &str) -> Result<ShExPath> {
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
    use crate::shexpath::{Attribute, AttributeOrAny, Step};

    fn child(attribute: Attribute) -> Step {
        Step::child(AttributeOrAny::Attribute(attribute))
    }

    fn any() -> Step {
        Step::child(AttributeOrAny::Any)
    }

    fn parse(src: &str) -> ShExPath {
        match ShExPathParser::parse(src) {
            Ok(path) => path,
            Err(e) => panic!("{src} is not parsed: {e}"),
        }
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
        assert!(ShExPathParser::parse("/Shapes").is_err());
        assert!(ShExPathParser::parse("/VALUEEXPR").is_err());
    }

    #[test]
    fn a_word_that_is_not_an_attribute_is_an_error() {
        assert!(ShExPathParser::parse("/asdf").is_err());
        assert!(ShExPathParser::parse("/abstract").is_err());
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
        assert!(ShExPathParser::parse("").is_err());
        assert!(ShExPathParser::parse("/").is_err());
    }

    #[test]
    fn trailing_input_is_an_error() {
        let err = ShExPathParser::parse("/shapes ]").unwrap_err();
        assert!(err.to_string().contains("Unexpected trailing input: ]"), "{err}");
    }
}
