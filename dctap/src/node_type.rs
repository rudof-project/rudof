use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum NodeType {
    IRI,
    BNode,
    Literal,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::IRI => write!(f, "IRI"),
            NodeType::BNode => write!(f, "BNode"),
            NodeType::Literal => write!(f, "Literal"),
        }
    }
}
