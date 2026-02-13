use crate::shapemap::{NodeSelector, Pattern, SHACLPathRef, ShapeSelector};
use crate::{
    IRes, ParseError, Span,
    compact::grammar::{map_error, tag_no_case_tws, token_tws, traced, tws0},
    compact::shex_grammar::shape_expr_label,
    iri, literal,
};
use crate::{ObjectValue, string};
use iri_s::IriS;
use nom::bytes::complete::tag;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::tuple,
};
use prefixmap::IriRef;
use srdf::RDF_TYPE_STR;

#[derive(Debug, PartialEq)]
pub(crate) enum ShapeMapStatement {
    Association {
        node_selector: NodeSelector,
        shape_selector: ShapeSelector,
    },
}

pub(crate) fn shapemap_statement<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Vec<ShapeMapStatement>> {
    traced(
        "shapemap_statement",
        map_error(
            move |i| {
                let (i, (a, _, ass, _, _)) =
                    all_consuming(tuple((association, tws0, rest_associations, tws0, opt(char(',')))))(i)?;
                let mut rs = vec![a];
                for a in ass {
                    rs.push(a);
                }
                Ok((i, rs))
            },
            || ParseError::ExpectedShapeMapAssociation,
        ),
    )
}

/// `association ::= node_spec @ shape_spec`
fn association(i: Span) -> IRes<ShapeMapStatement> {
    let (i, (ns, _, sl)) = tuple((node_selector(), token_tws("@"), shape_spec()))(i)?;
    let s = ShapeMapStatement::Association {
        node_selector: ns,
        shape_selector: sl,
    };
    Ok((i, s))
}

fn rest_associations(i: Span) -> IRes<Vec<ShapeMapStatement>> {
    let (i, ass) = many0(tuple((token_tws(","), tws0, association)))(i)?;
    let r = ass.into_iter().map(|(_, _, a)| a).collect();
    Ok((i, r))
}

pub(crate) fn shape_spec<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShapeSelector> {
    traced(
        "shape_spec",
        map_error(
            move |i| {
                alt((
                    map(shape_expr_label, ShapeSelector::Label),
                    map(tag_no_case_tws("START"), |_| ShapeSelector::Start),
                ))(i)
            },
            || ParseError::ExpectedShapeSpec,
        ),
    )
}

/// nodeSelector     : objectTerm | triplePattern | extended ;
pub(crate) fn node_selector<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, NodeSelector> {
    traced(
        "node_selector",
        map_error(
            move |i| alt((object_term, triple_pattern, extended))(i),
            || ParseError::ExpectedNodeSpec,
        ),
    )
}

fn object_term(i: Span) -> IRes<NodeSelector> {
    alt((subject_term, literal_selector))(i)
}

fn object_value<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ObjectValue> {
    move |i| alt((map(iri, ObjectValue::iri_ref), map(literal(), ObjectValue::literal)))(i)
}

fn triple_pattern(i: Span) -> IRes<NodeSelector> {
    let (i, (_, _, triple, _, _)) = tuple((open_curly, tws0, triple_pattern_inner, tws0, close_curly))(i)?;
    Ok((i, triple))
}

fn triple_pattern_inner(i: Span) -> IRes<NodeSelector> {
    alt((focus_object, subject_focus))(i)
}

fn node_or_wildcard(i: Span) -> IRes<Pattern> {
    alt((
        map(object_value(), Pattern::Node),
        map(token_tws("_"), |_| Pattern::wildcard()),
    ))(i)
}

fn focus_object(i: Span) -> IRes<NodeSelector> {
    let (i, (_, _, path, _, pattern)) = tuple((focus, tws0, shacl_path, tws0, node_or_wildcard))(i)?;
    Ok((
        i,
        NodeSelector::TriplePattern {
            subject: Pattern::Focus,
            path,
            object: pattern,
        },
    ))
}

fn subject_focus(i: Span) -> IRes<NodeSelector> {
    let (i, (pattern, _, path, _, _)) = tuple((node_or_wildcard, tws0, shacl_path, tws0, focus))(i)?;
    Ok((
        i,
        NodeSelector::TriplePattern {
            subject: pattern,
            path,
            object: Pattern::Focus,
        },
    ))
}

fn shacl_path(i: Span) -> IRes<SHACLPathRef> {
    map(predicate, SHACLPathRef::predicate)(i)
}

fn predicate(i: Span) -> IRes<IriRef> {
    alt((iri, rdf_type))(i)
}

