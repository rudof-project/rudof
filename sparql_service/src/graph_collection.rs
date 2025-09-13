use crate::GraphDescription;
use srdf::IriOrBlankNode;
use std::{collections::HashSet, fmt::Display, hash::Hash};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GraphCollection {
    id: IriOrBlankNode,
    collection: HashSet<GraphDescription>,
}

impl GraphCollection {
    pub fn new(id: &IriOrBlankNode) -> Self {
        GraphCollection {
            id: id.clone(),
            collection: HashSet::new(),
        }
    }
}

impl Hash for GraphCollection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for GraphCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {}", self.id)
    }
}
