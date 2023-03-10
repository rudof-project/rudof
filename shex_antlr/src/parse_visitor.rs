use std::rc::Rc;
use std::cell::RefCell;

use antlr_rust::{tree::{ParseTreeVisitor}, recognizer::Recognizer, token_factory::TokenFactory, errors::ANTLRError};
use crate::grammar::{ShExDocVisitor, ShExDocParserContextType, ShExDocContext};
use shex_ast::{SchemaBuilder};
use antlr_rust::error_listener::ErrorListener;


pub struct ParseVisitor<'a> {
    pub schema: SchemaBuilder<'a>,
    pub errors: Rc<RefCell<u32>>
}

impl <'a> ParseTreeVisitor<'a, ShExDocParserContextType> for ParseVisitor<'a> {}

impl <'a> ShExDocVisitor<'a> for ParseVisitor<'a> {
    fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'a>) { 
        println!("Visiting shExDoc");
        self.visit_children(ctx);
    }
}

#[derive(Debug, Clone)]
pub struct ShExDocErrorListener {
    num_errors: Rc<RefCell<u32>>
}

impl ShExDocErrorListener {
    pub fn new(counter: Rc<RefCell<u32>>) -> ShExDocErrorListener {
        ShExDocErrorListener { num_errors: counter }
    }

    pub fn num_errors(&self) -> u32 {
        *self.num_errors.borrow()
    }
}

impl <'a, T: Recognizer<'a>> ErrorListener<'a, T> for ShExDocErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&ANTLRError>,
    ) {
        let mut ne = *self.num_errors.borrow_mut();
        ne += 1;
        eprintln!(
            "line {}:{} Syntax error {} near '{}'",
            line,
            column,
            ne,
            msg
        );
    }
}
