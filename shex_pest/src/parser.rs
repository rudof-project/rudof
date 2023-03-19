use shex_ast::SchemaBuilder;
use pest::iterators::{Pair, Pairs};
use crate::pest::Parser;
use crate::parser_error::*;
use crate::shexc_error::ShExCError;
use iri_s::IriS;
use std::str::FromStr;
use regex::Regex;

#[derive(Parser)]
#[grammar = "shex.pest"]
struct ShExParser;


pub fn parse_text<'a>(input: &'a str) -> Result<SchemaBuilder<'a>, ShExCError> {
  let mut sb = SchemaBuilder::new();
  let mut parsed = ShExParser::parse(Rule::shexDoc, input)?;
  let top_node = parsed.next().unwrap();
  cnv_pairs(top_node, sb)
}



fn cnv_pairs<'a>(input_pair: Pair<'a, Rule>, 
                 sb: SchemaBuilder<'a>) -> Result<SchemaBuilder<'a>, ShExCError> {
 // let sb = sb.set_base(IriS::from_str("http://example.org/"));
 match input_pair.as_rule() {
  Rule::shexDoc => {
    input_pair.into_inner().fold(Ok(sb), |acc, inner_pair| {
      match inner_pair.as_rule() {
        Rule::directive => {
          // println!("directive: {:?}", inner_pair);
          directive(inner_pair.into_inner().next().unwrap(), acc)
        },
        Rule::EOI => {
          println!("done parsing!");
          acc
        },
        _ => Err(unexpected(&inner_pair))
      }
  })},
  _ => Err(unexpected(&input_pair))
 }
} 


fn directive<'a>(
    pair: Pair<Rule>, 
    acc: Result<SchemaBuilder<'a>, ShExCError>) -> Result<SchemaBuilder<'a>, ShExCError> {
  acc.and_then(|sb| {
    match pair.as_rule() {
      Rule::prefixDecl => {
        let mut pairs = pair.into_inner();
        println!("PrefixDecl pairs {:?}", pairs);
        let alias = pname_ns(pairs.next().unwrap())?;
        println!("Alias! {:?}", alias);
        let iri = iri_ref(pairs.next().unwrap())?;
        println!("PrefixDecl...alias {} as iri {}", alias, iri);
        Ok(sb) // Ok(sb.add_prefix(alias.as_str(), &iri.clone()))
      },
      Rule::baseDecl => {
        let iri = iri_ref(pair.into_inner().next().unwrap())?;
        Ok(sb.set_base(iri))
      }
      _ => { 
        println!("Unexpected: {:?}", pair);
        Err(unexpected(&pair)) 
      }
    }  
  })
}

fn iri_ref(input_pair: Pair<'_, Rule>) -> Result<IriS, ShExCError> {
  if input_pair.as_rule() == Rule::IRIREF {
      let iri = input_pair.as_str().to_string();
      // strip the '<' and '>' characters.
      let iri_str = unescape_iri(&iri[1..iri.len() - 1]);
      let iri = IriS::from_str(&iri_str)?;
      if iri.is_absolute() {
          Ok(iri)
      } else {
         Err(absoluteIriExpected(iri))
      }
  } else {
      Err(unexpected(&input_pair))
  }
}

fn pname_ns(input_pair: Pair<'_, Rule>) -> Result<String, ShExCError> {
  if input_pair.as_rule() == Rule::PNAME_NS {
    let alias = input_pair.as_str().to_string();
    Ok(alias)
  } else {
    Err(unexpected(&input_pair))
  }

}


fn absoluteIriExpected(iri: IriS) -> ShExCError {
  ShExCError::AbsoluteIRIExpectedError { iri: iri }
}

fn unexpected(pair: &Pair<Rule>) -> ShExCError {
 let e = ParserErrorFactory::new("ShExC")
    .error("parse").unexpected(&pair).clone();
 ShExCError::Unexpected(e)
}


fn unescape_iri(iri: &str) -> String {
  let unicode_esc = Regex::new(r"(\\U[[:xdigit:]]{8})|(\\u[[:xdigit:]]{4})").unwrap();
  let (new_iri, end) =
      unicode_esc
          .captures_iter(iri)
          .fold((String::new(), 0), |(so_far, start), cap| {
              let cap = cap.get(0).unwrap();
              (
                  format!(
                      "{}{}{}",
                      so_far,
                      &iri[start..cap.start()],
                      unescape_uchar(cap.as_str())
                  ),
                  cap.end(),
              )
          });

  format!("{}{}", new_iri, &iri[end..])
}

fn unescape_uchar(uchar: &str) -> char {
  use std::char;
  let uchar = &uchar[2..];
  let uchar_u32 = u32::from_str_radix(uchar, 16).unwrap();
  char::from_u32(uchar_u32).unwrap()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let result: Result<SchemaBuilder, ShExCError> = parse_text(
            r###"base <http://example.org/> 
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"###,
        );
        assert!(result.is_ok());
    }

/*     #[test]
    fn parse_simple_error() {
        let result: Result<SchemaBuilder, ShExCError> = parse_text(
            r###"bse <http://example.org/> 
prefix rdf: http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"###,
        );
        assert!(result.is_err());
    } */

}