use crate::LocatedParseError;
use crate::ParseError;
use crate::Span;
use crate::compact::shapemap_grammar::ShapeMapStatement;
use crate::compact::shapemap_grammar::shapemap_statement;
use crate::compact::shapemap_grammar::{node_selector, shape_spec};
use crate::compact::shex_grammar::iri;
use crate::shapemap::NodeSelector;
use crate::shapemap::ShapeSelector;
use crate::shapemap::query_shape_map::QueryShapeMap;
use crate::tws0;
use nom::Err;
use nom::Parser;
use nom::sequence::delimited;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use std::fs;
use std::path::Path;
// use tracing::debug;
// use tracing::trace;

type Result<A> = std::result::Result<A, ParseError>;

pub struct ShapeMapParser<'a> {
    shapemap_statement_iterator: ShapeMapStatementIterator<'a>,
}

impl ShapeMapParser<'_> {
    /// Parse a ShapeMap that uses [ShapeMap compact syntax](https://shexspec.github.io/shape-map/#grammar)
    ///
    pub fn parse(
        src: &str,
        nodes_prefixmap: &Option<PrefixMap>,
        base_nodes: &Option<IriS>,
        shapes_prefixmap: &Option<PrefixMap>,
        base_shapes: &Option<IriS>,
    ) -> Result<QueryShapeMap> {
        // trace!("Parsing shapemap with src: {src} and base_nodes: {base_nodes:?} and base_shapes: {base_shapes:?}");
        let mut query_shapemap = QueryShapeMap::new();
        if let Some(pm) = nodes_prefixmap {
            query_shapemap = query_shapemap.with_nodes_prefixmap(pm)
        };
        if let Some(pm) = shapes_prefixmap {
            query_shapemap = query_shapemap.with_shapes_prefixmap(pm)
        };
        let mut parser = ShapeMapParser {
            shapemap_statement_iterator: ShapeMapStatementIterator::new(Span::new(src))?,
        };
        for ss in parser.shapemap_statement_iterator.by_ref() {
            let statements = ss?;
            for s in statements {
                match s {
                    ShapeMapStatement::Association {
                        node_selector,
                        shape_selector,
                    } => {
                        // tracing::debug!("Association {node_selector}@{shape_selector}");
                        query_shapemap.add_association(node_selector, base_nodes, shape_selector, base_shapes)?;
                    },
                }
            }
        }
        Ok(query_shapemap)
    }

    pub fn parse_buf(
        path: &Path,
        nodes_prefixmap: &Option<PrefixMap>,
        base_nodes: &Option<IriS>,
        shapes_prefixmap: &Option<PrefixMap>,
        base_shapes: &Option<IriS>,
    ) -> Result<QueryShapeMap> {
        let data = fs::read_to_string(path)?;
        let query_shapemap = ShapeMapParser::parse(&data, nodes_prefixmap, base_nodes, shapes_prefixmap, base_shapes)?;
        Ok(query_shapemap)
    }

    pub fn parse_shape_selector(str: &str) -> Result<ShapeSelector> {
        let span = Span::new(str);
        let ss = parse_whole(span, shape_spec()).map_err(|e| match e {
            Err::Incomplete(s) => ParseError::Custom {
                msg: format!("Incomplete input: needed {s:?}"),
            },
            Err::Error(e) => ParseError::NomError { err: Box::new(e) },
            Err::Failure(f) => ParseError::NomError { err: Box::new(f) },
        })?;
        Ok(ss)
    }

    pub fn parse_node_selector(str: &str) -> Result<NodeSelector> {
        let span = Span::new(str);
        let ns = parse_whole(span, node_selector()).map_err(|e| match e {
            Err::Incomplete(s) => ParseError::Custom {
                msg: format!("Incomplete input parsing node selector {str}: needed {s:?}"),
            },
            Err::Error(e) => ParseError::NodeSelectorNomError {
                str: str.to_string(),
                err: Box::new(e),
            },
            Err::Failure(f) => ParseError::NodeSelectorNomError {
                str: str.to_string(),
                err: Box::new(f),
            },
        })?;
        Ok(ns)
    }

    pub fn parse_iri_ref(str: &str) -> Result<IriRef> {
        let span = Span::new(str);
        let iri_ref = parse_whole(span, iri).map_err(|e| match e {
            Err::Incomplete(s) => ParseError::Custom {
                msg: format!("Incomplete input: needed {s:?}"),
            },
            Err::Error(e) => ParseError::NomError { err: Box::new(e) },
            Err::Failure(f) => ParseError::NomError { err: Box::new(f) },
        })?;
        Ok(iri_ref)
    }
}

/// Runs `parser` over the whole of `span`, with whitespace and comments allowed around it.
///
/// Input the parser does not use is an error: a term rudof only partly understands must not be
/// truncated to the part it does, which is a different, valid term. The whole-shapemap grammar
/// already requires the same (`shapemap_grammar::shapemap_statement`).
fn parse_whole<'a, T>(
    span: Span<'a>,
    parser: impl Parser<Span<'a>, Output = T, Error = LocatedParseError>,
) -> std::result::Result<T, Err<LocatedParseError>> {
    let (rest, value) = delimited(tws0, parser, tws0).parse(span)?;
    if rest.is_empty() {
        Ok(value)
    } else {
        Err(Err::Error(
            ParseError::TrailingInput(rest.fragment().to_string()).at(rest),
        ))
    }
}

