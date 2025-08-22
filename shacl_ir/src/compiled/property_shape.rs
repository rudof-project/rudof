use std::collections::HashSet;

use srdf::RDFNode;
use srdf::Rdf;
use srdf::SHACLPath;

use shacl_ast::property_shape::PropertyShape;
use shacl_ast::Schema;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::severity::CompiledSeverity;
use super::shape::CompiledShape;
use super::target::CompiledTarget;

#[derive(Debug, Clone)]
pub struct CompiledPropertyShape {
    id: RDFNode,
    path: SHACLPath,
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
    // order: Option<NumericLiteral>,
    // group: Option<S::Term>,
    // source_iri: Option<S::IRI>,
    // annotations: Vec<(S::IRI, S::Term)>,
}

impl CompiledPropertyShape {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: RDFNode,
        path: SHACLPath,
        components: Vec<CompiledComponent>,
        targets: Vec<CompiledTarget>,
        property_shapes: Vec<CompiledShape>,
        closed: bool,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
    ) -> Self {
        CompiledPropertyShape {
            id,
            path,
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

    pub fn is_closed(&self) -> &bool {
        &self.closed
    }

    pub fn path(&self) -> &SHACLPath {
        &self.path
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
}

impl CompiledPropertyShape {
    pub fn compile<S: Rdf>(
        shape: PropertyShape<S>,
        schema: &Schema<S>,
    ) -> Result<Self, CompiledShaclError> {
        let id = shape.id().clone();
        let path = shape.path().to_owned();
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

        let compiled_property_shape = CompiledPropertyShape::new(
            id,
            path,
            compiled_components,
            targets,
            property_shapes,
            closed,
            deactivated,
            severity,
        );

        Ok(compiled_property_shape)
    }
}
