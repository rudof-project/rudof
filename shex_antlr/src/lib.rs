mod grammar;
mod shexdoc_visitor;
mod shexdoc_error_listener;

use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::grammar::shexdocparser;
    use crate::shexdoc_visitor::ParseVisitor;
    use crate::shexdoc_error_listener::ShExDocErrorListener;
    use antlr_rust::InputStream;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::errors::ANTLRError;
    use grammar::{ShExDocLexer, ShExDocParser};
    use antlr_rust::tree::Visitable;
    use super::*;
    use shex_ast::SchemaBuilder;
    use shex_ast::Schema;

    #[derive(Debug)]
    pub enum ParserError {
        Errors(u32),
        ANTLRErrors(ANTLRError)
    }

    pub fn shex_parse(str: &str) -> Result<Schema, ParserError> {
        let mut lexer = ShExDocLexer::new(InputStream::new(str));
        let num_errors = Rc::new(RefCell::new(0));
        let error_listener = Box::new(ShExDocErrorListener::new(Rc::clone(&num_errors)));    
        lexer.add_error_listener(error_listener);
        let input = CommonTokenStream::new(lexer);
        let mut parser = ShExDocParser::new(input);
        let root = parser.shExDoc(); 
        match root {
            Ok(tree) => {
                let mut visitor = ParseVisitor { 
                    schema_builder: SchemaBuilder::new(),
                    errors: Rc::clone(&num_errors)
                };
                tree.accept(&mut visitor);
                let errors = *visitor.errors.borrow();
                if errors > 0 {
                    Err(ParserError::Errors(errors))
                } else {
                    let builder = visitor.schema_builder;
                    let schema = builder.build();
                    Ok(schema)
                }
            },
            Err(es) => {
                Err(ParserError::ANTLRErrors(es))
            }
        }       
    }

    #[test]
    fn test_parser_ok() {
        let str = r#"<S> {
              <p> .  
            }"#.into();
        match shex_parse(str) {
          Ok(schema) => assert!(true, "Schema parsed"),
          Err(e) => assert!(false, "Obtained error: {:?}", e)
        }
    }

    fn test_parser_fail1() {
        let str = r#"<S> {
              <p> .  
            }"#.into();
        match shex_parse(str) {
          Ok(schema) => assert!(true, "Schema parsed"),
          Err(e) => assert!(false, "Obtained error: {:?}", e)
        }
    }

    #[test]
    fn test_parser_err() {
        let str = r#"<S> {
              <p .  
            }"#.into();
        assert!(shex_parse(str).is_err());
    }

}
