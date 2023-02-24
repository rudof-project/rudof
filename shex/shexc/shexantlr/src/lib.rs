mod grammar;
use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;

use grammar::{ShExDocLexer, ShExDocParser};
// use grammar::{ShExDocLexer, ShExDocParser};


pub fn add(left: usize, right: usize) -> usize {
    left + right
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
        let lexer = ShExDocLexer::new(InputStream::new(
            r#"<S> {
              <p> .  
            }"#.into()));
        let mut parser = ShExDocParser::new(CommonTokenStream::new(lexer));
        let root = parser.shExDoc().unwrap();
        assert_eq!(2+2, 4)
    }

}
