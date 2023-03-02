use antlr_rust::tree::{ParseTreeVisitor};
use crate::grammar::{ShExDocVisitor, ShExDocParserContextType, ShExDocContext};
use shex_ast::Schema;


pub struct ParseVisitor<'a> {
    pub schema: Schema<'a>
}

impl <'a> ParseTreeVisitor<'a, ShExDocParserContextType> for ParseVisitor<'a> {}

impl <'a> ShExDocVisitor<'a> for ParseVisitor<'a> {
    fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'a>) { 
        println!("Visiting shExDoc");
        self.visit_children(ctx) 
    }
}

