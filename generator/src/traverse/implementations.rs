use crate::traverse::Traversable;
use crate::traverse::Visitor;
use shex_ast::{
    NodeConstraint, Schema, Shape, ShapeDecl, ShapeExpr, ShapeExprWrapper, TripleExprWrapper,
};
use srdf::SRDFGraph;
use prefixmap::{IriRef, PrefixMap, PrefixMapError};

pub struct ShexVisitor {
    pub rdf: SRDFGraph,
}

impl ShexVisitor {
    pub fn new(rdf: SRDFGraph) -> Self {
        ShexVisitor { rdf }
    }
}

impl Traversable for Schema {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_schema(self);
    }
}

impl Traversable for ShapeDecl {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_shape_decl(self);
    }
}

impl Traversable for ShapeExpr {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            ShapeExpr::Shape(shape) => {
                visitor.visit_shape(shape);
            }
            ShapeExpr::ShapeNot { shape_expr } => {
                visitor.visit_shape_not(shape_expr);
            }
            ShapeExpr::ShapeAnd { shape_exprs } => {
                visitor.visit_shape_and(shape_exprs);
            }
            ShapeExpr::ShapeOr { shape_exprs } => {
                visitor.visit_shape_or(shape_exprs);
            }
            _ => {}
        }
    }
}

impl Traversable for ShapeExprWrapper {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_shape_expr_wrapper(self);
    }
}


impl Traversable for PrefixMap {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_prefix_map(self);
    }
}

//----------------- Implement the Visitor trait for ShexVisitor -----------------

impl Visitor for ShexVisitor {
    fn visit_schema(&mut self, schema: &Schema) {
        println!("Schema visited");
        schema.shapes().unwrap().iter().for_each(|shape_decl| {
            shape_decl.accept(self);
        });

        schema.prefixmap().unwrap().accept(self);
    }

    fn visit_shape_decl(&mut self, shape_decl: &ShapeDecl) {
        println!("ShapeDecl");
        shape_decl.shape_expr.accept(self);
    }

    fn visit_shape(&mut self, shape: &Shape) {
        println!("Shape visited");
        
    }

    fn visit_shape_not(&mut self, shape_not: &Box<ShapeExprWrapper>) {
        println!("ShapeNot visited");
        let shape_expr_wrapper = shape_not.as_ref();
        shape_expr_wrapper.accept(self);
    }

    fn visit_shape_and(&mut self, shape_exprs: &Vec<ShapeExprWrapper>) {
        println!("ShapeAnd visited");
        for shape_expr in shape_exprs {
            shape_expr.accept(self);
        }
    }

    fn visit_shape_or(&mut self, shape_exprs: &Vec<ShapeExprWrapper>) {
        println!("ShapeOr visited");
        for shape_expr in shape_exprs {
            shape_expr.accept(self);
        }
    }

    fn visit_shape_expr_wrapper(&mut self, shape_expr_wrapper: &ShapeExprWrapper) {
        println!("ShapeExprWrapper visited");
        shape_expr_wrapper.se.accept(self);
    }

  
    fn visit_prefix_map(&mut self, prefixmap: &PrefixMap) {
        println!("PrefixMap visited {:?}", prefixmap);
        self.rdf.merge_prefixes(prefixmap.clone()).unwrap();
       
    }
}