use super::{Shacl2ShExConfig, Shacl2ShExError};
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_iri::iri;
use rudof_rdf::rdf_core::{SHACLPath, term::Object};
use shacl::ir::{IRComponent, IRNodeShape, IRPropertyShape, IRSchema, IRShape};
use shacl::types::Target;
use shex_ast::{
    BNode, NodeConstraint, Schema as ShExSchema, Shape as ShExShape, ShapeExpr, ShapeExprLabel, TripleExpr,
    TripleExprWrapper, ValueSetValue,
};
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
            current_shex: ShExSchema::new(&iri!("http://default/")),
        }
    }

    pub fn current_shex(&self) -> &ShExSchema {
        &self.current_shex
    }

    pub fn convert(&mut self, schema: &IRSchema) -> Result<(), Shacl2ShExError> {
        let prefixmap = schema.prefix_map().clone().without_rich_qualifying();
        self.current_shex = ShExSchema::new(&iri!("http://default/")).with_prefixmap(Some(prefixmap));
        for (_, shape) in schema.iter() {
            match &shape {
                IRShape::NodeShape(ns) => {
                    let (label, shape_expr, is_abstract) = self.convert_shape(ns, schema)?;
                    self.current_shex.add_shape(label, shape_expr, is_abstract)
                },
                IRShape::PropertyShape(_) => {
                    // Ignoring property shapes at top level conversion
                },
            }
        }
        Ok(())
    }

    pub fn convert_shape(
        &self,
        shape: &IRNodeShape,
        schema: &IRSchema,
    ) -> Result<(ShapeExprLabel, ShapeExpr, bool), Shacl2ShExError> {
        let label = self.rdfnode2label(shape.id())?;
        let shape_expr = self.node_shape2shape_expr(shape, schema)?;
        let is_abstract = false; // TODO: No virtual shapes in SHACL so it is always false
        Ok((label, shape_expr, is_abstract))
    }

    pub fn rdfnode2label(&self, node: &Object) -> Result<ShapeExprLabel, Shacl2ShExError> {
        match node {
            Object::Iri(iri) => Ok(ShapeExprLabel::iri(iri.clone())),
            Object::BlankNode(bn) => Ok(ShapeExprLabel::bnode(BNode::new(bn))),
            Object::Literal(lit) => Err(Shacl2ShExError::RDFNode2LabelLiteral { literal: lit.clone() }),
            Object::Triple { .. } => Err(Shacl2ShExError::not_implemented("shapes identified by RDF triples")),
        }
    }

    pub fn node_shape2shape_expr(&self, shape: &IRNodeShape, schema: &IRSchema) -> Result<ShapeExpr, Shacl2ShExError> {
        let mut exprs = Vec::new();
        for node in shape.property_shapes() {
            match schema.get_shape_from_idx(node) {
                None => Err(Shacl2ShExError::not_implemented(
                    "property shapes missing from the schema",
                )),
                Some(shape) => match shape {
                    IRShape::PropertyShape(ps) => {
                        let tc = self.property_shape2triple_constraint(ps, schema)?;
                        exprs.push(tc);
                        Ok(())
                    },
                    IRShape::NodeShape(_ns) => Err(Shacl2ShExError::NotExpectedNodeShape {
                        // TODO - maybe implement display on IRNodeShape and enhance IRShape Display
                        node_shape: shape.to_string(),
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
    pub fn convert_target_decls(
        &self,
        targets: &Vec<Target>,
        schema: &IRSchema,
    ) -> Result<Option<TripleExpr>, Shacl2ShExError> {
        let mut values = Vec::new();
        for target in targets {
            if let Some(value) = self.target2value_set_value(target, schema)? {
                values.push(value);
            }
        }
        let value_cls = ShapeExpr::node_constraint(NodeConstraint::new().with_values(values));
        let tc = TripleExpr::triple_constraint(None, None, IriRef::iri(IriS::rdf_type()), Some(value_cls), None, None);
        Ok(Some(tc))
    }

    pub fn target2value_set_value(
        &self,
        target: &Target,
        _schema: &IRSchema,
    ) -> Result<Option<ValueSetValue>, Shacl2ShExError> {
        match target {
            Target::Node(_) => Ok(None),
            Target::Class(cls) => {
                let value_set_value = match cls {
                    Object::Iri(iri) => Ok(ValueSetValue::iri(IriRef::iri(iri.clone()))),
                    Object::BlankNode(bn) => {
                        Err(Shacl2ShExError::UnexpectedBlankNodeForTargetClass { bnode: bn.clone() })
                    },
                    Object::Literal(lit) => {
                        Err(Shacl2ShExError::UnexpectedLiteralForTargetClass { literal: lit.clone() })
                    },
                    Object::Triple { .. } => Err(Shacl2ShExError::not_implemented("RDF triples as target classes")),
                }?;
                Ok(Some(value_set_value))
            },
            Target::SubjectsOf(_) => Ok(None),
            Target::ObjectsOf(_) => Ok(None),
            Target::ImplicitClass(_) => Ok(None),
            Target::WrongNode(_)
            | Target::WrongClass(_)
            | Target::WrongSubjectsOf(_)
            | Target::WrongObjectsOf(_)
            | Target::WrongImplicitClass(_) => Err(Shacl2ShExError::not_implemented("malformed targets")),
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
                    expressions: Self::merge_expressions(expressions, &vec![TripleExprWrapper { te: tc.clone() }]),
                    min: *min,
                    max: *max,
                    sem_acts: sem_acts.clone(),
                    annotations: annotations.clone(),
                },
                _ => TripleExpr::each_of(vec![te1.clone(), te2.clone()]),
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
                    expressions: Self::merge_expressions(&vec![TripleExprWrapper { te: tc.clone() }], exprs),
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
                _ => TripleExpr::each_of(vec![te1.clone(), te2.clone()]),
            },
            _ => TripleExpr::each_of(vec![te1.clone(), te2.clone()]),
        }
    }

    pub fn merge_expressions(e1: &Vec<TripleExprWrapper>, e2: &Vec<TripleExprWrapper>) -> Vec<TripleExprWrapper> {
        let mut es = Vec::new();
        for e in e1 {
            es.push(e.clone())
        }
        for e in e2 {
            es.push(e.clone())
        }
        es
    }

    pub fn property_shape2triple_constraint(
        &self,
        shape: &IRPropertyShape,
        schema: &IRSchema,
    ) -> Result<TripleExpr, Shacl2ShExError> {
        let predicate = self.shacl_path2predicate(shape.path())?;
        let negated = None;
        let inverse = None;
        let se = self.components2shape_expr(shape.components(), schema)?;
        // sh:minCount and sh:maxCount are the cardinality of the triple
        // constraint. Without them a SHACL property may have any number of
        // values, i.e. `*` in ShEx (whose default is exactly one).
        let mut min = 0;
        let mut max = -1;
        for component in shape.components() {
            match component {
                IRComponent::MinCount(c) => min = i32::try_from(c.min_count()).unwrap_or(i32::MAX),
                IRComponent::MaxCount(c) => max = i32::try_from(c.max_count()).unwrap_or(-1),
                _ => {},
            }
        }
        Ok(TripleExpr::triple_constraint(
            negated,
            inverse,
            predicate,
            se,
            Some(min),
            Some(max),
        ))
    }

    /// The constraints on the values of a property: all its components but
    /// the cardinality (sh:minCount, sh:maxCount), combined with AND.
    pub fn components2shape_expr(
        &self,
        components: &Vec<IRComponent>,
        schema: &IRSchema,
    ) -> Result<Option<ShapeExpr>, Shacl2ShExError> {
        let mut ses = Vec::new();
        for c in components {
            if matches!(c, IRComponent::MinCount(_) | IRComponent::MaxCount(_)) {
                continue;
            }
            ses.push(self.component2shape_expr(c, schema)?);
        }
        Ok(match ses.len() {
            0 => None,
            1 => ses.pop(),
            _ => Some(ShapeExpr::and(ses)),
        })
    }

    pub fn create_class_constraint(&self, cls: &Object) -> Result<ShapeExpr, Shacl2ShExError> {
        let rdf_type = IriRef::iri(IriS::rdf_type());
        let value = match cls {
            Object::Iri(iri) => ValueSetValue::iri(IriRef::iri(iri.clone())),
            Object::BlankNode(_) | Object::Literal(_) | Object::Triple { .. } => {
                return Err(Shacl2ShExError::not_implemented("sh:class values that are not IRIs"));
            },
        };
        let cls = NodeConstraint::new().with_values(vec![value]);
        let te = TripleExpr::triple_constraint(None, None, rdf_type, Some(ShapeExpr::node_constraint(cls)), None, None);
        let se = ShapeExpr::shape(ShExShape::new(None, None, Some(te)));
        Ok(se)
    }

    pub fn component2shape_expr(
        &self,
        component: &IRComponent,
        schema: &IRSchema,
    ) -> Result<ShapeExpr, Shacl2ShExError> {
        match component {
            IRComponent::Class(cls) => {
                // TODO: Converting Class components for {cls:?} doesn't match rdfs:subClassOf semantics of SHACL yet
                let se = self.create_class_constraint(cls.class_rule())?;
                Ok(se)
            },
            IRComponent::Datatype(dt) => Ok(ShapeExpr::node_constraint(
                NodeConstraint::new().with_datatype(dt.datatype().clone().into()),
            )),
            IRComponent::NodeKind(_) => Err(Shacl2ShExError::not_implemented("sh:nodeKind")),
            IRComponent::MinCount(_) => Err(Shacl2ShExError::not_implemented("sh:minCount")),
            IRComponent::MaxCount(_) => Err(Shacl2ShExError::not_implemented("sh:maxCount")),
            IRComponent::MinExclusive(_) => Err(Shacl2ShExError::not_implemented("sh:minExclusive")),
            IRComponent::MaxExclusive(_) => Err(Shacl2ShExError::not_implemented("sh:maxExclusive")),
            IRComponent::MinInclusive(_) => Err(Shacl2ShExError::not_implemented("sh:minInclusive")),
            IRComponent::MaxInclusive(_) => Err(Shacl2ShExError::not_implemented("sh:maxInclusive")),
            IRComponent::MinLength(_) => Err(Shacl2ShExError::not_implemented("sh:minLength")),
            IRComponent::MaxLength(_) => Err(Shacl2ShExError::not_implemented("sh:maxLength")),
            IRComponent::Pattern(_) => Err(Shacl2ShExError::not_implemented("sh:pattern")),
            IRComponent::UniqueLang(_) => Err(Shacl2ShExError::not_implemented("sh:uniqueLang")),
            IRComponent::LanguageIn(_) => Err(Shacl2ShExError::not_implemented("sh:languageIn")),
            IRComponent::Equals(_) => Err(Shacl2ShExError::not_implemented("sh:equals")),
            IRComponent::Disjoint(_) => Err(Shacl2ShExError::not_implemented("sh:disjoint")),
            IRComponent::LessThan(_) => Err(Shacl2ShExError::not_implemented("sh:lessThan")),
            IRComponent::LessThanOrEquals(_) => Err(Shacl2ShExError::not_implemented("sh:lessThanOrEquals")),
            IRComponent::Or(_) => {
                debug!("Not implemented OR Shapes");
                Ok(ShapeExpr::empty_shape())
            },
            IRComponent::And(_) => Err(Shacl2ShExError::not_implemented("sh:and")),
            IRComponent::Not(_) => Err(Shacl2ShExError::not_implemented("sh:not")),
            IRComponent::Xone(_) => Err(Shacl2ShExError::not_implemented("sh:xone")),
            IRComponent::Closed(_) => Err(Shacl2ShExError::not_implemented("sh:closed")),
            // sh:node: a reference to the shape
            IRComponent::Node(node) => match schema.get_shape_from_idx(node.shape()) {
                Some(shape) => Ok(ShapeExpr::shape_ref(self.rdfnode2label(shape.id())?)),
                None => Err(Shacl2ShExError::not_implemented(
                    "sh:node with a shape missing from the schema",
                )),
            },
            IRComponent::HasValue(_) => Err(Shacl2ShExError::not_implemented("sh:hasValue")),
            IRComponent::In(_) => Err(Shacl2ShExError::not_implemented("sh:in")),
            IRComponent::QualifiedValueShape(_) => Err(Shacl2ShExError::not_implemented("sh:qualifiedValueShape")),
            IRComponent::Deactivated(_) => Err(Shacl2ShExError::not_implemented("sh:deactivated")),
            IRComponent::BasicSparql(_) => Err(Shacl2ShExError::not_implemented("sh:sparql")),
        }
    }

    pub fn shacl_path2predicate(&self, path: &SHACLPath) -> Result<IriRef, Shacl2ShExError> {
        match path {
            SHACLPath::Predicate { pred } => Ok(IriRef::iri(pred.clone())),
            _ => Err(Shacl2ShExError::not_implemented(
                "property paths other than a single predicate (sequence, alternative, inverse, ...)",
            )),
        }
    }
}
