use crate::shapemap_grammar::shapemap_statement;
use crate::shapemap_grammar::ShapeMapStatement;
use crate::shapemap_grammar::{node_selector, shape_spec};
use crate::shex_grammar::iri;
use crate::tws0;
use crate::ParseError;
use crate::Span;
use nom::Err;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use shapemap::query_shape_map::QueryShapeMap;
use shapemap::NodeSelector;
use shapemap::ShapeSelector;
use std::fs;
use std::path::Path;
use tracing::debug;

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
        shapes_prefixmap: &Option<PrefixMap>,
    ) -> Result<QueryShapeMap> {
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
                        // tracing::debug!("Association {node_selector:?}@{shape_selector:?}");
                        query_shapemap.add_association(node_selector, shape_selector);
                    }
                }
            }
        }
        Ok(query_shapemap)
    }

    pub fn parse_buf(
        path: &Path,
        nodes_prefixmap: &Option<PrefixMap>,
        shapes_prefixmap: &Option<PrefixMap>,
    ) -> Result<QueryShapeMap> {
        let data = fs::read_to_string(path)?;
        let query_shapemap = ShapeMapParser::parse(&data, nodes_prefixmap, shapes_prefixmap)?;
        Ok(query_shapemap)
    }

    pub fn parse_shape_selector(str: &str) -> Result<ShapeSelector> {
        let span = Span::new(str);
        let (_, ss) = shape_spec()(span).map_err(|e| match e {
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
        let (_, ns) = node_selector()(span).map_err(|e| match e {
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
        let (_, iri_ref) = iri(span).map_err(|e| match e {
            Err::Incomplete(s) => ParseError::Custom {
                msg: format!("Incomplete input: needed {s:?}"),
            },
            Err::Error(e) => ParseError::NomError { err: Box::new(e) },
            Err::Failure(f) => ParseError::NomError { err: Box::new(f) },
        })?;
        Ok(iri_ref)
    }
}

struct ShapeMapStatementIterator<'a> {
    src: Span<'a>,
    done: bool,
}

impl ShapeMapStatementIterator<'_> {
    pub fn new(src: Span) -> Result<ShapeMapStatementIterator> {
        match tws0(src) {
            Ok((left, _)) => Ok(ShapeMapStatementIterator {
                src: left,
                done: false,
            }),
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
            }
            Err(Err::Incomplete(_)) => {
                debug!("Incomplete! shapemap_statement");
                self.done = true;
                r = None;
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(ParseError::NomError { err: Box::new(e) }));
                self.done = true;
            }
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

    use iri_s::IriS;
    use shapemap::{NodeSelector, ShapeSelector};

    use super::*;

    #[test]
    fn test_prefix() {
        let str = r#":a@:S"#;
        let mut nodes_prefixmap = PrefixMap::new();
        nodes_prefixmap
            .insert("", &IriS::new_unchecked("http://example.org/"))
            .unwrap();

        let mut shapes_prefixmap = PrefixMap::new();
        shapes_prefixmap
            .insert("", &IriS::new_unchecked("http://example.org/shapes/"))
            .unwrap();

        let parsed_shapemap = ShapeMapParser::parse(
            str,
            &Some(nodes_prefixmap.clone()),
            &Some(shapes_prefixmap.clone()),
        )
        .unwrap();

        let mut expected = QueryShapeMap::new()
            .with_nodes_prefixmap(&nodes_prefixmap)
            .with_shapes_prefixmap(&shapes_prefixmap);

        expected.add_association(
            NodeSelector::prefixed("", "a"),
            ShapeSelector::prefixed("", "S"),
        );
        assert_eq!(parsed_shapemap, expected)
    }
}
