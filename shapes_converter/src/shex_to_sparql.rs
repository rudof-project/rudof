use iri_s::IriS;
use shex_ast::Schema;
use spargebra::Query;

use crate::{SelectQuery, ShEx2SparqlError};

pub struct ShEx2Sparql {}

impl ShEx2Sparql {
    pub fn shex_to_sparql(
        shex: Schema,
        maybe_shape: Option<IriS>,
    ) -> Result<Query, ShEx2SparqlError> {
        match maybe_shape {
            Some(shape) => {
                if let Some(_shape_expr) = shex.find_shape_by_iri(&shape)? {
                    let query = SelectQuery::new();
                    todo!()
                } else {
                    Err(ShEx2SparqlError::ShapeNotFound {
                        iri: shape,
                        schema: shex,
                    })
                }
            }
            None => {
                // Should search start shape
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iri_s::iri;
    use shex_compact::ShExParser;
    use spargebra::Query;

    #[test]
    fn test_simple() {
        let shex_str = "\
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string ;
  :knows @<Person> 
}";
        let schema = ShExParser::parse(shex_str, None).unwrap();
        let query_str = "\
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

Select * where {
    ?this :name  ?name  .
    ?this :knows ?knows  
}";
        let expected_query = Query::parse(query_str, None).unwrap();
        println!("expected_query: {expected_query:?}");

        let person = iri!("http://example.org/Person");
        let converted_query = ShEx2Sparql::shex_to_sparql(schema, Some(person)).unwrap();
        assert_eq!(converted_query, expected_query);
    }
}
