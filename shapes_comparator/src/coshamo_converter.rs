use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::{Schema, ShapeExpr, TripleExpr};
use sparql_service::ServiceDescription;
use tracing::{debug, trace};

use crate::{CoShaMo, ComparatorConfig, ComparatorError, ValueConstraint, ValueDescription};

#[derive(Clone, Debug)]
pub struct CoShaMoConverter {
    config: ComparatorConfig,
    current_coshamo: CoShaMo,
}

impl CoShaMoConverter {
    pub fn new(config: &ComparatorConfig) -> Self {
        CoShaMoConverter {
            config: config.clone(),
            current_coshamo: CoShaMo::new(),
        }
    }

    pub fn from_service(
        &mut self,
        service: ServiceDescription,
        label: &Option<String>,
    ) -> Result<CoShaMo, ComparatorError> {
        self.current_coshamo = CoShaMo::new();
        self.service2coshamo(&service, label)
    }

    fn service2coshamo(
        &mut self,
        _service: &ServiceDescription,
        _label: &Option<String>,
    ) -> Result<CoShaMo, ComparatorError> {
        Ok(self.current_coshamo.clone())
    }

    pub fn from_shex(
        &mut self,
        schema: &Schema,
        label: Option<&str>,
    ) -> Result<CoShaMo, ComparatorError> {
        self.current_coshamo = CoShaMo::new().with_prefixmap(schema.prefixmap());
        // choose the shape
        if let Some(label) = label {
            if let Some(shape) = schema.find_shape(label).map_err(|e| {
                trace!("Schema: {schema}");
                ComparatorError::ShapeNotFound {
                    label: label.to_string(),
                    available_shapes: if let Some(shapes) = schema.shapes() {
                        shapes
                            .iter()
                            .map(|s| s.id().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                            .to_string()
                    } else {
                        "No Shapes".to_string()
                    },
                    error: e.to_string(),
                }
            })? {
                self.shape2coshamo(&shape)
            } else {
                trace!("Returned None when trying to find {label} at schema: {schema}");
                Err(ComparatorError::ShapeNotFound {
                    label: label.to_string(),
                    available_shapes: if let Some(shapes) = schema.shapes() {
                        shapes
                            .iter()
                            .map(|s| s.id().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                            .to_string()
                    } else {
                        "No Shapes".to_string()
                    },
                    error: "Shape not found".to_string(),
                })
            }
        } else {
            debug!("No label provided");
            Err(ComparatorError::NoShapeLabelProvided)
        }
    }

    fn get_iri(&self, iri_ref: &IriRef) -> Result<IriS, ComparatorError> {
        self.current_coshamo.resolve(iri_ref)
    }

    fn triple_expr2coshamo(
        &mut self,
        triple_expr: &TripleExpr,
        coshamo: &mut CoShaMo,
    ) -> Result<(), ComparatorError> {
        match triple_expr {
            TripleExpr::EachOf {
                id: _,
                expressions,
                min: _,
                max: _,
                sem_acts: _,
                annotations: _,
            } => {
                for e in expressions {
                    let (iri, tc) = self.triple_expr_as_constraint2coshamo(&e.te, coshamo)?;
                    let iri_s = self.get_iri(&iri)?;
                    coshamo.add_constraint(&iri_s, tc);
                }
                Ok(())
            }
            TripleExpr::OneOf {
                id: _,
                expressions: _,
                min: _,
                max: _,
                sem_acts: _,
                annotations: _,
            } => Err(ComparatorError::NotImplemented {
                feature: "OneOf".to_string(),
            }),
            TripleExpr::TripleConstraint {
                id: _,
                negated: _,
                inverse: _,
                predicate,
                value_expr,
                min: _,
                max: _,
                sem_acts: _,
                annotations,
            } => {
                self.triple_constraint2coshamo(predicate, value_expr, annotations)?;
                let iri_s = self.get_iri(predicate)?;
                self.current_coshamo
                    .add_constraint(&iri_s, ValueDescription::new(predicate));
                Ok(())
            }
            TripleExpr::TripleExprRef(_) => todo!(),
        }
    }

    fn triple_constraint2coshamo(
        &mut self,
        predicate: &IriRef,
        value_expr: &Option<Box<ShapeExpr>>,
        _annotations: &Option<Vec<shex_ast::Annotation>>,
    ) -> Result<(), ComparatorError> {
        let iri_s = self.get_iri(predicate)?;
        let value_constraint = self.value_expr2value_constraint(value_expr)?;
        self.current_coshamo.add_constraint(
            &iri_s,
            ValueDescription::new(predicate).with_value_constraint(value_constraint),
        );
        Ok(())
    }

    fn triple_expr_as_constraint2coshamo(
        &mut self,
        triple_expr: &TripleExpr,
        _coshamo: &mut CoShaMo,
    ) -> Result<(IriRef, ValueDescription), ComparatorError> {
        match triple_expr {
            TripleExpr::EachOf { .. } => Err(ComparatorError::NotImplemented {
                feature: "EachOf as constraint".to_string(),
            }),
            TripleExpr::OneOf { .. } => Err(ComparatorError::NotImplemented {
                feature: "OneOf as constraint".to_string(),
            }),
            TripleExpr::TripleConstraint {
                predicate,
                value_expr,
                ..
            } => {
                let node_constraint = self.value_expr2value_constraint(value_expr)?;
                Ok((
                    predicate.clone(),
                    ValueDescription::new(predicate).with_value_constraint(node_constraint),
                ))
            }
            TripleExpr::TripleExprRef(_) => Err(ComparatorError::NotImplemented {
                feature: "TripleExprRef as constraint".to_string(),
            }),
        }
    }

    fn value_expr2value_constraint(
        &mut self,
        value_expr: &Option<Box<ShapeExpr>>,
    ) -> Result<ValueConstraint, ComparatorError> {
        if self.config.ignore_value_constraints() {
            return Ok(ValueConstraint::Any);
        }
        if let Some(value_expr) = value_expr {
            match value_expr.as_ref() {
                ShapeExpr::NodeConstraint(ref nc) => {
                    if let Some(datatype) = &nc.datatype() {
                        let iri_s = self.get_iri(datatype)?;
                        Ok(ValueConstraint::datatype(iri_s))
                    } else if let Some(nk) = &nc.node_kind() {
                        Ok(ValueConstraint::nodekind(&nk.to_string()))
                    } else {
                        Err(ComparatorError::NotImplemented {
                            feature: format!("Complex node constraint as ValueExpr: {nc:?}"),
                        })
                    }
                }
                ShapeExpr::Shape(s) => Err(ComparatorError::NotImplemented {
                    feature: format!("Shape as ValueExpr, shape: {s:?}"),
                }),
                ShapeExpr::ShapeOr { .. } => Err(ComparatorError::NotImplemented {
                    feature: "ShapeOr as ValueExpr".to_string(),
                }),
                ShapeExpr::ShapeAnd { .. } => Err(ComparatorError::NotImplemented {
                    feature: "ShapeAnd as ValueExpr".to_string(),
                }),
                ShapeExpr::ShapeNot { .. } => Err(ComparatorError::NotImplemented {
                    feature: "ShapeNot as ValueExpr".to_string(),
                }),
                ShapeExpr::External => Err(ComparatorError::NotImplemented {
                    feature: "External as ValueExpr".to_string(),
                }),
                ShapeExpr::Ref(r) => {
                    let r: String = r.into();
                    ValueConstraint::reference(&r)
                }
            }
        } else {
            Ok(ValueConstraint::Any)
        }
    }

    fn shape2coshamo(&mut self, shape: &ShapeExpr) -> Result<CoShaMo, ComparatorError> {
        let mut coshamo = CoShaMo::new();

        // convert the shape to CoShaMo
        match shape {
            ShapeExpr::ShapeOr { shape_exprs: _ } => Err(ComparatorError::NotImplemented {
                feature: "ShapeOr".to_string(),
            }),
            ShapeExpr::ShapeAnd { shape_exprs: _ } => Err(ComparatorError::NotImplemented {
                feature: "ShapeAnd".to_string(),
            }),
            ShapeExpr::ShapeNot { shape_expr: _ } => Err(ComparatorError::NotImplemented {
                feature: "ShapeNot".to_string(),
            }),
            ShapeExpr::NodeConstraint(_) => Err(ComparatorError::NotImplemented {
                feature: "NodeConstraint".to_string(),
            }),
            ShapeExpr::Shape(shape) => {
                if let Some(triple_expr) = shape.triple_expr() {
                    self.triple_expr2coshamo(&triple_expr, &mut coshamo)?
                }
                // Process shape.constraints
                // Not implemented yet
                Ok(coshamo)
            }
            ShapeExpr::External => Err(ComparatorError::NotImplemented {
                feature: "External".to_string(),
            }),
            ShapeExpr::Ref(_) => Err(ComparatorError::NotImplemented {
                feature: "Reference".to_string(),
            }),
        }
    }
}
