mod grammar;

#[allow(unused_imports)]
use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTreeVisitor, Visitable};

use grammar::{ShExDocLexer, ShExDocParser, ShExDocVisitor, ShExDocParserContextType};
// use grammar::{ShExDocLexer, ShExDocParser};

#[derive(Debug, PartialEq)]
pub enum AST<'node> {
    Nothing(Vec<AST<'node>>),
    Expr(&'node str)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct CustomParseTreeVisitor<'node> {
    pub _nodes: Vec<AST<'node>>
}

impl <'node> ParseTreeVisitor<'node, ShExDocParserContextType> 
 for CustomParseTreeVisitor<'node> {}

impl <'node> ShExDocVisitor<'node> for CustomParseTreeVisitor<'node> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parser() {
        let mut lexer = ShExDocLexer::new(InputStream::new(
            r#"<S> {
              <p> .  
            }"#.into()));
        let mut parser = ShExDocParser::new(CommonTokenStream::new(lexer));
        let root = parser.shExDoc().expect("parse tree root node");
        let mut visitor = CustomParseTreeVisitor { _nodes: vec![]  };
        let v = root.accept(&mut visitor);
        println!("Result of root accept = {:?}",v);
        assert_eq!(2+2, 4)
    }

}
