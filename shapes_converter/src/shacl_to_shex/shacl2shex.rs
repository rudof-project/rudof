use super::{Shacl2ShExConfig, Shacl2ShExError};
use iri_s::IriS;
use prefixmap::IriRef;
use shacl_ast::{
    component::Component, node_shape::NodeShape, property_shape::PropertyShape,
    shape::Shape as ShaclShape, target::Target, Schema as ShaclSchema,
};
use shex_ast::{
    BNode, NodeConstraint, Schema as ShExSchema, Shape as ShExShape, ShapeExpr, ShapeExprLabel,
    TripleExpr, TripleExprWrapper, ValueSetValue,
};
use srdf::{Object, RDFNode, Rdf, SHACLPath};
use tracing::debug;

#[allow(dead_code)] // TODO: only for config...
pub struct Shacl2ShEx {
    config: Shacl2ShExConfig,
    current_shex: ShExSchema,
}

impl Shacl2ShEx {
    pub fn new(config: &Shacl2ShExConfig) -> Shacl2ShEx {
        Shacl2ShEx {
            config: config.clone(),
            current_shex: ShExSchema::new(),
        }
    }

    pub fn current_shex(&self) -> &ShExSchema {
        &self.current_shex
    }

    pub fn convert<RDF: Rdf>(&mut self, schema: &ShaclSchema<RDF>) -> Result<(), Shacl2ShExError> {
        let prefixmap = schema.prefix_map().without_rich_qualifying();
        self.current_shex = ShExSchema::new().with_prefixmap(Some(prefixmap));
        for (_, shape) in schema.iter() {
            match &shape {
                shacl_ast::shape::Shape::NodeShape(ns) => {
                    let (label, shape_expr, is_abstract) = self.convert_shape(ns, schema)?;
                    self.current_shex.add_shape(label, shape_expr, is_abstract)
                }
                shacl_ast::shape::Shape::PropertyShape(_) => {
                    // Ignoring property shapes at top level conversion
                }
            }
        }
        Ok(())
    }

    pub fn convert_shape<RDF: Rdf>(
        &self,
        shape: &NodeShape<RDF>,
        schema: &ShaclSchema<RDF>,
    ) -> Result<(ShapeExprLabel, ShapeExpr, bool), Shacl2ShExError> {
        let label = self.rdfnode2label(shape.id())?;
        let shape_expr = self.node_shape2shape_expr(shape, schema)?;
        let is_abstract = false; // TODO: No virtual shapes in SHACL so it is always false
        Ok((label, shape_expr, is_abstract))
    }

    pub fn rdfnode2label(&self, node: &RDFNode) -> Result<ShapeExprLabel, Shacl2ShExError> {
        match node {
            srdf::Object::Iri(iri) => Ok(ShapeExprLabel::iri(iri.clone())),
            srdf::Object::BlankNode(bn) => Ok(ShapeExprLabel::bnode(BNode::new(bn))),
            srdf::Object::Literal(lit) => Err(Shacl2ShExError::RDFNode2LabelLiteral {
                literal: lit.clone(),
            }),
            Object::Triple { .. } => todo!(),
        }
    }

    pub fn node_shape2shape_expr<RDF: Rdf>(
        &self,
        shape: &NodeShape<RDF>,
        schema: &ShaclSchema<RDF>,
    ) -> Result<ShapeExpr, Shacl2ShExError> {
        let mut exprs = Vec::new();
        for node in shape.property_shapes() {
            match schema.get_shape(node) {
                None => todo!(),
                Some(shape) => match shape {
                    ShaclShape::PropertyShape(ps) => {
                        let tc = self.property_shape2triple_constraint(ps)?;
                        exprs.push(tc);
                        Ok(())
                    }
                    ShaclShape::NodeShape(ns) => Err(Shacl2ShExError::NotExpectedNodeShape {
                        node_shape: ns.to_string(),
                    }),
                },
            }?
        }
        let is_closed = None; // TODO: Check real value
        let extra = None; // TODO: Check if we could find a way to obtain extras in SHACL ?
        let mut te = if exprs.is_empty() {
            None
        } else {
            Some(TripleExpr::each_of(exprs))
        };
        if self.config.add_target_class() {
            let target_class_expr = self.convert_target_decls(shape.targets(), schema)?;
            te = match (te, target_class_expr) {
                (None, None) => None,
                (None, Some(t)) => Some(t),
                (Some(t), None) => Some(t),
                (Some(t1), Some(t2)) => Some(self.merge_triple_exprs(&t1, &t2)),
            };
        }
        let shape = ShExShape::new(is_closed, extra, te);
        Ok(ShapeExpr::shape(shape))
    }

