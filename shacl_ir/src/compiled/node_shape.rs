use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component_ir::ComponentIR;
use super::severity::CompiledSeverity;
use super::target::CompiledTarget;
use crate::{
    closed_info::ClosedInfo,
    dependency_graph::{DependencyGraph, PosNeg},
    schema_ir::SchemaIR,
    shape_label_idx::ShapeLabelIdx,
};
use iri_s::IriS;
use shacl_ast::Schema;
use shacl_ast::node_shape::NodeShape;
use srdf::{RDFNode, Rdf};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NodeShapeIR {
    id: RDFNode,
    components: Vec<ComponentIR>,
    targets: Vec<CompiledTarget>,
    property_shapes: Vec<ShapeLabelIdx>,
    closed_info: ClosedInfo,
    deactivated: bool,

    // message: MessageMap,
    severity: Option<CompiledSeverity>,
    // name: MessageMap,
    // description: MessageMap,
    // group: S::Term,
    // source_iri: S::IRI,
}

impl NodeShapeIR {
    pub fn new(
        id: RDFNode,
        components: Vec<ComponentIR>,
        targets: Vec<CompiledTarget>,
        property_shapes: Vec<ShapeLabelIdx>,
        closed_info: ClosedInfo,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
    ) -> Self {
        NodeShapeIR {
            id,
            components,
            targets,
            property_shapes,
            closed_info,
            deactivated,
            severity,
        }
    }

    pub fn id(&self) -> &RDFNode {
        &self.id
    }

    pub fn deactivated(&self) -> bool {
        self.deactivated
    }

    pub fn severity(&self) -> CompiledSeverity {
        match &self.severity {
            Some(severity) => severity.clone(),
            None => CompiledSeverity::Violation,
        }
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        self.closed_info
            .allowed_properties()
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    pub fn components(&self) -> &Vec<ComponentIR> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<ShapeLabelIdx> {
        &self.property_shapes
    }

    pub fn closed(&self) -> bool {
        self.closed_info.is_closed()
    }

    pub(crate) fn add_edges(
        &self,
        shape_idx: ShapeLabelIdx,
        dg: &mut DependencyGraph,
        posneg: PosNeg,
        schema_ir: &SchemaIR,
        visited: &mut HashSet<ShapeLabelIdx>,
    ) {
        for component in &self.components {
            component.add_edges(shape_idx, dg, posneg, schema_ir, visited);
        }
        for property_shape_idx in &self.property_shapes {
            if let Some(shape) = schema_ir.get_shape_from_idx(property_shape_idx) {
                dg.add_edge(shape_idx, *property_shape_idx, posneg);
                if visited.contains(property_shape_idx) {
                } else {
                    visited.insert(*property_shape_idx);
                    shape.add_edges(*property_shape_idx, dg, posneg, schema_ir, visited);
                }
            }
        }
    }
}

impl NodeShapeIR {
    /// Compiles an AST NodeShape to an internal representation NodeShape
    /// It embeds some components like deactivated as boolean attributes of the internal representation of the node shape
    pub fn compile<S: Rdf>(
        shape: Box<NodeShape<S>>,
        schema: &Schema<S>,
        schema_ir: &mut SchemaIR,
    ) -> Result<Self, Box<CompiledShaclError>> {
        let id = shape.id().clone();
        let deactivated = shape.is_deactivated().to_owned();
        let severity = CompiledSeverity::compile(shape.severity())?;

        let components = shape.components().iter().collect::<Vec<_>>();
        let mut compiled_components = Vec::new();
        for component in components {
            if let Some(component) = ComponentIR::compile(component.to_owned(), schema, schema_ir)?
            {
                compiled_components.push(component);
            }
        }

        let mut targets = Vec::new();
        for target in shape.targets() {
            let ans = CompiledTarget::compile(target.to_owned())?;
            targets.push(ans);
        }

        let mut property_shapes = Vec::new();
        for property_shape in shape.property_shapes() {
            let shape = compile_shape(property_shape, schema, schema_ir)?;
            property_shapes.push(shape);
        }

        let closed_info = ClosedInfo::get_closed_info_node_shape(&shape, schema)
            .map_err(|e| Box::new(CompiledShaclError::ShaclError { source: e }))?;

        let compiled_node_shape = NodeShapeIR::new(
            id,
            compiled_components,
            targets,
            property_shapes,
            closed_info,
            deactivated,
            severity,
        );

        Ok(compiled_node_shape)
    }
}
