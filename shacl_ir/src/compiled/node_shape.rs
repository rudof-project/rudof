use std::collections::HashSet;
use std::fmt::Display;

use iri_s::IriS;
use srdf::{RDFNode, Rdf};

use shacl_ast::node_shape::NodeShape;
use shacl_ast::Schema;

use crate::closed_info::ClosedInfo;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::severity::CompiledSeverity;
use super::shape::CompiledShape;
use super::target::CompiledTarget;

#[derive(Debug, Clone)]
pub struct CompiledNodeShape {
    id: RDFNode,
    components: Vec<CompiledComponent>,
    targets: Vec<CompiledTarget>,
    property_shapes: Vec<CompiledShape>,
    closed_info: ClosedInfo,
    deactivated: bool,

    // message: MessageMap,
    severity: Option<CompiledSeverity>,
    // name: MessageMap,
    // description: MessageMap,
    // group: S::Term,
    // source_iri: S::IRI,
}

impl CompiledNodeShape {
    pub fn new(
        id: RDFNode,
        components: Vec<CompiledComponent>,
        targets: Vec<CompiledTarget>,
        property_shapes: Vec<CompiledShape>,
        closed_info: ClosedInfo,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
    ) -> Self {
        CompiledNodeShape {
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

    pub fn is_deactivated(&self) -> &bool {
        &self.deactivated
    }

    pub fn severity(&self) -> &CompiledSeverity {
        match &self.severity {
            Some(severity) => severity,
            None => &CompiledSeverity::Violation,
        }
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        self.closed_info
            .allowed_properties()
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    pub fn components(&self) -> &Vec<CompiledComponent> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape> {
        &self.property_shapes
    }

    pub fn closed(&self) -> bool {
        self.closed_info.is_closed()
    }
}

impl CompiledNodeShape {
    /// Compiles an AST NodeShape to an internal representation NodeShape
    /// It embeds some components like deactivated as boolean attributes of the internal representation of the node shape
    pub fn compile<S: Rdf>(
        shape: Box<NodeShape<S>>,
        schema: &Schema<S>,
    ) -> Result<Self, CompiledShaclError> {
        let id = shape.id().clone();
        let deactivated = shape.is_deactivated().to_owned();
        let severity = CompiledSeverity::compile(shape.severity())?;

        let components = shape.components().iter().collect::<Vec<_>>();
        let mut compiled_components = Vec::new();
        for component in components {
            if let Some(component) = CompiledComponent::compile(component.to_owned(), schema)? {
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
            let shape = compile_shape(property_shape.to_owned(), schema)?;
            property_shapes.push(shape);
        }

        let closed_info = ClosedInfo::get_closed_info_node_shape(&shape, schema)?;

        let compiled_node_shape = CompiledNodeShape::new(
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

impl Display for CompiledNodeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NodeShape {}", self.id)?;
        writeln!(f, " Deactivated: {}", self.deactivated)?;
        writeln!(f, " Severity: {}", self.severity())?;
        writeln!(f, " Closed: {}", self.closed())?;
        writeln!(f, " Components:")?;
        for component in &self.components {
            writeln!(f, "  - {}", component)?;
        }
        writeln!(f, " Targets:")?;
        for target in &self.targets {
            writeln!(f, "  - {}", target)?;
        }
        writeln!(f, " Property Shapes:")?;
        for property_shape in &self.property_shapes {
            writeln!(f, "  - {}", property_shape)?;
        }
        Ok(())
    }
}
