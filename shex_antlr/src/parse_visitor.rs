use antlr_rust::tree::{ParseTreeVisitor};
use crate::grammar::{ShExDocVisitor, ShExDocParserContextType, ShExDocContext};
use shex_ast::Schema;


pub struct ParseVisitor {
    pub schema: Schema
}

impl <'a> ParseTreeVisitor<'a, ShExDocParserContextType> for ParseVisitor {}

impl <'a> ShExDocVisitor<'a> for ParseVisitor {
    fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'a>) { 
        println!("Visiting shExDoc");
        self.visit_children(ctx) 
    }
}

