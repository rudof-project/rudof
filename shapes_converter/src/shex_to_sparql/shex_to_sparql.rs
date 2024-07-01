use prefixmap::IriRef;
use shex_ast::{Schema, Shape, ShapeExpr, TripleExpr};

use crate::shex_to_sparql::{
    SelectQuery, ShEx2SparqlConfig, ShEx2SparqlError, TriplePattern, Var, VarBuilder,
};

pub struct ShEx2Sparql {
    config: ShEx2SparqlConfig,
}

impl ShEx2Sparql {
    pub fn new(config: ShEx2SparqlConfig) -> ShEx2Sparql {
        ShEx2Sparql { config }
    }

    pub fn convert(
        &self,
        shex: &Schema,
        maybe_shape: Option<IriRef>,
    ) -> Result<SelectQuery, ShEx2SparqlError> {
        let mut var_builder = VarBuilder::new();
        match maybe_shape {
            Some(shape) => {
                if let Some(shape_expr) = shex.find_shape_by_iri_ref(&shape)? {
                    shape_expr2query(&shape_expr, &self.config, shex, &mut var_builder)
                } else {
                    Err(ShEx2SparqlError::ShapeNotFound {
                        iri: shape,
                        schema: shex.clone(),
                    })
                }
            }
            None => {
                if let Some(shape_expr) = shex.start() {
                    shape_expr2query(&shape_expr, &self.config, shex, &mut var_builder)
                } else {
                    // Convert the first shape
                    if let Some(shapes) = shex.shapes() {
                        if let Some(shape_decl) = shapes.first() {
                            shape_expr2query(
                                &shape_decl.shape_expr,
                                &self.config,
                                shex,
                                &mut var_builder,
                            )
                        } else {
                            Err(ShEx2SparqlError::EmptyShapes {
                                schema: shex.clone(),
                            })
                        }
                    } else {
                        Err(ShEx2SparqlError::NoShapes {
                            schema: shex.clone(),
                        })
                    }
                }
            }
        }
    }
}

fn shape_expr2query(
    shape_expr: &ShapeExpr,
    config: &ShEx2SparqlConfig,
    schema: &Schema,
    var_builder: &mut VarBuilder,
) -> Result<SelectQuery, ShEx2SparqlError> {
    let patterns = shape_expr2patterns(shape_expr, config, schema, var_builder)?;
    let query = SelectQuery::new()
        .with_prefixmap(schema.prefixmap())
        .with_base(schema.base())
        .with_patterns(patterns);
    Ok(query)
}

fn shape_expr2patterns(
    se: &ShapeExpr,
    config: &ShEx2SparqlConfig,
    schema: &Schema,
    var_builder: &mut VarBuilder,
) -> Result<Vec<TriplePattern>, ShEx2SparqlError> {
    let mut ps = Vec::new();
    match se {
        ShapeExpr::ShapeOr { shape_exprs: _ } => Err(ShEx2SparqlError::not_implemented("ShapeOr")),
        ShapeExpr::ShapeAnd { shape_exprs: _ } => {
            Err(ShEx2SparqlError::not_implemented("ShapeAND"))
        }
        ShapeExpr::ShapeNot { shape_expr: _ } => Err(ShEx2SparqlError::not_implemented("ShapeNot")),
        ShapeExpr::NodeConstraint(nc) => {
            let msg = format!("Node constraint: {nc:?}");
            Err(ShEx2SparqlError::not_implemented(msg.as_str()))
        }
        ShapeExpr::Shape(s) => {
            shape2patterns(s, &mut ps, config, schema, var_builder);
            Ok(ps)
        }
        ShapeExpr::External => Err(ShEx2SparqlError::not_implemented("ShapeExternal")),
        ShapeExpr::Ref(sref) => {
            if let Some(shape_expr) = schema.find_shape_by_label(sref)? {
                shape_expr2patterns(&shape_expr, config, schema, var_builder)
            } else {
                Err(ShEx2SparqlError::ShapeRefNotFound {
                    sref: sref.clone(),
                    schema: schema.clone(),
                })
            }
        }
    }
}

fn shape2patterns(
    s: &Shape,
    ps: &mut Vec<TriplePattern>,
    config: &ShEx2SparqlConfig,
    schema: &Schema,
    var_builder: &mut VarBuilder,
) {
    // Maybe add triples for extra?
    if let Some(expr) = s.triple_expr() {
        triple_expr2patterns(&expr, ps, config, schema, var_builder)
    }
}

fn triple_expr2patterns(
    te: &TripleExpr,
    ps: &mut Vec<TriplePattern>,
    config: &ShEx2SparqlConfig,
    schema: &Schema,
    var_builder: &mut VarBuilder,
) {
    match te {
        TripleExpr::EachOf { expressions, .. } => {
            for tew in expressions {
                triple_expr2patterns(&tew.te, ps, config, schema, var_builder)
            }
        }
        TripleExpr::OneOf { expressions, .. } => {
            for tew in expressions {
                triple_expr2patterns(&tew.te, ps, config, schema, var_builder)
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
                let obj = var_from_predicate(predicate, schema, var_builder);
                let tp = TriplePattern::new(&subj, pred, &obj);
                ps.push(tp)
            }
        }
        TripleExpr::TripleExprRef(_) => todo!(),
    }
}

fn var_from_predicate(predicate: &IriRef, schema: &Schema, var_builder: &mut VarBuilder) -> Var {
    match predicate {
        IriRef::Iri(iri) => match schema.prefixmap() {
            None => Var::new_from_iri(iri, var_builder),
            Some(prefixmap) => {
                if let Some(local) = prefixmap.qualify_local(iri) {
                    Var::new(local.as_str())
                } else {
                    todo!()
                }
            }
        },
        IriRef::Prefixed { prefix: _, local } => Var::new(local),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let converter = ShEx2Sparql::new(ShEx2SparqlConfig::default());
        let converted_query = converter.convert(&schema, None).unwrap();
        let converted_query_str = format!("{}", converted_query);
        let converted_query_parsed = Query::parse(converted_query_str.as_str(), None).unwrap();
        assert_eq!(converted_query_parsed, expected_query);
    }
}
