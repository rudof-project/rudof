use std::fs;
use std::path::PathBuf;

use iri_s::IriS;
use nom::Err;
use prefixmap::Deref;
use shex_ast::Iri;
use shex_ast::Schema;

use crate::grammar_structs::ShExStatement;
use crate::shex_statement;
use crate::tws0;
use crate::ParseError;
use crate::Span;

// This code is inspired from:
// https://github.com/vandenoever/rome/blob/master/src/io/turtle/parser.rs

type Result<A> = std::result::Result<A, ParseError>;

pub struct ShExParser<'a> {
    shex_statement_iterator: StatementIterator<'a>,
    // state: ParserState,
    // done: bool,
}

impl<'a> ShExParser<'a> {
    /// Parse a ShEx schema that uses [ShEx compact syntax](https://shex.io/shex-semantics/index.html#shexc)
    ///
    /// `base` is an optional IRI that acts as the base for relative IRIs
    pub fn parse(src: &str, base: Option<IriS>) -> Result<Schema> {
        let mut schema = Schema::new();
        let mut parser = ShExParser {
            shex_statement_iterator: StatementIterator::new(Span::new(src))?,
            // state: ParserState::default(),
            // done: false,
        };
        while let Some(ss) = parser.shex_statement_iterator.next() {
            let statements = ss?;
            for s in statements {
                match s {
                    ShExStatement::BaseDecl { iri } => {
                        schema = schema.with_base(Some(iri));
                    }
                    ShExStatement::PrefixDecl { alias, iri } => {
                        schema.add_prefix(alias, &iri);
                    }
                    ShExStatement::StartDecl { shape_expr } => {
                        schema = schema.with_start(Some(shape_expr))
                    }
                    ShExStatement::ImportDecl { iri } => {
                        schema = schema.with_import(Iri::new(iri.as_str()));
                    }
                    ShExStatement::ShapeDecl {
                        is_abstract,
                        shape_label,
                        shape_expr,
                    } => {
                        let shape_label = shape_label.deref(&schema.base(), &schema.prefixmap())?;
                        let shape_expr = shape_expr.deref(&schema.base(), &schema.prefixmap())?;
                        schema.add_shape(shape_label, shape_expr, is_abstract);
                    }
                    ShExStatement::StartActions { actions } => {
                        schema = schema.with_start_actions(Some(actions));
                    }
                }
            }
        }
        Ok(schema)
    }

    pub fn parse_buf(path_buf: &PathBuf, base: Option<IriS>) -> Result<Schema> {
        let data = fs::read_to_string(&path_buf.as_path())?;
        let schema = ShExParser::parse(&data, base)?;
        Ok(schema)
    }
}

struct StatementIterator<'a> {
    src: Span<'a>,
    done: bool,
}

impl<'a> StatementIterator<'a> {
    pub fn new(src: Span) -> Result<StatementIterator> {
        match tws0(src) {
            Ok((left, _)) => Ok(StatementIterator {
                src: left,
                done: false,
            }),
            Err(Err::Incomplete(_)) => Ok(StatementIterator { src, done: false }),
            Err(e) => Err(ParseError::Custom {
                msg: format!("cannot start parsing. Error: {}", e),
            }),
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<Vec<ShExStatement<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        match shex_statement()(self.src) {
            Ok((left, s)) => {
                if s.is_empty() {
                    r = None;
                } else {
                    r = Some(Ok(s));
                }
                self.src = left;
            }
            Err(Err::Incomplete(_)) => {
                println!("Incomplete! shex_statement");
                self.done = true;
                r = None;
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(ParseError::NomError { err: Box::new(e) }));
                self.done = true;
            }
        }

        match tws0(self.src) {
            Ok((left, _)) => {
                self.src = left;
            }
            Err(Err::Incomplete(_)) => {
                println!("Incomplete! tws");
                self.done = true;
            }
            Err(e) => {
                r = Some(Err(ParseError::Custom {
                    msg: format!("error parsing whitespace. Error: {}", e),
                }));
                self.done = true;
            }
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
    use shex_ast::{Shape, ShapeExpr, ShapeExprLabel};

    use super::*;

    #[test]
    fn test_prefix() {
        let str = r#"
 prefix e: <http://example.org/>
 e:S {}
 "#;
        let schema = ShExParser::parse(str, None).unwrap();
        let mut expected = Schema::new();
        expected.add_prefix("e", &IriS::new_unchecked("http://example.org/"));
        expected.add_shape(
            ShapeExprLabel::iri_unchecked("http://example.org/S"),
            ShapeExpr::Shape(Shape::new(None, None, None)),
            false,
        );
        assert_eq!(schema, expected)
    }
}
