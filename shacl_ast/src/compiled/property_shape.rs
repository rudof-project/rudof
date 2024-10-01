use srdf::SHACLPath;
use srdf::SRDFBasic;

use super::component::Component;
use super::severity::Severity;
use super::target::Target;

#[derive(Hash, PartialEq, Eq)]
pub struct PropertyShape<S: SRDFBasic> {
    id: S::Term,
    path: SHACLPath,
    components: Vec<Component<S>>,
    targets: Vec<Target<S>>,
    property_shapes: Vec<PropertyShape<S>>,
    closed: bool,
    // ignored_properties: Vec<S::IRI>,
    deactivated: bool,
    // message: MessageMap,
    severity: Severity<S>,
    // name: MessageMap,
    // description: MessageMap,
    // order: Option<NumericLiteral>,
    // group: Option<S::Term>,
    // source_iri: Option<S::IRI>,
    // annotations: Vec<(S::IRI, S::Term)>,
}

impl<S: SRDFBasic> PropertyShape<S> {
    pub fn new(
        id: S::Term,
        path: SHACLPath,
        components: Vec<Component<S>>,
        targets: Vec<Target<S>>,
        property_shapes: Vec<PropertyShape<S>>,
        closed: bool,
        deactivated: bool,
        severity: Severity<S>,
    ) -> Self {
        PropertyShape {
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

    pub fn path(&self) -> &SHACLPath {
        &self.path
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

    pub fn property_shapes(&self) -> &Vec<PropertyShape<S>> {
        &self.property_shapes
    }
}
