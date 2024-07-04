use super::{Name, UmlCardinality, ValueConstraint};

#[derive(Debug, PartialEq)]
pub struct UmlEntry {
    pub name: Name,
    pub value_constraint: ValueConstraint,
    pub card: UmlCardinality,
}

impl UmlEntry {
    pub fn new(name: Name, value_constraint: ValueConstraint, card: UmlCardinality) -> UmlEntry {
        UmlEntry {
            name,
            value_constraint,
            card,
        }
    }
}