struct ShapeMapStatementIterator<'a> {
    src: Span<'a>,
    done: bool,
}

impl ShapeMapStatementIterator<'_> {
    pub fn new(src: Span) -> Result<ShapeMapStatementIterator> {
        match tws0(src) {
            Ok((left, _)) => Ok(ShapeMapStatementIterator { src: left, done: false }),
            Err(Err::Incomplete(_)) => Ok(ShapeMapStatementIterator { src, done: false }),
            Err(e) => Err(ParseError::Custom {
                msg: format!("cannot start parsing. Error: {e}"),
            }),
        }
    }
}

impl Iterator for ShapeMapStatementIterator<'_> {
    type Item = Result<Vec<ShapeMapStatement>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        match shapemap_statement()(self.src) {
            Ok((left, s)) => {
                if s.is_empty() {
                    r = None;
                } else {
                    r = Some(Ok(s));
                }
                self.src = left;
            },
            Err(Err::Incomplete(_)) => {
                // debug!("Incomplete! shapemap_statement");
                self.done = true;
                r = None;
            },
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(ParseError::NomError { err: Box::new(e) }));
                self.done = true;
            },
        }
        if self.src.is_empty() {
            self.done = true;
        }
        if r.is_none() && !self.src.is_empty() {
            r = Some(Err(ParseError::Custom {
                msg: format!("trailing bytes {}", self.src),
            }));
        }
        r
    }
}

#[cfg(test)]
mod tests {

    use crate::shapemap::{NodeSelector, ShapeSelector};
    use rudof_iri::IriS;

    use super::*;

    #[test]
    fn test_prefix() {
        let str = r#":a@:S"#;
        let mut nodes_prefixmap = PrefixMap::new();
        nodes_prefixmap.add_prefix("", IriS::new_unchecked("http://example.org/"));

        let mut shapes_prefixmap = PrefixMap::new();
        shapes_prefixmap.add_prefix("", IriS::new_unchecked("http://example.org/shapes/"));

        let parsed_shapemap = ShapeMapParser::parse(
            str,
            &Some(nodes_prefixmap.clone()),
            &None,
            &Some(shapes_prefixmap.clone()),
            &None,
        )
        .unwrap();

        let mut expected = QueryShapeMap::new()
            .with_nodes_prefixmap(&nodes_prefixmap)
            .with_shapes_prefixmap(&shapes_prefixmap);

        expected
            .add_association(
                NodeSelector::prefixed("", "a"),
                &None,
                ShapeSelector::prefixed("", "S"),
                &None,
            )
            .unwrap();
        assert_eq!(parsed_shapemap, expected)
    }

    // The three single-term entry points parse the whole of their input: what they cannot
    // use is an error, not a term truncated to the part they can.

    #[test]
    fn test_parse_node_selector_takes_the_whole_input() {
        for str in [
            ":a",
            "<http://example.org/a>",
            "_:b1",
            "\"Alice\"",
            "\"Alice\"@en",
            "23",
            "\"23\"^^xsd:integer",
            "{FOCUS a :Person}",
            "  :a  ",
            ":a # a comment\n",
        ] {
            assert!(
                ShapeMapParser::parse_node_selector(str).is_ok(),
                "{str} is not parsed as a node selector"
            );
        }
    }

    #[test]
    fn test_parse_node_selector_rejects_trailing_input() {
        for (str, rest) in [
            (":a basura", "basura"),
            (":a @ :S {", "@ :S {"),
            ("\"Hola\"@es--rtl", "--rtl"),
            ("\"lit\" and trailing garbage", "and trailing garbage"),
            ("{FOCUS a :Person} :a", ":a"),
            // `pname_ns` takes `http:`, and `//example.org/a` cannot start a local name.
            ("http://example.org/a", "//example.org/a"),
        ] {
            let err = ShapeMapParser::parse_node_selector(str).unwrap_err();
            assert!(
                err.to_string().contains(&format!("Unexpected trailing input: {rest}")),
                "{str}: {err}"
            );
        }
    }

    #[test]
    fn test_parse_shape_selector_takes_the_whole_input() {
        for str in [":S", "<http://example.org/S>", "START", "start", "  :S  "] {
            assert!(
                ShapeMapParser::parse_shape_selector(str).is_ok(),
                "{str} is not parsed as a shape selector"
            );
        }

        let err = ShapeMapParser::parse_shape_selector(":S basura").unwrap_err();
        assert!(err.to_string().contains("Unexpected trailing input: basura"), "{err}");
    }

    #[test]
    fn test_parse_iri_ref_takes_the_whole_input() {
        for str in [":p", "ex:p", "<http://example.org/p>", "  ex:p  "] {
            assert!(
                ShapeMapParser::parse_iri_ref(str).is_ok(),
                "{str} is not parsed as an IRI reference"
            );
        }

        for (str, rest) in [
            ("ex:p basura", "basura"),
            ("<http://example.org/p> ex:q", "ex:q"),
            ("http://example.org/p", "//example.org/p"),
        ] {
            let err = ShapeMapParser::parse_iri_ref(str).unwrap_err();
            assert!(
                err.to_string().contains(&format!("Unexpected trailing input: {rest}")),
                "{str}: {err}"
            );
        }
    }
}
