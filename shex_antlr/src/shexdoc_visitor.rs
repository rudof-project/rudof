use std::rc::Rc;
use std::cell::RefCell;

use antlr_rust::{tree::{ParseTreeVisitor}, recognizer::Recognizer, token_factory::TokenFactory, errors::ANTLRError};
use crate::grammar::{ShExDocVisitor, ShExDocParserContextType, ShExDocContext};
use shex_ast::{SchemaBuilder};
use antlr_rust::error_listener::ErrorListener;


pub struct ParseVisitor<'a> {
    pub schema_builder: SchemaBuilder<'a>,
    pub errors: Rc<RefCell<u32>>
}

impl <'a> ParseTreeVisitor<'a, ShExDocParserContextType> for ParseVisitor<'a> {}

impl <'a> ShExDocVisitor<'a> for ParseVisitor<'a> {
    fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'a>) { 
        println!("Visiting shExDoc...num_errors={:?}", self.errors.borrow());
        self.visit_children(ctx);
    }
}

