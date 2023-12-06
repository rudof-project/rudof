use crate::shapemap_grammar::shapemap_statement;
use crate::shapemap_grammar::ShapeMapStatement;
use crate::tws0;
use crate::ParseError;
use crate::Span;
use nom::Err;
use prefixmap::PrefixMap;
use shapemap::query_shape_map::QueryShapeMap;
use std::fs;
use std::path::PathBuf;

type Result<A> = std::result::Result<A, ParseError>;

pub struct ShapeMapParser<'a> {
    shapemap_statement_iterator: ShapeMapStatementIterator<'a>,
}

impl<'a> ShapeMapParser<'a> {
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
        while let Some(ss) = parser.shapemap_statement_iterator.next() {
            let statements = ss?;
            for s in statements {
                match s {
                    ShapeMapStatement::Association {
                        node_selector,
                        shape_selector,
                    } => {
                        log::debug!("Association {node_selector:?}@{shape_selector:?}");
                        query_shapemap.add_association(node_selector, shape_selector);
                    }
                }
            }
        }
        Ok(query_shapemap)
    }

    pub fn parse_buf(
        path_buf: &PathBuf,
        nodes_prefixmap: &Option<PrefixMap>,
        shapes_prefixmap: &Option<PrefixMap>,
    ) -> Result<QueryShapeMap> {
        let data = fs::read_to_string(&path_buf.as_path())?;
        let query_shapemap = ShapeMapParser::parse(&data, nodes_prefixmap, shapes_prefixmap)?;
        Ok(query_shapemap)
    }
}

struct ShapeMapStatementIterator<'a> {
    src: Span<'a>,
    done: bool,
}

impl<'a> ShapeMapStatementIterator<'a> {
    pub fn new(src: Span) -> Result<ShapeMapStatementIterator> {
        match tws0(src) {
            Ok((left, _)) => Ok(ShapeMapStatementIterator {
                src: left,
                done: false,
            }),
            Err(Err::Incomplete(_)) => Ok(ShapeMapStatementIterator { src, done: false }),
            Err(e) => Err(ParseError::Custom {
                msg: format!("cannot start parsing. Error: {}", e),
            }),
        }
    }
}

impl<'a> Iterator for ShapeMapStatementIterator<'a> {
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
                println!("Incomplete! shapemap_statement");
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
        nodes_prefixmap.insert("", &IriS::new_unchecked("http://example.org/"));

        let mut shapes_prefixmap = PrefixMap::new();
        shapes_prefixmap.insert("", &IriS::new_unchecked("http://example.org/shapes/"));

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
            NodeSelector::iri_unchecked("http://example.org/a"),
            ShapeSelector::iri_unchecked("http://example.org/shapes/S"),
        );
        assert_eq!(parsed_shapemap, expected)
    }
}
