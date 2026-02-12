use serde::Serialize;

use super::{Cardinality, Name, ValueConstraint};

/// Represents a Shape entry. This value is the one that is referenced from the templates
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ShapeTemplateEntry {
    pub name: Name,
    pub value_constraint: ValueConstraint,
    pub card: Cardinality,
}

impl ShapeTemplateEntry {
    pub fn new(name: Name, value_constraint: ValueConstraint, card: Cardinality) -> ShapeTemplateEntry {
        ShapeTemplateEntry {
            name,
            value_constraint,
            card,
        }
    }
}
