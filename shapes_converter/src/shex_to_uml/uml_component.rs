use super::{Name, NodeId};

#[derive(Debug, PartialEq)]
pub enum UmlComponent {
    UmlClass {
        id: NodeId,
        label: String,
        href: Option<String>,
        entries: Vec<UmlEntry>,
    },
}

impl UmlComponent {}

#[derive(Debug, PartialEq)]
pub struct UmlEntry {
    name: Name,
    value_constraint: ValueConstraint,
    card: UMLCardinality,
}

#[derive(Debug, PartialEq)]
pub enum ValueConstraint {
    Any,
}

impl Default for ValueConstraint {
    fn default() -> Self {
        ValueConstraint::Any
    }
}

#[derive(Debug, PartialEq)]
pub enum UMLCardinality {
    OneOne,
}

impl Default for UMLCardinality {
    fn default() -> Self {
        UMLCardinality::OneOne
    }
}