    /// Collect targetClass declarations and add a rdf:type constraint for each
    pub fn convert_target_decls<RDF: Rdf>(
        &self,
        targets: &Vec<Target<RDF>>,
        schema: &ShaclSchema<RDF>,
    ) -> Result<Option<TripleExpr>, Shacl2ShExError> {
        let mut values = Vec::new();
        for target in targets {
            if let Some(value) = self.target2value_set_value(target, schema)? {
                values.push(value);
            }
        }
        let value_cls = ShapeExpr::node_constraint(NodeConstraint::new().with_values(values));
        let tc = TripleExpr::triple_constraint(
            None,
            None,
            IriRef::iri(IriS::rdf_type()),
            Some(value_cls),
            None,
            None,
        );
        Ok(Some(tc))
    }

    pub fn target2value_set_value<RDF: Rdf>(
        &self,
        target: &Target<RDF>,
        _schema: &ShaclSchema<RDF>,
    ) -> Result<Option<ValueSetValue>, Shacl2ShExError> {
        match target {
            Target::TargetNode(_) => Ok(None),
            Target::TargetClass(cls) => {
                let value_set_value = match cls {
                    Object::Iri(iri) => Ok(ValueSetValue::iri(IriRef::iri(iri.clone()))),
                    Object::BlankNode(bn) => {
                        Err(Shacl2ShExError::UnexpectedBlankNodeForTargetClass {
                            bnode: bn.clone(),
                        })
                    }
                    Object::Literal(lit) => Err(Shacl2ShExError::UnexpectedLiteralForTargetClass {
                        literal: lit.clone(),
                    }),
                    Object::Triple { .. } => todo!(),
                }?;
                Ok(Some(value_set_value))
            }
            Target::TargetSubjectsOf(_) => Ok(None),
            Target::TargetObjectsOf(_) => Ok(None),
            Target::TargetImplicitClass(_) => Ok(None),
            Target::WrongTargetNode(_) => todo!(),
            Target::WrongTargetClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        }
    }

