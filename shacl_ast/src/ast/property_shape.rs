use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

use super::component::Component;
use super::severity::Severity;
use super::shacl_path::SHACLPath;
use super::shape::Shape;
use super::target::Target;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyShape<R: Rdf> {
    id: Object<R>,
    path: SHACLPath<R::Triple>,
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
    // order: Option<NumericLiteral>,
    // group: Option<R::Term>,
    // source_iri: Option<R::IRI>,
    // annotations: Vec<(R::IRI,Object<R>)>,
}

impl<R: Rdf> PropertyShape<R> {
    pub fn new(id: Object<R>, path: SHACLPath<R::Triple>) -> Self {
        PropertyShape {
            id,
            path,
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
            // order: None,
            // group: None,
            // source_iri: None,
            // annotations: Vec::new()
        }
    }

    // pub fn with_name(mut self, name: MessageMap) -> Self {
    //     self.name = name;
    //     self
    // }

    // pub fn with_description(mut self, description: MessageMap) -> Self {
    //     self.description = description;
    //     self
    // }

    // pub fn with_order(mut self, order: Option<NumericLiteral>) -> Self {
    //     self.order = order;
    //     self
    // }

    // pub fn with_group(mut self, group: Option<RDFNode>) -> Self {
    //     self.group = group;
    //     self
    // }

    pub fn with_targets(mut self, targets: Vec<Target<R>>) -> Self {
        self.targets = targets;
        self
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

    pub fn with_severity(mut self, severity: Option<Severity<R>>) -> Self {
        self.severity = severity;
        self
    }

    pub fn id(&self) -> &Object<R> {
        &self.id
    }

    pub fn is_closed(&self) -> &bool {
        &self.closed
    }

    // pub fn path(&self) -> &SHACLPath {
    //     &self.path
    // }

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
}
