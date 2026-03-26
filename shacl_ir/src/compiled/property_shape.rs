// use super::component_ir::ComponentIR;
// use super::severity::CompiledSeverity;
// use super::target::CompiledTarget;
// use crate::closed_info::ClosedInfo;
// use crate::dependency_graph::DependencyGraph;
// use crate::dependency_graph::PosNeg;
// use crate::schema_ir::SchemaIR;
// use crate::shape_label_idx::ShapeLabelIdx;
// use shacl_ast::property_shape::PropertyShape;
// use shacl_ast::ShaclSchema;

// impl PropertyShapeIR {
    // pub(crate) fn add_edges(
    //     &self,
    //     shape_idx: ShapeLabelIdx,
    //     dg: &mut DependencyGraph,
    //     posneg: PosNeg,
    //     schema_ir: &SchemaIR,
    //     visited: &mut HashSet<ShapeLabelIdx>,
    // ) {
    //     for component in &self.components {
    //         component.add_edges(shape_idx, dg, posneg, schema_ir, visited);
    //     }
    //     for property_shape_idx in &self.property_shapes {
    //         if let Some(shape) = schema_ir.get_shape_from_idx(property_shape_idx) {
    //             dg.add_edge(shape_idx, *property_shape_idx, posneg);
    //             if visited.contains(property_shape_idx) {
    //             } else {
    //                 visited.insert(*property_shape_idx);
    //                 shape.add_edges(*property_shape_idx, dg, posneg, schema_ir, visited);
    //             }
    //         }
    //     }
    // }
// }
