use srdf::model::rdf::TObject;
use srdf::model::rdf::Rdf;

use crate::property_shape::PropertyShape;
use crate::severity::Severity;
use crate::shacl_path::SHACLPath;
use crate::target::Target;
use crate::Schema;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::shape::CompiledShape;

#[derive(Debug, Clone)]
pub struct CompiledPropertyShape<R: Rdf> {
    id: TObject<R>,
    path: SHACLPath<R::Triple>,
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
    // order: Option<NumericLiteral>,
    // group: Option<Object<R>>,
    // source_iri: Option<R::IRI>,
    // annotations: Vec<(R::IRI, Object<R>)>,
}

impl<R: Rdf> CompiledPropertyShape<R> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: TObject<R>,
        path: SHACLPath<R::Triple>,
        components: Vec<CompiledComponent<R>>,
        targets: Vec<Target<R>>,
        property_shapes: Vec<CompiledShape<R>>,
        closed: bool,
        deactivated: bool,
        severity: Option<Severity<R>>,
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

    pub fn id(&self) -> &TObject<R> {
        &self.id
    }

    pub fn is_closed(&self) -> &bool {
        &self.closed
    }

    pub fn path(&self) -> &SHACLPath<R::Triple> {
        &self.path
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
}

impl<R: Rdf + Clone> CompiledPropertyShape<R> {
    pub fn compile(
        shape: PropertyShape<R>,
        schema: &Schema<R>,
    ) -> Result<Self, CompiledShaclError> {
        let mut compiled_components = Vec::new();
        for component in shape.components() {
            let component = CompiledComponent::compile(component.to_owned(), schema)?;
            compiled_components.push(component);
        }
        let mut property_shapes = Vec::new();
        for property_shape in shape.property_shapes() {
            let shape = compile_shape(property_shape.to_owned(), schema)?;
            property_shapes.push(shape);
        }

        let compiled_property_shape = CompiledPropertyShape::new(
            shape.id().clone(),
            shape.path().clone(),
            compiled_components,
            shape.targets().clone(),
            property_shapes,
            shape.is_closed().clone(),
            shape.is_deactivated().clone(),
            Some(shape.severity().clone()),
        );

        Ok(compiled_property_shape)
    }
}
