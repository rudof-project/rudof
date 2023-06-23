use oxrdf::*;
use shapemap::ShapeMapState;
use shapemap_oxgraph::{NodeIndex, ShapeLabel, ShapeMapOxGraph};
use shex_ast::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError<'a> {
    #[error("ShapeLabel not found {shape_label:?} schema: {schema:?}")]
    LabelNotFoundError {
        shape_label: &'a ShapeLabel,
        schema: &'a CompiledSchema,
    },
}

#[derive(Debug)]
pub struct CompiledSchema {
    shapes: HashMap<ShapeLabel, ShapeExpr>,
}

#[derive(Debug)]
enum ShapeExpr {
    ShapeOr {
        exprs: Vec<Box<ShapeExpr>>,
    },
    ShapeAnd {
        exprs: Vec<Box<ShapeExpr>>,
    },
    ShapeNot {
        expr: Box<ShapeExpr>,
    },
    NodeConstraint {
        node_kind: Option<NodeKind>,
        datatype: Option<NamedNode>,
        xs_facet: Vec<XsFacet>,
        values: Vec<ValueSetValue>,
    },
    Shape {
        closed: bool,
        extra: Vec<NamedNode>,
        expression: Option<TripleExpr>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    ShapeExternal {},
    Ref {
        label: ShapeLabel,
    },
}

#[derive(Debug)]
enum TripleExpr {
    EachOf {
        id: Option<TripleExprLabel>,
        expressions: Vec<Box<TripleExpr>>,
        min: Option<i32>,
        max: Option<i32>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    OneOf {
        id: Option<TripleExprLabel>,
        expressions: Vec<Box<TripleExpr>>,
        min: Option<i32>,
        max: Option<i32>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    TripleConstraint {
        id: Option<TripleExprLabel>,
        inverse: bool,
        predicate: NamedNode,
        value_expr: Option<Box<ShapeExpr>>,
        min: Option<i32>,
        max: Option<i32>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    TripleExprRef(TripleExprLabel),
}

impl CompiledSchema {
    pub fn from_schema_json<'a>(schema: SchemaJson) -> Result<CompiledSchema, ValidationError<'a>> {
        let mut shapes = HashMap::new();
        if let Some(shape_decls) = schema.shapes {
            for sd in shape_decls {
                let label = Self::id_to_shape_label(sd.id.clone())?;
                let se = Self::shape_decl_to_shape_expr(sd)?;
                shapes.insert(label, se);
            }
        }
        Ok(CompiledSchema { shapes: shapes })
    }

    fn id_to_shape_label<'a>(id: String) -> Result<ShapeLabel, ValidationError<'a>> {
        // TODO: It doesn't check anything...
        Ok(ShapeLabel::NamedNode(NamedNode::new_unchecked(id)))
    }

    fn shape_decl_to_shape_expr<'a>(sd: ShapeDecl) -> Result<ShapeExpr, ValidationError<'a>> {
        Self::cnv_shape_expr(sd.shape_expr)
    }

    fn cnv_shape_expr<'a>(se: schema_json::ShapeExpr) -> Result<ShapeExpr, ValidationError<'a>> {
        match se {
            schema_json::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for se in ses {
                    let unboxed = (*se).se;
                    let se = Self::cnv_shape_expr(unboxed)?;
                    cnv.push(Box::new(se));
                }
                Ok(ShapeExpr::ShapeOr { exprs: cnv })
            }
            schema_json::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for se in ses {
                    let unboxed = (*se).se;
                    let se = Self::cnv_shape_expr(unboxed)?;
                    cnv.push(Box::new(se));
                }
                Ok(ShapeExpr::ShapeAnd { exprs: cnv })
            }
            schema_json::ShapeExpr::ShapeNot { shape_expr: se } => {
                let unboxed = (*se).se;
                let se = Self::cnv_shape_expr(unboxed)?;
                Ok(ShapeExpr::ShapeNot { expr: Box::new(se) })
            }
            schema_json::ShapeExpr::Shape {
                closed: closed,
                extra: extra,
                expression: expression,
                sem_acts: sem_acts,
                annotations: annotations,
            } => {
                let new_extra = Self::cnv_extra(extra)?;
                let expression = Self::cnv_triple_expr(expression)?;
                Ok(ShapeExpr::Shape {
                    closed: Self::cnv_closed(closed),
                    extra: new_extra,
                    expression: expression,
                    sem_acts: Self::cnv_sem_acts(sem_acts),
                    annotations: Self::cnv_annotations(annotations),
                })
            }
            _ => todo!(),
        }
    }

    fn find_label(&self, label: &ShapeLabel) -> Option<&ShapeExpr> {
        self.shapes.get(label)
    }

    fn cnv_closed(closed: Option<bool>) -> bool {
        if let Some(closed) = closed {
            return closed;
        } else {
            false
        }
    }

    fn cnv_extra<'a>(extra: Option<Vec<IriRef>>) -> Result<Vec<NamedNode>, ValidationError<'a>> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = Self::cnv_iri_ref(iri)?;
                vs.push(nm);
            }
            Ok(vs)
        } else {
            Ok(Vec::new())
        }
    }

    fn cnv_iri_ref<'a>(iri: IriRef) -> Result<NamedNode, ValidationError<'a>> {
        // TODO Check possible errors
        Ok(NamedNode::new_unchecked(iri.value.clone()))
    }

    fn cnv_triple_expr<'a>(
        triple_expr_wrapper: Option<TripleExprWrapper>,
    ) -> Result<Option<TripleExpr>, ValidationError<'a>> {
        if let Some(tew) = triple_expr_wrapper {
            let te = tew.te;
            match te {
                schema_json::TripleExpr::EachOf {
                    id: id,
                    expressions: expressions,
                    min: min,
                    max: max,
                    sem_acts: sem_acts,
                    annotations: annotations,
                } => todo!(),
                schema_json::TripleExpr::OneOf {
                    id: id,
                    expressions: expressions,
                    min: min,
                    max: max,
                    sem_acts: sem_acts,
                    annotations: annotations,
                } => todo!(),
                schema_json::TripleExpr::TripleConstraint {
                    id: id,
                    inverse: inverse,
                    predicate: predicate,
                    value_expr: value_expr,
                    min: min,
                    max: max,
                    sem_acts: sem_acts,
                    annotations: annotations,
                } => {
                    let id = Self::cnv_id(id);
                    let sem_acts = Self::cnv_sem_acts(sem_acts);
                    let annotations = Self::cnv_annotations(annotations);
                    let predicate = Self::cnv_iri_ref(predicate)?;
                    let value_expr = if let Some(se) = value_expr {
                        let se = Self::cnv_shape_expr(*se)?;
                        Some(Box::new(se))
                    } else {
                        None
                    };
                    Ok(Some(TripleExpr::TripleConstraint {
                        id: id,
                        inverse: inverse.unwrap_or(false),
                        predicate: predicate,
                        value_expr: value_expr,
                        min: min,
                        max: max,
                        sem_acts: sem_acts,
                        annotations: annotations,
                    }))
                }
                schema_json::TripleExpr::TripleExprRef(_) => todo!(),
            }
        } else {
            Ok(None)
        }
    }

    fn cnv_id(id: Option<schema_json::TripleExprLabel>) -> Option<TripleExprLabel> {
        id
    }

    fn cnv_sem_acts(sem_acts: Option<Vec<SemAct>>) -> Vec<SemAct> {
        todo!()
    }

    fn cnv_annotations(annotations: Option<Vec<Annotation>>) -> Vec<Annotation> {
        todo!()
    }
}

pub fn check_node_shape<'a>(
    node: NodeIndex,
    shape_label: &'a ShapeLabel,
    schema: &'a CompiledSchema,
    sm: ShapeMapOxGraph,
) -> Result<ShapeMapState<'a, NodeIndex, ShapeLabel>, ValidationError<'a>> {
    if let Some(shape) = schema.find_label(shape_label) {
        todo!();
    } else {
        Err(ValidationError::LabelNotFoundError {
            shape_label: shape_label,
            schema: schema,
        })
    }
}

#[cfg(test)]
mod tests {
    use oxrdf::NamedNode;

    #[test]
    fn validation_node_test() {
        assert_eq!(22, 22);
    }
}
