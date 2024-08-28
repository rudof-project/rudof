use super::{Shacl2ShExConfig, Shacl2ShExError};
use iri_s::IriS;
use prefixmap::IriRef;
use shacl_ast::{
    component::Component, node_shape::NodeShape, property_shape::PropertyShape,
    shape::Shape as ShaclShape, Schema as ShaclSchema,
};
use shex_ast::{
    BNode, NodeConstraint, Schema as ShExSchema, Shape as ShExShape, ShapeExpr, ShapeExprLabel,
    TripleExpr, ValueSetValue,
};
use srdf::{Object, RDFNode, SHACLPath};
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

    pub fn convert(&mut self, schema: &ShaclSchema) -> Result<(), Shacl2ShExError> {
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

    pub fn convert_shape(
        &self,
        shape: &NodeShape,
        schema: &ShaclSchema,
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
        }
    }

    pub fn node_shape2shape_expr(
        &self,
        shape: &NodeShape,
        schema: &ShaclSchema,
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
                        node_shape: ns.clone(),
                    }),
                },
            }?
        }
        let is_closed = None; // TODO: Check real value
        let extra = None; // TODO: Check if we could find a way to obtain extras in SHACL ?
        let te = if exprs.is_empty() {
            None
        } else {
            Some(TripleExpr::each_of(exprs))
        };
        let shape = ShExShape::new(is_closed, extra, te);
        Ok(ShapeExpr::shape(shape))
    }

    pub fn property_shape2triple_constraint(
        &self,
        shape: &PropertyShape,
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
        }
    }

    pub fn shacl_path2predicate(&self, path: &SHACLPath) -> Result<IriRef, Shacl2ShExError> {
        match path {
            SHACLPath::Predicate { pred } => Ok(IriRef::iri(pred.clone())),
            _ => todo!(),
        }
    }
}
