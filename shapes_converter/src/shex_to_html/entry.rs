use serde::Serialize;

use super::{Cardinality, Name, ValueConstraint};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Entry {
    pub name: Name,
    pub value_constraint: ValueConstraint,
    pub card: Cardinality,
}

impl Entry {
    pub fn new(name: Name, value_constraint: ValueConstraint, card: Cardinality) -> Entry {
        Entry {
            name,
            value_constraint,
            card,
        }
    }
}