fn rdf_type(i: Span) -> IRes<IriRef> {
    let (i, _) = tag("a")(i)?;
    let rdf_type: IriRef = IriRef::iri(IriS::new_unchecked(RDF_TYPE_STR));
    Ok((i, rdf_type))
}

fn focus(i: Span) -> IRes<Pattern> {
    let (i, _) = tag_no_case_tws("FOCUS")(i)?;
    Ok((i, Pattern::Focus))
}

fn extended(i: Span) -> IRes<NodeSelector> {
    let (i, (_keyword, _, query)) = tuple((tag_no_case_tws("SPARQL"), tws0, string()))(i)?;
    Ok((i, NodeSelector::Sparql { query }))
}

fn subject_term(i: Span) -> IRes<NodeSelector> {
    let (i, iri) = iri(i)?;
    Ok((i, NodeSelector::iri_ref(iri)))
}

fn literal_selector(i: Span) -> IRes<NodeSelector> {
    let (i, lit) = literal()(i)?;
    Ok((i, NodeSelector::literal(lit)))
}

fn open_curly(i: Span) -> IRes<char> {
    char('{')(i)
}

fn close_curly(i: Span) -> IRes<char> {
    char('}')(i)
}

#[cfg(test)]
mod tests {
    use prefixmap::IriRef;
    use srdf::rdf_type;

    use crate::shapemap::ShapeSelector;

    use super::*;

    #[test]
    fn example_shapemap() {
        let input = Span::new(":a@:label");
        let (_, shape_map) = association(input).unwrap();
        let expected = ShapeMapStatement::Association {
            node_selector: NodeSelector::prefixed("", "a"),
            shape_selector: ShapeSelector::prefixed("", "label"),
        };
        assert_eq!(shape_map, expected);
    }

    #[test]
    fn example_shapemap_sparql() {
        let query = r#""""foo""""#;
        let str = format!("SPARQL {query}@:label");
        println!("Str: {str}");
        let input = Span::new(str.as_str());
        let (_, shape_map) = association(input).unwrap();
        let expected = ShapeMapStatement::Association {
            node_selector: NodeSelector::Sparql {
                query: "foo".to_string(),
            },
            shape_selector: ShapeSelector::prefixed("", "label"),
        };
        assert_eq!(shape_map, expected);
    }

    #[test]
    fn shapemap_triple_pattern() {
        let input = Span::new("{ FOCUS a :Person }@:label");
        let (_, shape_map) = association(input).unwrap();
        let expected = ShapeMapStatement::Association {
            node_selector: NodeSelector::triple_pattern(
                Pattern::focus(),
                SHACLPathRef::predicate(IriRef::iri(rdf_type().clone())),
                Pattern::prefixed("", "Person"),
            ),
            shape_selector: ShapeSelector::prefixed("", "label"),
        };
        assert_eq!(shape_map, expected);
    }

    #[test]
    fn test_triple_pattern() {
        let input = Span::new("{ FOCUS a :Person }");
        let (_, tp) = triple_pattern(input).unwrap();
        let expected = NodeSelector::triple_pattern(
            Pattern::focus(),
            SHACLPathRef::predicate(IriRef::iri(rdf_type().clone())),
            Pattern::prefixed("", "Person"),
        );
        assert_eq!(tp, expected);
    }

    #[test]
    fn test_triple_pattern_inner() {
        let input = Span::new("FOCUS a :Person");
        let (_, value) = triple_pattern_inner(input).unwrap();
        let expected = NodeSelector::triple_pattern(
            Pattern::focus(),
            SHACLPathRef::predicate(IriRef::iri(rdf_type().clone())),
            Pattern::prefixed("", "Person"),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn test_focus_object() {
        let input = Span::new("FOCUS a :Person");
        let (_, value) = focus_object(input).unwrap();
        let expected = NodeSelector::triple_pattern(
            Pattern::focus(),
            SHACLPathRef::predicate(IriRef::iri(rdf_type().clone())),
            Pattern::prefixed("", "Person"),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn test_focus() {
        let input = Span::new("FOCUS");
        let (_, value) = focus(input).unwrap();
        let expected = Pattern::focus();
        assert_eq!(value, expected);
    }

    /*    #[test_log::test]
    fn example_shapemap_failed () {
        let input = Span::new("\n @START \n # Comment \n@STRT\n");
        let shape_map = parse_shapemap(input).unwrap();
        let expected = InputShapeMap::new();
        assert_eq!(shape_map, expected);
    } */
}
