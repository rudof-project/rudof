use crate::ParseError;
use crate::Span;
use crate::ast::Schema;
use crate::compact::grammar_structs::ShExStatement;
use crate::shex_statement;
use crate::tws0;
use iri_s::IriS;
use nom::Err;
use prefixmap::DerefIri;
use std::fs;
use std::io;
use std::path::Path;
use tracing::debug;

// This code is inspired from:
// https://github.com/vandenoever/rome/blob/master/src/io/turtle/parser.rs

type Result<A> = std::result::Result<A, ParseError>;

pub struct ShExParser<'a> {
    shex_statement_iterator: StatementIterator<'a>,
    // state: ParserState,
    // done: bool,
}

impl ShExParser<'_> {
    /// Parse a ShEx schema that uses [ShEx compact syntax](https://shex.io/shex-semantics/index.html#shexc)
    ///
    /// `base` is an optional IRI that acts as the base for relative IRIs
    pub fn parse(src: &str, base: Option<IriS>, source_iri: &IriS) -> Result<Schema> {
        let mut schema = Schema::new(source_iri).with_base(base);
        let mut parser = ShExParser {
            shex_statement_iterator: StatementIterator::new(Span::new(src))?,
        };
        // let mut shapes_counter = 0;
        for s in parser.shex_statement_iterator.by_ref() {
            match s? {
                ShExStatement::BaseDecl { iri } => {
                    schema = schema.with_base(Some(iri));
                },
                ShExStatement::PrefixDecl { alias, iri } => {
                    schema.add_prefix(alias, &iri)?;
                },
                ShExStatement::StartDecl { shape_expr } => schema = schema.with_start(Some(shape_expr)),
                ShExStatement::ImportDecl { iri } => {
                    schema = schema.with_import(iri);
                },
                ShExStatement::ShapeDecl {
                    is_abstract,
                    shape_label,
                    shape_expr,
                } => {
                    let shape_label = shape_label.deref_iri(schema.base().as_ref(), schema.prefixmap().as_ref())?;
                    let shape_expr = shape_expr.deref_iri(schema.base().as_ref(), schema.prefixmap().as_ref())?;
                    // shapes_counter += 1;
                    // tracing::debug!("Shape decl #{shapes_counter}: {shape_label} ");
                    schema.add_shape(shape_label, shape_expr, is_abstract);
                },
                ShExStatement::StartActions { actions } => {
                    schema = schema.with_start_actions(Some(actions));
                },
            }
        }
        Ok(schema)
    }

    pub fn parse_buf(path: &Path, base: Option<IriS>) -> Result<Schema> {
        let source_iri = path.try_into().map_err(|e| ParseError::Custom {
            msg: format!("Cannot convert path to IRI: {e}"),
        })?;
        let data = fs::read_to_string(path)?;
        let schema = ShExParser::parse(&data, base, &source_iri)?;
        Ok(schema)
    }

    pub fn from_reader<R: io::Read>(mut reader: R, base: Option<IriS>, source_iri: &IriS) -> Result<Schema> {
        let mut v = Vec::new();
        reader.read_to_end(&mut v)?;
        let s = String::from_utf8(v).map_err(|e| ParseError::Utf8Error { error: format!("{e}") })?;
        Self::parse(s.as_str(), base, source_iri)
    }
}

struct StatementIterator<'a> {
    src: Span<'a>,
    done: bool,
}

impl StatementIterator<'_> {
    pub fn new(src: Span) -> Result<StatementIterator> {
        match tws0(src) {
            Ok((left, _)) => Ok(StatementIterator { src: left, done: false }),
            Err(Err::Incomplete(_)) => Ok(StatementIterator { src, done: false }),
            Err(e) => Err(ParseError::Custom {
                msg: format!("cannot start parsing. Error: {e}"),
            }),
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<ShExStatement<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        if self.src.is_empty() {
            self.done = true;
            return None;
        }
        match shex_statement()(self.src) {
            Ok((left, s)) => {
                r = Some(Ok(s));
                self.src = left;
            },
            Err(Err::Incomplete(needed)) => {
                debug!("Incomplete! shex_statement. Needed: {needed:?}");
                self.done = true;
                r = None;
            },
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(ParseError::NomError { err: Box::new(e) }));
                self.done = true;
            },
        }

        // Skip extra whitespace
        match tws0(self.src) {
            Ok((left, _)) => {
                self.src = left;
            },
            Err(Err::Incomplete(needed)) => {
                debug!("Incomplete on tws: needed {needed:?}");
                self.done = true;
            },
            Err(e) => {
                r = Some(Err(ParseError::Custom {
                    msg: format!("error parsing whitespace. Error: {e}"),
                }));
                self.done = true;
            },
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Shape, ShapeExpr, ShapeExprLabel};

    use super::*;
    use iri_s::iri;

    #[test]
    fn test_prefix() {
        let str = r#"
 prefix e: <http://example.org/>
 e:S {}
 "#;
        let schema = ShExParser::parse(str, None, &iri!("http://default/")).unwrap();
        let mut expected = Schema::new(&iri!("http://default/"));
        expected
            .add_prefix("e", &IriS::new_unchecked("http://example.org/"))
            .unwrap();
        expected.add_shape(
            ShapeExprLabel::iri_unchecked("http://example.org/S"),
            ShapeExpr::Shape(Shape::new(None, None, None)),
            false,
        );
        assert_eq!(schema, expected)
    }
}
