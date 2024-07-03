use super::{Name, NodeId};

#[derive(Debug, PartialEq)]
pub struct UmlLink {
    source: NodeId,
    target: NodeId,
    name: Name,
}

impl UmlLink {}
