use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

use super::component::Component;
use super::severity::Severity;
use super::shape::Shape;
use super::target::Target;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeShape<R: Rdf> {
    id: Object<R>,
    components: Vec<Component<R>>,
    targets: Vec<Target<R>>,
    property_shapes: Vec<Shape<R>>,
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

impl<R: Rdf> NodeShape<R> {
    pub fn new(
        id: Object<R>,
        components: Vec<Component<R>>,
        targets: Vec<Target<R>>,
        property_shapes: Vec<Shape<R>>,
        closed: bool,
        deactivated: bool,
        severity: Option<Severity<R>>,
    ) -> Self {
        NodeShape {
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

    pub fn components(&self) -> &Vec<Component<R>> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<Target<R>> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<Shape<R>> {
        &self.property_shapes
    }

    pub fn closed(&self) -> &bool {
        &self.closed
    }
}
