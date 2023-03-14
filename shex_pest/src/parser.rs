use shex_ast::SchemaBuilder;
use pest::iterators::{Pair, Pairs};
use crate::pest::Parser;
use crate::parser_error::ParserErrorFactory;
use crate::shexc_error::ShExCError;
use iri_s::IriS;

#[derive(Parser)]
#[grammar = "shex.pest"]
struct ShExParser;


pub fn parse_text<'a>(input: &'a str) -> Result<&'a mut SchemaBuilder<'a>, ShExCError> {
  let mut sb = SchemaBuilder::new();
  let mut parsed = ShExParser::parse(Rule::shexDoc, input)?;
  let top_node = parsed.next().unwrap();
  // cnv_pairs(top_node, &mut sb)
  todo!()
}



fn cnv_pairs<'a>(input_pair: Pair<'a, Rule>, 
                 sb: &'a mut SchemaBuilder<'a>) -> Result<&'a mut SchemaBuilder<'a>, ShExCError> {
 let mut sb = sb.set_base(IriS::from_str("http://example.org/"));
 match input_pair.as_rule() {
   Rule::shexDoc => {
    let mut directive = input_pair.into_inner().next().unwrap();
    parse_directive(directive, sb)
   },
   _ => Err(unexpected(&input_pair))
 }
}

fn parse_directive<'a>(
    pairs: Pair<Rule>, 
    sb: &'a mut SchemaBuilder<'a>) -> Result<&'a mut SchemaBuilder<'a>, ShExCError> {
  Ok(sb)      
}



fn unexpected(pair: &Pair<Rule>) -> ShExCError {
 let e = ParserErrorFactory::new("ShExC")
    .error("parse").unexpected(&pair).clone();
 ShExCError::Unexpected(e)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let result: Result<&mut SchemaBuilder, ShExCError> = parse_text(
            r###"base <http://example.org/> 
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"###,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn parse_simple_error() {
        let result: Result<&mut SchemaBuilder, ShExCError> = parse_text(
            r###"bse <http://example.org/> 
prefix rdf: http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"###,
        );
        assert!(result.is_err());
    }

}