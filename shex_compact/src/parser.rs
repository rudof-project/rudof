use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use iri_s::IriS;
use nom::error::Error;
use nom::Err;
use prefixmap::PrefixMap;
use shex_ast::Schema;
use shex_ast::ShapeExpr;

use crate::shex_statement;
use crate::tws;
use crate::ParseError;
use crate::ParserState;
use crate::ShExStatement;

// This code is inspired from:
// https://github.com/vandenoever/rome/blob/master/src/io/turtle/parser.rs

type Result<A> = std::result::Result<A, ParseError>;

pub struct ShExParser<'a> {
    shex_statement_iterator: StatementIterator<'a>,
    state: ParserState,
    done: bool,
}

impl <'a> ShExParser<'a> {

    pub fn parse(src: String, base: Option<IriS>) -> Result<Schema> {
        let mut schema = Schema::new();
        let mut parser = ShExParser {
            shex_statement_iterator: StatementIterator::new(src.as_str())?,
            state: ParserState::default(),
            done: false,
        };
        while let Some(ss) = parser.shex_statement_iterator.next() {
            let statements = ss?;
            for s in statements {
                match s {
                    ShExStatement::BaseDecl { iri } => {
                        todo!()
                    }
                    ShExStatement::PrefixDecl { alias, iri } => {
                        schema.add_prefix(alias, &iri);
                    }
                    ShExStatement::StartDecl { shape_expr } => {
                        todo!()
                    }
                    ShExStatement::ImportDecl { iri } => {
                        todo!()
                    }
                    ShExStatement::ShapeDecl {
                        shape_label,
                        shape_expr,
                    } => {
                        todo!()
                    }
                    ShExStatement::StartActions {
                        actions
                    } => {
                        todo!()
                    }
                }
            }
        };
        Ok(schema)
    }

    /*pub fn process_statements(&mut self) -> Result<'a, ()> {
        // let mut schema = Schema::new();
        while let Some(ss) = self.shex_statement_iterator.next() {
            let statements = ss?;
            for s in statements {
                match s {
                    ShExStatement::BaseDecl { iri } => {
                        todo!()
                    }
                    ShExStatement::PrefixDecl { alias, iri } => {
                        self.schema.add_prefix(alias, &iri);
                    }
                    ShExStatement::StartDecl { shape_expr } => {
                        todo!()
                    }
                    ShExStatement::ImportDecl { iri } => {
                        todo!()
                    }
                    ShExStatement::ShapeDecl {
                        shape_label,
                        shape_expr,
                    } => {
                        todo!()
                    }
                }
            }
        }
        Ok(())
    }*/


    pub fn parse_buf(path_buf: &PathBuf, base: Option<IriS>) -> Result<Schema> {
        let data = fs::read_to_string(&path_buf.as_path())?;
        let schema = ShExParser::parse(data, base)?;
        Ok(schema)
    }
}

struct StatementIterator<'a> {
    src: &'a str,
    done: bool,
}

impl<'a> StatementIterator<'a> {
    pub fn new(src: &str) -> Result<StatementIterator> {
        match tws(src) {
            Ok((left, _)) => Ok(StatementIterator {
                src: left,
                done: false,
            }),
            Err(Err::Incomplete(_)) => Ok(StatementIterator { src, done: false }),
            Err(e) => Err(ParseError::Custom {
                msg: format!("cannot start parsing. Error: {}",e),
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
        match shex_statement(self.src) {
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
                r = Some(Err(ParseError::NomError { err: e.code }));
                self.done = true;
            }
        }

        match tws(self.src) {
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
    use super::*;

    #[test]
    fn test_prefix() {
        let str = r#"prefix e: <http://example.org/>"#;
        let schema = ShExParser::parse(str.to_string(), None).unwrap();
        let mut expected = Schema::new();
        expected.add_prefix("e", &IriS::new_unchecked("http://example.org/"));
        assert_eq!(schema, expected)
    }
}
