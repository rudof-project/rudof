use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

use crate::node_shape::NodeShape;
use crate::severity::Severity;
use crate::target::Target;
use crate::Schema;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::shape::CompiledShape;

#[derive(Debug, Clone)]
pub struct CompiledNodeShape<R: Rdf> {
    id: Object<R>,
    components: Vec<CompiledComponent<R>>,
    targets: Vec<Target<R>>,
    property_shapes: Vec<CompiledShape<R>>,
    closed: bool,
    // ignored_properties: Vec<R::IRI>,
    deactivated: bool,
    // message: MessageMap,
    severity: Option<Severity<R>>,
    // name: MessageMap,
    // description: MessageMap,
    // group: Object<R>,
    // source_iri: R::IRI,
}

impl<R: Rdf> CompiledNodeShape<R> {
    pub fn new(
        id: Object<R>,
        components: Vec<CompiledComponent<R>>,
        targets: Vec<Target<R>>,
        property_shapes: Vec<CompiledShape<R>>,
        closed: bool,
        deactivated: bool,
        severity: Option<Severity<R>>,
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

    pub fn id(&self) -> &Object<R> {
        &self.id
    }

    pub fn is_deactivated(&self) -> &bool {
        &self.deactivated
    }

    pub fn severity(&self) -> &Severity<R> {
        match &self.severity {
            Some(severity) => severity,
            None => &Severity::Violation,
        }
    }

    pub fn components(&self) -> &Vec<CompiledComponent<R>> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<Target<R>> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape<R>> {
        &self.property_shapes
    }

    pub fn closed(&self) -> &bool {
        &self.closed
    }
}

impl<R: Rdf> CompiledNodeShape<R> {
    pub fn compile(
        shape: Box<NodeShape<R>>,
        schema: &Schema<R>,
    ) -> Result<Self, CompiledShaclError> {
        let compiled_components = shape
            .components()
            .iter()
            .map(|component| CompiledComponent::compile(component, schema))
            .collect::<Result<Vec<_>, _>>()?;

        let compiled_property_shapes = shape
            .property_shapes()
            .iter()
            .map(|property_shape| compile_shape(property_shape, schema))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CompiledNodeShape::new(
            shape.id(),
            compiled_components,
            shape.targets(),
            compiled_property_shapes,
            shape.closed(),
            shape.is_deactivated(),
            Some(shape.severity()),
        ))
    }
}
