use rudof_iri::IriS;
use std::fmt::{Display, Formatter};

/// Closed Constraint Component.
///
/// The RDF data model offers a huge amount of flexibility. Any node can in
/// principle have values for any property. However, in some cases it makes
/// sense to specify conditions on which properties can be applied to nodes.
/// The SHACL Core language includes a property called sh:closed that can be
/// used to specify the condition that each value node has values only for
/// those properties that have been explicitly enumerated via the property
/// shapes specified for the shape via sh:property.
///
/// https://www.w3.org/TR/shacl/#ClosedConstraintComponent
#[derive(Debug, Clone)]
pub struct Closed {
    is_closed: bool,
    ignored_properties: Vec<IriS>,
}

impl Closed {
    pub fn new(is_closed: bool, ignored_properties: Vec<IriS>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<IriS> {
        &self.ignored_properties
    }
}

impl Display for Closed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Closed: is_closed: {}, ignored_properties: [{}]",
            self.is_closed,
            self.ignored_properties()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
