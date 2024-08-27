use super::{Shacl2ShExConfig, Shacl2ShExError, StartShapeMode};
use prefixmap::IriRef;
use shacl_ast::{
    node_shape::NodeShape, property_shape::PropertyShape, shape::Shape as ShaclShape,
    Schema as ShaclSchema,
};
use shex_ast::{
    BNode, Schema as ShExSchema, Shape as ShExShape, ShapeExpr, ShapeExprLabel, TripleExpr,
};
use srdf::{object, Object, RDFNode, SHACLPath};

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
                    }
                    ShaclShape::NodeShape(_) => todo!(),
                },
            }
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
        let se = None;
        let min = None;
        let max = None;
        Ok(TripleExpr::triple_constraint(
            negated, inverse, predicate, se, min, max,
        ))
    }

    pub fn shacl_path2predicate(&self, path: &SHACLPath) -> Result<IriRef, Shacl2ShExError> {
        match path {
            SHACLPath::Predicate { pred } => Ok(IriRef::iri(pred.clone())),
            _ => todo!(),
        }
    }
}
