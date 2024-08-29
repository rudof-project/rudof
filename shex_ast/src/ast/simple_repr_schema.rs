use crate::{Shape, ShapeExprLabel};
use iri_s::IriS;
use prefixmap::IriRef;
use serde_derive::{Deserialize, Serialize};

use super::{Schema, ShapeDecl, ShapeExpr, TripleExpr, ValueSetValue};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SimpleReprSchema {
    shapes: Vec<SimpleReprShape>,
}

impl SimpleReprSchema {
    pub fn new() -> SimpleReprSchema {
        SimpleReprSchema { shapes: Vec::new() }
    }

    pub fn from_schema(&mut self, schema: &Schema) {
        if let Some(shapes) = schema.shapes() {
            for shape in shapes {
                let simple_shape = self.convert_shape_decl(&shape, schema);
                self.shapes.push(simple_shape)
            }
        }
    }

    pub fn convert_shape_decl(
        &mut self,
        shape_decl: &ShapeDecl,
        schema: &Schema,
    ) -> SimpleReprShape {
        self.convert_shape_expr(&shape_decl.id, &shape_decl.shape_expr, schema)
    }

    pub fn convert_shape_expr(
        &mut self,
        name: &ShapeExprLabel,
        shape: &ShapeExpr,
        schema: &Schema,
    ) -> SimpleReprShape {
        match shape {
            ShapeExpr::ShapeOr { shape_exprs: _ } => todo!(),
            ShapeExpr::ShapeAnd { shape_exprs: _ } => todo!(),
            ShapeExpr::ShapeNot { shape_expr: _ } => todo!(),
            ShapeExpr::NodeConstraint(_) => todo!(),
            ShapeExpr::Shape(shape) => self.convert_shape(name, shape, schema),
            ShapeExpr::External => todo!(),
            ShapeExpr::Ref(_) => todo!(),
        }
    }

    pub fn convert_shape(
        &mut self,
        name: &ShapeExprLabel,
        shape: &Shape,
        schema: &Schema,
    ) -> SimpleReprShape {
        let mut simple = SimpleReprShape::new(name);
        if let Some(triple_expr) = &shape.expression {
            self.convert_triple_expr(&mut simple, &triple_expr.te, schema);
        }
        simple
    }

    pub fn convert_triple_expr(
        &mut self,
        shape: &mut SimpleReprShape,
        te: &TripleExpr,
        schema: &Schema,
    ) {
        match te {
            TripleExpr::EachOf {
                id: _,
                expressions,
                min: _,
                max: _,
                sem_acts: _,
                annotations: _,
            } => {
                for te in expressions {
                    self.convert_triple_expr(shape, &(te.te), schema);
                }
            }
            TripleExpr::OneOf {
                id: _,
                expressions,
                min: _,
                max: _,
                sem_acts: _,
                annotations: _,
            } => {
                for te in expressions {
                    self.convert_triple_expr(shape, &te.te, schema);
                }
            }
            TripleExpr::TripleConstraint {
                id: _,
                negated: _,
                inverse: _,
                predicate,
                value_expr,
                min: _,
                max: _,
                sem_acts: _,
                annotations: _,
            } => {
                let iri = schema.resolve_iriref(&predicate);
                if iri == IriS::rdf_type() {
                    if let Some(se) = value_expr {
                        self.extract_class_values(&se, shape);
                    }
                } else {
                    shape.add_predicate(predicate)
                }
            }
            TripleExpr::TripleExprRef(_) => todo!(),
        }
    }

    fn extract_class_values(&mut self, se: &ShapeExpr, shape: &mut SimpleReprShape) {
        match se {
            ShapeExpr::NodeConstraint(nc) => {
                if let Some(values) = nc.values() {
                    for value in values {
                        match value {
                            ValueSetValue::ObjectValue(ov) => match ov {
                                super::ObjectValue::IriRef(iri_ref) => shape.add_class(&iri_ref),
                                super::ObjectValue::Literal(_) => todo!(),
                            },
                            _ => todo!(),
                        }
                    }
                }
            }
            _ => todo!(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SimpleReprShape {
    #[serde(skip_serializing)]
    id: ShapeExprLabel,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    class: Vec<IriRef>,

    predicates: Vec<IriRef>,
}

impl SimpleReprShape {
    pub fn new(label: &ShapeExprLabel) -> SimpleReprShape {
        SimpleReprShape {
            id: label.clone(),
            class: Vec::new(),
            predicates: Vec::new(),
        }
    }

    pub fn add_predicate(&mut self, pred: &IriRef) {
        self.predicates.push(pred.clone())
    }

    pub fn add_class(&mut self, cls: &IriRef) {
        self.class.push(cls.clone())
    }
}
