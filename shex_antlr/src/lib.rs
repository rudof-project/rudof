mod grammar;
mod parse_visitor;



#[cfg(test)]
mod tests {
    use crate::parse_visitor::ParseVisitor;
    use antlr_rust::InputStream;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use grammar::{ShExDocLexer, ShExDocParser};
    use antlr_rust::tree::Visitable;
    use super::*;
    use shex_ast::SchemaBuilder;

    #[test]
    fn test_parser() {
        let lexer = ShExDocLexer::new(InputStream::new(
            r#"<S> {
              <p .  
            }"#.into()));
        let mut parser = ShExDocParser::new(CommonTokenStream::new(lexer));
        println!("After parser...Before root...");
        let root = parser.shExDoc().expect("parse tree root node");
        println!("After root...Before visitor {:?}", root);
        let mut visitor = ParseVisitor { schema: SchemaBuilder::new().build() };
        println!("Afer visitor...before root accept");
        let v = root.accept(&mut visitor);
        println!("Result of root accept = {:?}",v);
        assert_eq!(2+2, 4)
    }

}
