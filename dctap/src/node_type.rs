use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum NodeType {
    Basic(BasicNodeType),
    Or(Vec<BasicNodeType>),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum BasicNodeType {
    Iri,
    BNode,
    Literal,
}

impl NodeType {
    pub fn merge_node_type(&mut self, node_type: &NodeType) -> Self {
        match (self, node_type) {
            (NodeType::Basic(node_type), NodeType::Basic(other)) => {
                NodeType::Or(vec![node_type.clone(), other.clone()])
            },
            (NodeType::Basic(b), NodeType::Or(ns)) => {
                let mut v = vec![b.clone()];
                for n in ns {
                    v.push(n.clone())
                }
                NodeType::Or(v)
            },
            (NodeType::Or(vs), NodeType::Basic(b)) => {
                let mut v: Vec<BasicNodeType> = Vec::new();
                v.append(vs);
                v.push(b.clone());
                NodeType::Or(v)
            },
            (NodeType::Or(vs), NodeType::Or(ts)) => {
                let mut v = Vec::new();
                v.append(vs);
                for t in ts {
                    v.push(t.clone())
                }
                NodeType::Or(v)
            },
        }
    }
}

impl Display for BasicNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicNodeType::Iri => write!(f, "IRI"),
            BasicNodeType::BNode => write!(f, "BlankNode"),
            BasicNodeType::Literal => write!(f, "Literal"),
        }
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Basic(b) => write!(f, "{b}"),
            NodeType::Or(vs) => write!(f, "{}", vs.iter().format(" OR ")),
        }
    }
}
