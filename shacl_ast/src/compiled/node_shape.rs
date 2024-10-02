use srdf::SRDFBasic;

use super::component::Component;
use super::severity::Severity;
use super::shape::Shape;
use super::target::Target;

#[derive(Hash, PartialEq, Eq)]
pub struct NodeShape<S: SRDFBasic> {
    id: S::Term,
    components: Vec<Component<S>>,
    targets: Vec<Target<S>>,
    property_shapes: Vec<Shape<S>>,
    closed: bool,
    // ignored_properties: Vec<S::IRI>,
    deactivated: bool,
    // message: MessageMap,
    severity: Severity<S>,
    // name: MessageMap,
    // description: MessageMap,
    // group: S::Term,
    // source_iri: S::IRI,
}

impl<S: SRDFBasic> NodeShape<S> {
    pub fn new(
        id: S::Term,
        components: Vec<Component<S>>,
        targets: Vec<Target<S>>,
        property_shapes: Vec<Shape<S>>,
        closed: bool,
        deactivated: bool,
        severity: Severity<S>,
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

    pub fn id(&self) -> &S::Term {
        &self.id
    }

    pub fn is_deactivated(&self) -> &bool {
        &self.deactivated
    }

    pub fn severity(&self) -> &Severity<S> {
        &self.severity
    }

    pub fn components(&self) -> &Vec<Component<S>> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<Target<S>> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<Shape<S>> {
        &self.property_shapes
    }

    pub fn closed(&self) -> &bool {
        &self.closed
    }
}
