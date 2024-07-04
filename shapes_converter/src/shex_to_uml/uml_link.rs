use super::{Name, NodeId, UmlCardinality};

#[derive(Debug, PartialEq)]
pub struct UmlLink {
    pub source: NodeId,
    pub target: NodeId,
    pub name: Name,
    pub card: UmlCardinality,
}

impl UmlLink {
    pub fn new(source: NodeId, target: NodeId, name: Name, card: UmlCardinality) -> UmlLink {
        UmlLink {
            source,
            target,
            name,
            card,
        }
    }
}
