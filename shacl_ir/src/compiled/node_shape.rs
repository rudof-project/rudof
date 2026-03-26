// use super::compile_shape;
// use super::compiled_shacl_error::CompiledShaclError;
// use super::component_ir::ComponentIR;
// use super::severity::CompiledSeverity;
// use super::target::CompiledTarget;
// use crate::{
//     closed_info::ClosedInfo,
//     dependency_graph::{DependencyGraph, PosNeg},
//     schema_ir::SchemaIR,
//     shape_label_idx::ShapeLabelIdx,
// };
// use shacl_ast::node_shape::NodeShape;
// use shacl_ast::ShaclSchema;
// use std::collections::HashSet;
//
// impl NodeShapeIR {
//
//     pub(crate) fn add_edges(
//         &self,
//         shape_idx: ShapeLabelIdx,
//         dg: &mut DependencyGraph,
//         posneg: PosNeg,
//         schema_ir: &SchemaIR,
//         visited: &mut HashSet<ShapeLabelIdx>,
//     ) {
//         for component in &self.components {
//             component.add_edges(shape_idx, dg, posneg, schema_ir, visited);
//         }
//         for property_shape_idx in &self.property_shapes {
//             if let Some(shape) = schema_ir.get_shape_from_idx(property_shape_idx) {
//                 dg.add_edge(shape_idx, *property_shape_idx, posneg);
//                 if visited.contains(property_shape_idx) {
//                 } else {
//                     visited.insert(*property_shape_idx);
//                     shape.add_edges(*property_shape_idx, dg, posneg, schema_ir, visited);
//                 }
//             }
//         }
//     }
// }
