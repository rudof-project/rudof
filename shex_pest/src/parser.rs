use shex_ast::SchemaBuilder;
use pest::error::Error;
use crate::pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "shex.pest"]
struct ShExParser;




pub(super) fn parse_text(input: &str) -> Result<SchemaBuilder, Error<Rule>> {
    let mut parsed = ShExParser::parse(Rule::shexDoc, input)?;
    let top_node = parsed.next().unwrap();
    cnv_pairs(top_node)
}

fn cnv_pairs(input_pair: Pair<'_, Rule>) -> Result<SchemaBuilder, Error<Rule>> {
 let sb = SchemaBuilder::new();
 match input_pair.as_rule() {
   Rule::shexDoc => Ok(sb),
   _ => todo!()
 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let result: Result<SchemaBuilder, Error<Rule>> = parse_text(
            r###"base <http://example.org/> 
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"###,
        );
        assert!(result.is_ok());
    }
}