use std::rc::Rc;
use std::cell::RefCell;

use antlr_rust::{tree::{ParseTreeVisitor}, recognizer::Recognizer, token_factory::TokenFactory, errors::ANTLRError};
use antlr_rust::error_listener::ErrorListener;

#[derive(Debug, Clone)]
pub struct ParseError {
    line: isize,
    column: isize,
    msg: String
}

impl ParseError {
    pub fn new(line: isize, col: isize, msg: &str, ) -> Self {
        ParseError{ line: line, column: col, msg: String::from(msg) }
    }
}

