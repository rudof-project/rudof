use crate::NamedGraphDescription;
use serde::{Deserialize, Serialize};
use srdf::IriOrBlankNode;
use std::{collections::HashSet, fmt::Display, hash::Hash};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct GraphCollection {
    id: IriOrBlankNode,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    collection: HashSet<NamedGraphDescription>,
}

impl GraphCollection {
    pub fn new(id: &IriOrBlankNode) -> Self {
        GraphCollection {
            id: id.clone(),
            collection: HashSet::new(),
        }
    }

    pub fn with_collection<I: Iterator<Item = NamedGraphDescription>>(mut self, graphs: I) -> Self {
        self.collection = HashSet::from_iter(graphs);
        self
    }
}

impl Hash for GraphCollection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for GraphCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {}", self.id)?;
        for graph in &self.collection {
            writeln!(f, "\nGraph: {}", graph)?;
        }
        Ok(())
    }
}
