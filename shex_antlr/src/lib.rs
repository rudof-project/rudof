mod grammar;
mod parse_visitor;

use std::cell::RefCell;
use std::rc::Rc;
use antlr_rust::error_strategy::BailErrorStrategy;

#[cfg(test)]
mod tests {
    use crate::grammar::shexdocparser;
    use crate::parse_visitor::ParseVisitor;
    use crate::parse_visitor::ShExDocErrorListener;
    use antlr_rust::BailErrorStrategy;
    use antlr_rust::InputStream;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use grammar::{ShExDocLexer, ShExDocParser};
    use antlr_rust::tree::Visitable;
    use super::*;
    use shex_ast::SchemaBuilder;

    #[test]
    fn test_parser() {
        let mut lexer = ShExDocLexer::new(InputStream::new(
            r#"<S> {
              <p .  
            }"#.into()));
        let num_errors = Rc::new(RefCell::new(0));
        let error_listener = Box::new(ShExDocErrorListener::new(Rc::clone(&num_errors)));    
        lexer.add_error_listener(error_listener);
        let input = CommonTokenStream::new(lexer);
        let mut parser = ShExDocParser::new(input);
        
        let es = BailErrorStrategy::<'_, shexdocparser::ShExDocParserContextType>::new();
        
        // println!("After parser...Before root..., num_errors: {:?}", error_listener.num_errors());
        //parser.err_handler.try_into()
        let root = parser.shExDoc(); 
        match root {
            Ok(tree) => {
                println!("After parse...");
                println!("After root...Before visitor {:?}", tree);
                let mut visitor = ParseVisitor { schema:   
                    SchemaBuilder::new(),
                    errors: Rc::clone(&num_errors)
                };
                println!("Afer visitor...before root accept");
                let v = tree.accept(&mut visitor);
                println!("Visitor errors = {:?}",visitor.errors);
                assert!(true, "Result: {:?}",v)
            }
            Err(es) => {
                println!("Found errors...");
                assert!(false, "Errors found parsing: {:?}",es)
            }
        } 
    }

}
