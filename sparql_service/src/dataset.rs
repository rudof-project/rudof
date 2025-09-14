use crate::{GraphDescription, NamedGraphDescription};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use srdf::IriOrBlankNode;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct Dataset {
    id: Option<IriOrBlankNode>,
    default_graph: Option<GraphDescription>,
    named_graphs: Vec<NamedGraphDescription>,
}

impl Hash for Dataset {
    // TODO: Review this implementation
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Dataset {
    pub fn new(ib: &IriOrBlankNode) -> Dataset {
        Dataset {
            id: Some(ib.clone()),
            default_graph: None,
            named_graphs: Vec::new(),
        }
    }

    pub fn with_default_graph(mut self, default_graph: Option<GraphDescription>) -> Self {
        self.default_graph = default_graph;
        self
    }

    pub fn with_named_graphs(mut self, named_graphs: Vec<NamedGraphDescription>) -> Self {
        self.named_graphs = named_graphs;
        self
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Dataset: {}",
            self.id
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or("No Id".to_string())
        )?;
        if let Some(default_graph) = &self.default_graph {
            writeln!(f, " default_graph: {default_graph}")?;
        }
        let named_graphs_str = if self.named_graphs.iter().peekable().peek().is_none() {
            "[]".to_string()
        } else {
            format!(
                " named graphs: {}",
                self.named_graphs.iter().map(|ng| ng.to_string()).join("\n")
            )
        };
        writeln!(f, " named_graphs: {named_graphs_str}")?;
        Ok(())
    }
}
