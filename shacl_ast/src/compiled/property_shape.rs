use std::collections::HashSet;

use srdf::Rdf;
use srdf::SHACLPath;

use crate::property_shape::PropertyShape;
use crate::Schema;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::severity::CompiledSeverity;
use super::shape::CompiledShape;
use super::target::CompiledTarget;

#[derive(Debug)]
pub struct CompiledPropertyShape<S: Rdf> {
    id: S::Term,
    path: SHACLPath,
    components: Vec<CompiledComponent<S>>,
    targets: Vec<CompiledTarget<S>>,
    property_shapes: Vec<CompiledShape<S>>,
    closed: bool,
    // ignored_properties: Vec<S::IRI>,
    deactivated: bool,
    // message: MessageMap,
    severity: Option<CompiledSeverity<S>>,
    // name: MessageMap,
    // description: MessageMap,
    // order: Option<NumericLiteral>,
    // group: Option<S::Term>,
    // source_iri: Option<S::IRI>,
    // annotations: Vec<(S::IRI, S::Term)>,
}

impl<S: Rdf> CompiledPropertyShape<S> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: S::Term,
        path: SHACLPath,
        components: Vec<CompiledComponent<S>>,
        targets: Vec<CompiledTarget<S>>,
        property_shapes: Vec<CompiledShape<S>>,
        closed: bool,
        deactivated: bool,
        severity: Option<CompiledSeverity<S>>,
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

    pub fn id(&self) -> &S::Term {
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

    pub fn severity(&self) -> &CompiledSeverity<S> {
        match &self.severity {
            Some(severity) => severity,
            None => &CompiledSeverity::Violation,
        }
    }

    pub fn components(&self) -> &Vec<CompiledComponent<S>> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<CompiledTarget<S>> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape<S>> {
        &self.property_shapes
    }
}

impl<S: Rdf> CompiledPropertyShape<S> {
    pub fn compile(shape: PropertyShape, schema: &Schema) -> Result<Self, CompiledShaclError> {
        let id = shape.id().clone().into();
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