    pub fn merge_triple_exprs(&self, te1: &TripleExpr, te2: &TripleExpr) -> TripleExpr {
        match te1 {
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => match te2 {
                TripleExpr::EachOf {
                    id: _,
                    expressions: exprs,
                    min: _,
                    max: _,
                    sem_acts: _,
                    annotations: _,
                } => TripleExpr::EachOf {
                    id: id.clone(),
                    expressions: Self::merge_expressions(expressions, exprs),
                    min: *min,
                    max: *max,
                    sem_acts: sem_acts.clone(),
                    annotations: annotations.clone(),
                },
                tc @ TripleExpr::TripleConstraint {
                    id,
                    negated: _,
                    inverse: _,
                    predicate: _,
                    value_expr: _,
                    min,
                    max,
                    sem_acts,
                    annotations,
                } => TripleExpr::EachOf {
                    id: id.clone(),
                    expressions: Self::merge_expressions(
                        expressions,
                        &vec![TripleExprWrapper { te: tc.clone() }],
                    ),
                    min: *min,
                    max: *max,
                    sem_acts: sem_acts.clone(),
                    annotations: annotations.clone(),
                },
                _ => todo!(),
            },
            tc @ TripleExpr::TripleConstraint {
                id,
                negated: _,
                inverse: _,
                predicate: _,
                value_expr: _,
                min,
                max,
                sem_acts,
                annotations,
            } => match te2 {
                TripleExpr::EachOf {
                    id: _,
                    expressions: exprs,
                    min: _,
                    max: _,
                    sem_acts: _,
                    annotations: _,
                } => TripleExpr::EachOf {
                    id: id.clone(),
                    expressions: Self::merge_expressions(
                        &vec![TripleExprWrapper { te: tc.clone() }],
                        exprs,
                    ),
                    min: *min,
                    max: *max,
                    sem_acts: sem_acts.clone(),
                    annotations: annotations.clone(),
                },
                tc2 @ TripleExpr::TripleConstraint {
                    id,
                    negated: _,
                    inverse: _,
                    predicate: _,
                    value_expr: _,
                    min,
                    max,
                    sem_acts,
                    annotations,
                } => TripleExpr::EachOf {
                    id: id.clone(),
                    expressions: vec![
                        TripleExprWrapper { te: tc.clone() },
                        TripleExprWrapper { te: tc2.clone() },
                    ],
                    min: *min,
                    max: *max,
                    sem_acts: sem_acts.clone(),
                    annotations: annotations.clone(),
                },
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn merge_expressions(
        e1: &Vec<TripleExprWrapper>,
        e2: &Vec<TripleExprWrapper>,
    ) -> Vec<TripleExprWrapper> {
        let mut es = Vec::new();
        for e in e1 {
            es.push(e.clone())
        }
        for e in e2 {
            es.push(e.clone())
        }
        es
    }

    pub fn property_shape2triple_constraint<RDF: Rdf>(
        &self,
        shape: &PropertyShape<RDF>,
    ) -> Result<TripleExpr, Shacl2ShExError> {
        let predicate = self.shacl_path2predicate(shape.path())?;
        let negated = None;
        let inverse = None;
        let se = self.components2shape_expr(shape.components())?;
        let min = None;
        let max = None;
        Ok(TripleExpr::triple_constraint(
            negated, inverse, predicate, se, min, max,
        ))
    }

    pub fn components2shape_expr(
        &self,
        components: &Vec<Component>,
    ) -> Result<Option<ShapeExpr>, Shacl2ShExError> {
        let mut ses = Vec::new();
        for c in components {
            let se = self.component2shape_expr(c)?;
            ses.push(se);
        }
        if ses.is_empty() {
            Ok(None)
        } else {
            match ses.len() {
                1 => {
                    let se = &ses[0];
                    Ok(Some(se.clone()))
                }
                _ => {
                    // Err(Shacl2ShExError::not_implemented("Conversion of shapes with multiple components is not implemented yet: {components:?}"))}
                    debug!("More than one component: {components:?}, taking only the first one");
                    let se = &ses[0];
                    Ok(Some(se.clone()))
                }
            }
        }
    }

    pub fn create_class_constraint(&self, cls: &RDFNode) -> Result<ShapeExpr, Shacl2ShExError> {
        let rdf_type = IriRef::iri(IriS::rdf_type());
        let value = match cls {
            Object::Iri(iri) => ValueSetValue::iri(IriRef::iri(iri.clone())),
            Object::BlankNode(_) => todo!(),
            Object::Literal(_) => todo!(),
            Object::Triple { .. } => todo!(),
        };
        let cls = NodeConstraint::new().with_values(vec![value]);
        let te = TripleExpr::triple_constraint(
            None,
            None,
            rdf_type,
            Some(ShapeExpr::node_constraint(cls)),
            None,
            None,
        );
        let se = ShapeExpr::shape(ShExShape::new(None, None, Some(te)));
        Ok(se)
    }

    pub fn component2shape_expr(
        &self,
        component: &Component,
    ) -> Result<ShapeExpr, Shacl2ShExError> {
        match component {
            Component::Class(cls) => {
                debug!("TODO: Converting Class components for {cls:?} doesn't match rdfs:subClassOf semantics of SHACL yet");
                let se = self.create_class_constraint(cls)?;
                Ok(se)
            }
            Component::Datatype(dt) => Ok(ShapeExpr::node_constraint(
                NodeConstraint::new().with_datatype(dt.clone()),
            )),
            Component::NodeKind(_) => todo!(),
            Component::MinCount(_) => todo!(),
            Component::MaxCount(_) => todo!(),
            Component::MinExclusive(_) => todo!(),
            Component::MaxExclusive(_) => todo!(),
            Component::MinInclusive(_) => todo!(),
            Component::MaxInclusive(_) => todo!(),
            Component::MinLength(_) => todo!(),
            Component::MaxLength(_) => todo!(),
            Component::Pattern {
                pattern: _,
                flags: _,
            } => todo!(),
            Component::UniqueLang(_) => todo!(),
            Component::LanguageIn { langs: _ } => todo!(),
            Component::Equals(_) => todo!(),
            Component::Disjoint(_) => todo!(),
            Component::LessThan(_) => todo!(),
            Component::LessThanOrEquals(_) => todo!(),
            Component::Or { shapes: _ } => {
                debug!("Not implemented OR Shapes");
                Ok(ShapeExpr::empty_shape())
            }
            Component::And { shapes: _ } => todo!(),
            Component::Not { shape: _ } => todo!(),
            Component::Xone { shapes: _ } => todo!(),
            Component::Closed {
                is_closed: _,
                ignored_properties: _,
            } => todo!(),
            Component::Node { shape: _ } => todo!(),
            Component::HasValue { value: _ } => todo!(),
            Component::In { values: _ } => todo!(),
            Component::QualifiedValueShape {
                shape: _,
                qualified_min_count: _,
                qualified_max_count: _,
                qualified_value_shapes_disjoint: _,
            } => todo!(),
            Component::Deactivated(_) => todo!(),
        }
    }

    pub fn shacl_path2predicate(&self, path: &SHACLPath) -> Result<IriRef, Shacl2ShExError> {
        match path {
            SHACLPath::Predicate { pred } => Ok(IriRef::iri(pred.clone())),
            _ => todo!(),
        }
    }
}
