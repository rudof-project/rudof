use std::collections::HashSet;

use crate::shex_to_uml::NodeId;

use super::UmlClass;

#[derive(Debug, PartialEq)]
pub enum UmlComponent {
    UmlClass(UmlClass),
    Or { exprs: HashSet<NodeId> },
    Not { expr: Box<UmlComponent> },
    And { exprs: Vec<UmlComponent> },
}

impl UmlComponent {
    pub fn class(class: UmlClass) -> UmlComponent {
        UmlComponent::UmlClass(class)
    }

    pub fn or(nodes: HashSet<NodeId>) -> UmlComponent {
        UmlComponent::Or { exprs: nodes }
    }
}
