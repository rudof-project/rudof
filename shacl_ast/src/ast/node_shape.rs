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
    property_shapes: Vec<Object<R>>,
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
    pub fn new(id: Object<R>) -> Self {
        NodeShape {
            id,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            closed: false,
            // ignored_properties: Vec::new(),
            deactivated: false,
            // message: MessageMap::new(),
            severity: None,
            // name: MessageMap::new(),
            // description: MessageMap::new(),
            // group: None,
            // source_iri: None,
        }
    }

    pub fn with_targets(mut self, targets: Vec<Target<R>>) -> Self {
        self.targets = targets;
        self
    }

    pub fn set_targets(&mut self, targets: Vec<Target<R>>) {
        self.targets = targets;
    }

    pub fn with_property_shapes(mut self, property_shapes: Vec<Object<R>>) -> Self {
        self.property_shapes = property_shapes;
        self
    }

    pub fn with_components(mut self, components: Vec<Component<R>>) -> Self {
        self.components = components;
        self
    }

    pub fn with_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
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

    pub fn property_shapes(&self) -> &Vec<Object<R>> {
        &self.property_shapes
    }

    pub fn closed(&self) -> &bool {
        &self.closed
    }
}
