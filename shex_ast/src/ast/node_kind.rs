use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Iri,
    BNode,
    NonLiteral,
    Literal,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Iri => write!(f, "IRI"),
            NodeKind::BNode => write!(f, "BNode"),
            NodeKind::NonLiteral => write!(f, "NonLiteral"),
            NodeKind::Literal => write!(f, "Literal"),
        }
    }
}
