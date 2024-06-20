use iri_s::IriS;
use shex_ast::{Schema, ShapeExpr};
use spargebra::Query;

use crate::{SelectQuery, ShEx2SparqlError, TriplePattern};

pub struct ShEx2Sparql {}

impl ShEx2Sparql {
    pub fn shex_to_sparql(
        shex: Schema,
        maybe_shape: Option<IriS>,
    ) -> Result<SelectQuery, ShEx2SparqlError> {
        match maybe_shape {
            Some(shape) => {
                if let Some(shape_expr) = shex.find_shape_by_iri(&shape)? {
                    let patterns = shape_expr2patterns(shape_expr);
                    let mut query = SelectQuery::new()
                        .with_prefixmap(shex.prefixmap())
                        .with_base(shex.base())
                        .with_patterns(patterns);
                    Ok(query)
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

fn shape_expr2patterns(se: &ShapeExpr) -> Vec<TriplePattern> {
    let mut ps = Vec::new();
    match se {
        ShapeExpr::ShapeOr { shape_exprs: _ } => todo!(),
        ShapeExpr::ShapeAnd { shape_exprs: _ } => todo!(),
        ShapeExpr::ShapeNot { shape_expr: _ } => todo!(),
        ShapeExpr::NodeConstraint(_) => todo!(),
        ShapeExpr::Shape(_) => todo!(),
        ShapeExpr::External => todo!(),
        ShapeExpr::Ref(_) => todo!(),
    }
    ps
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
