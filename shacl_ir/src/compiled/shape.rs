// use super::component_ir::ComponentIR;
// use super::node_shape::NodeShapeIR;
// use super::property_shape::PropertyShapeIR;
// use super::target::CompiledTarget;
// use crate::dependency_graph::{DependencyGraph, PosNeg};
// use crate::schema_ir::SchemaIR;
// use crate::severity::CompiledSeverity;
// use crate::shape_label_idx::ShapeLabelIdx;
// use shacl_ast::shape::Shape;
// use shacl_ast::ShaclSchema;

// impl ShapeIR {
    // pub(crate) fn add_edges(
    //     &self,
    //     shape_idx: ShapeLabelIdx,
    //     dg: &mut DependencyGraph,
    //     posneg: PosNeg,
    //     schema_ir: &SchemaIR,
    //     visited: &mut HashSet<ShapeLabelIdx>,
    // ) {
    //     match self {
    //         ShapeIR::NodeShape(ns) => {
    //             ns.add_edges(shape_idx, dg, posneg, schema_ir, visited);
    //         },
    //         ShapeIR::PropertyShape(ps) => {
    //             ps.add_edges(shape_idx, dg, posneg, schema_ir, visited);
    //         },
    //     }
    // }
// }
