use std::collections::HashSet;

use srdf::{RDFNode, Rdf};

use shacl_ast::node_shape::NodeShape;
use shacl_ast::Schema;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::severity::CompiledSeverity;
use super::shape::CompiledShape;
use super::target::CompiledTarget;

#[derive(Debug)]
pub struct CompiledNodeShape {
    id: RDFNode,
    components: Vec<CompiledComponent>,
    targets: Vec<CompiledTarget>,
    property_shapes: Vec<CompiledShape>,
    closed: bool,
    // ignored_properties: Vec<S::IRI>,
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
        closed: bool,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
    ) -> Self {
        CompiledNodeShape {
            id,
            components,
            targets,
            property_shapes,
            closed,
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

    pub fn components(&self) -> &Vec<CompiledComponent> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape> {
        &self.property_shapes
    }

    pub fn closed(&self) -> &bool {
        &self.closed
    }
}

impl CompiledNodeShape {
    pub fn compile<S: Rdf>(
        shape: Box<NodeShape<S>>,
        schema: &Schema<S>,
    ) -> Result<Self, CompiledShaclError> {
        let id = shape.id().clone();
        let closed = shape.is_closed().to_owned();
        let deactivated = shape.is_deactivated().to_owned();
        let severity = CompiledSeverity::compile(shape.severity())?;

        let components = shape.components().iter().collect::<HashSet<_>>();
        let mut compiled_components = Vec::new();
        for component in components {
            let component = CompiledComponent::compile(component.to_owned(), schema)?;
            compiled_components.push(component);
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

        let compiled_node_shape = CompiledNodeShape::new(
            id,
            compiled_components,
            targets,
            property_shapes,
            closed,
            deactivated,
            severity,
        );

        Ok(compiled_node_shape)
    }
}
