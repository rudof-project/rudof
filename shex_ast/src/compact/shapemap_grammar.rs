use crate::shapemap::{NodeSelector, ShapeSelector};
use crate::{
    IRes, ParseError, Span,
    compact::grammar::{map_error, tag_no_case_tws, token_tws, traced, tws0},
    compact::shex_grammar::shape_expr_label,
    iri, literal,
};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::tuple,
};

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
                let (i, (a, _, ass, _, _)) = all_consuming(tuple((
                    association,
                    tws0,
                    rest_associations,
                    tws0,
                    opt(char(',')),
                )))(i)?;
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
        map_error(move |i| object_term(i), || ParseError::ExpectedNodeSpec),
    )
}

fn object_term(i: Span) -> IRes<NodeSelector> {
    alt((subject_term, literal_selector))(i)
}

fn subject_term(i: Span) -> IRes<NodeSelector> {
    let (i, iri) = iri(i)?;
    Ok((i, NodeSelector::iri_ref(iri)))
}

fn literal_selector(i: Span) -> IRes<NodeSelector> {
    let (i, lit) = literal()(i)?;
    Ok((i, NodeSelector::literal(lit)))
}

#[cfg(test)]
mod tests {
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

    /*    #[test_log::test]
    fn example_shapemap_failed () {
        let input = Span::new("\n @START \n # Comment \n@STRT\n");
        let shape_map = parse_shapemap(input).unwrap();
        let expected = InputShapeMap::new();
        assert_eq!(shape_map, expected);
    } */
}
