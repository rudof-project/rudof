use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

use super::component::Component;
use super::severity::Severity;
use super::shape::Shape;
use super::target::Target;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyShape<R: Rdf> {
    id: Object<R>,
    // path: SHACLPath,
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
    // order: Option<NumericLiteral>,
    // group: Option<R::Term>,
    // source_iri: Option<R::IRI>,
    // annotations: Vec<(R::IRI,Object<R>)>,
}

impl<R: Rdf> PropertyShape<R> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Object<R>,
        // path: SHACLPath,
        components: Vec<Component<R>>,
        targets: Vec<Target<R>>,
        property_shapes: Vec<Shape<R>>,
        closed: bool,
        deactivated: bool,
        severity: Option<Severity<R>>,
    ) -> Self {
        PropertyShape {
            id,
            // path,
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

    pub fn property_shapes(&self) -> &Vec<Shape<R>> {
        &self.property_shapes
    }
}
