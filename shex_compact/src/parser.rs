use std::collections::HashMap;

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

type Result<'a, A> = std::result::Result<A, ParseError<'a>>;

pub struct ShExParser<'a> {
    shex_statement_iterator: StatementIterator<'a>,
    state: ParserState,
    done: bool,
}

impl<'a> ShExParser<'a> {
    pub fn new(src: &'a str, base: Option<IriS>) -> Result<ShExParser<'a>> {
        Ok(ShExParser {
            shex_statement_iterator: StatementIterator::new(src)?,
            state: ParserState::default(),
            done: false,
        })
    }

    pub fn parse(&self) -> Result<Schema> {
        let mut schema = Schema::new();
        for statements in self.shex_statement_iterator {}
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
            Err(_) => Err(ParseError::Custom {
                msg: "cannot start parsing",
            }),
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<'a, Vec<ShExStatement<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        match shex_statement(self.src) {
            Ok((left, s)) => {
                r = Some(Ok(s));
                self.src = left;
            }
            Err(Err::Incomplete(_)) => {
                self.done = true;
                r = None;
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(ParseError::NomError { err: e }));
                self.done = true;
            }
        }
        match tws(self.src) {
            Ok((left, _)) => {
                self.src = left;
            }
            Err(Err::Incomplete(_)) => {
                self.done = true;
            }
            Err(_) => {
                r = Some(Err(ParseError::Custom {
                    msg: "error parsing whitespace",
                }));
                self.done = true;
            }
        }
        if r.is_none() && !self.src.is_empty() {
            r = Some(Err(ParseError::Custom {
                msg: "trailing bytes",
            }));
        }
        r
    }
}
