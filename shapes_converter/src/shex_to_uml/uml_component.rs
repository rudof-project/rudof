use std::collections::BTreeSet;

use crate::shex_to_uml::NodeId;

use super::UmlClass;

#[derive(Debug, PartialEq)]
pub enum UmlComponent {
    UmlClass(UmlClass),
    Or { exprs: BTreeSet<NodeId> },
    Not { expr: NodeId },
    And { exprs: BTreeSet<NodeId> },
}

impl UmlComponent {
    pub fn class(class: UmlClass) -> UmlComponent {
        UmlComponent::UmlClass(class)
    }

    pub fn or(nodes: BTreeSet<NodeId>) -> UmlComponent {
        UmlComponent::Or { exprs: nodes }
    }

    pub fn and(nodes: BTreeSet<NodeId>) -> UmlComponent {
        UmlComponent::And { exprs: nodes }
    }

    pub fn not(node: NodeId) -> UmlComponent {
        UmlComponent::Not { expr: node }
    }
}
