pub mod implementations;

use shex_ast::{Schema, ShapeDecl, Shape, ShapeExprWrapper, TripleExprWrapper};
use prefixmap::PrefixMap;

pub trait Visitor {
    fn visit_schema(&mut self, schema: &Schema);
    fn visit_shape_decl(&mut self, shape_decl: &ShapeDecl);
    fn visit_shape(&mut self, shape: &Shape);
    fn visit_shape_not(&mut self, shape_expr: &Box<ShapeExprWrapper>);
    fn visit_shape_and(&mut self, shape_exprs: &Vec<ShapeExprWrapper>);
    fn visit_shape_or(&mut self, shape_exprs: &Vec<ShapeExprWrapper>);
    fn visit_shape_expr_wrapper(&mut self, shape_expr_wrapper: &ShapeExprWrapper);
    fn visit_prefix_map(&mut self, prefix_map: &PrefixMap);
    
}

pub trait Traversable {
    fn accept(&self, visitor: &mut dyn Visitor);
}



