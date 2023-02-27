use antlr_rust::tree::{ParseTreeVisitor};
use crate::grammar::{ShExDocVisitor, ShExDocParserContextType, ShExDocContext};
use shex_ast::Schema;


pub struct ParseVisitor {
    pub schema: Schema
}

impl <'node> ParseTreeVisitor<'node, ShExDocParserContextType> for ParseVisitor {}

impl <'node> ShExDocVisitor<'node> for ParseVisitor {
    fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'node>) { 
        println!("Visiting shExDoc");
        self.visit_children(ctx) 
    }
}
