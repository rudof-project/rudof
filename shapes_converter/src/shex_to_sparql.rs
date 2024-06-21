use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use shex_ast::{Schema, Shape, ShapeExpr, TripleExpr};

use crate::{SelectQuery, ShEx2SparqlConfig, ShEx2SparqlError, TriplePattern, Var};

pub struct ShEx2Sparql {
    config: ShEx2SparqlConfig,
}

impl ShEx2Sparql {
    pub fn new(config: ShEx2SparqlConfig) -> ShEx2Sparql {
        ShEx2Sparql { config }
    }

    pub fn convert(
        &self,
        shex: Schema,
        maybe_shape: Option<IriRef>,
    ) -> Result<SelectQuery, ShEx2SparqlError> {
        match maybe_shape {
            Some(shape) => {
                if let Some(shape_expr) = shex.find_shape_by_iri_ref(&shape)? {
                    let prefixmap = shex.prefixmap().unwrap_or_else(|| PrefixMap::new());
                    let patterns = shape_expr2patterns(&shape_expr, &self.config, &prefixmap);
                    let query = SelectQuery::new()
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

fn shape_expr2patterns(
    se: &ShapeExpr,
    config: &ShEx2SparqlConfig,
    prefixmap: &PrefixMap,
) -> Vec<TriplePattern> {
    let mut ps = Vec::new();
    match se {
        ShapeExpr::ShapeOr { shape_exprs: _ } => todo!(),
        ShapeExpr::ShapeAnd { shape_exprs: _ } => todo!(),
        ShapeExpr::ShapeNot { shape_expr: _ } => todo!(),
        ShapeExpr::NodeConstraint(_) => todo!(),
        ShapeExpr::Shape(s) => shape2patterns(s, &mut ps, config, prefixmap),
        ShapeExpr::External => todo!(),
        ShapeExpr::Ref(_) => todo!(),
    }
    ps
}

fn shape2patterns(
    s: &Shape,
    ps: &mut Vec<TriplePattern>,
    config: &ShEx2SparqlConfig,
    prefixmap: &PrefixMap,
) {
    // Maybe add triples for extra?
    if let Some(expr) = s.triple_expr() {
        triple_expr2patterns(&expr, ps, config, prefixmap)
    }
}

fn triple_expr2patterns(
    te: &TripleExpr,
    ps: &mut Vec<TriplePattern>,
    config: &ShEx2SparqlConfig,
    prefixmap: &PrefixMap,
) {
    match te {
        TripleExpr::EachOf { expressions, .. } => {
            for tew in expressions {
                triple_expr2patterns(&tew.te, ps, config, prefixmap)
            }
        }
        TripleExpr::OneOf { expressions, .. } => {
            for tew in expressions {
                triple_expr2patterns(&tew.te, ps, config, prefixmap)
            }
        }
        TripleExpr::TripleConstraint {
            inverse,
            predicate,
            value_expr: _,
            min: _,
            max: _,
            ..
        } => {
            // TODO: check min and max cardinalities
            if let Some(true) = inverse {
                todo!()
            } else {
                // TODO: check if value_expr is a single valuevalueset
                let subj = Var::new(config.this_variable_name.as_str());
                let pred = predicate;
                let obj = var_from_predicate(&predicate, prefixmap);
                let tp = TriplePattern::new(&subj, pred, &obj);
                ps.push(tp)
            }
        }
        TripleExpr::TripleExprRef(_) => todo!(),
    }
}

fn var_from_predicate(predicate: &IriRef, prefixmap: &PrefixMap) -> Var {
    match predicate {
        IriRef::Iri(iri) => {
            if let Some(local) = prefixmap.qualify_local(iri) {
                Var::new(local.as_str())
            } else {
                todo!()
            }
        }
        IriRef::Prefixed { prefix: _, local } => Var::new(local),
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
        let person = iri!("http://example.org/Person");
        let converter = ShEx2Sparql::new(ShEx2SparqlConfig::default());
        let converted_query = converter.convert(schema, Some(person)).unwrap();
        let converted_query_str = format!("{}", converted_query);
        let converted_query_parsed = Query::parse(converted_query_str.as_str(), None).unwrap();
        assert_eq!(converted_query_parsed, expected_query);
    }
}
