use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use either::Either;

use crate::{
    edge::Edge, edge_id::EdgeId, node::Node, node_id::NodeId, pgs_error::PgsError, record::Record, type_name::LabelName,
};

/// Simple representation of a property graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyGraph {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, Edge>,
    node_names: HashMap<String, NodeId>,
    edge_names: HashMap<String, EdgeId>,
    node_id_counter: usize,
    edge_id_counter: usize,
}

impl Default for PropertyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyGraph {
    /// Creates a new empty PropertyGraph.
    pub fn new() -> Self {
        PropertyGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            node_names: HashMap::new(),
            edge_names: HashMap::new(),
            node_id_counter: 0,
            edge_id_counter: 0,
        }
    }

    /// Merges another PropertyGraph into self.
    /// Nodes are merged by name. New nodes get new IDs.
    /// Edges are remapped to the new node IDs to avoid conflicts.
    pub fn merge(&mut self, other: &PropertyGraph) {
        // Map old node IDs in `other` to new IDs in `self`
        let mut id_map: std::collections::HashMap<NodeId, NodeId> = HashMap::new();

        // Merge nodes
        for (name, other_id) in &other.node_names {
            let other_node = &other.nodes[other_id];

            if let Some(existing_id) = self.node_names.get(name) {
                // Node with this name exists: merge content
                if let Some(existing_node) = self.nodes.get_mut(existing_id) {
                    existing_node.merge(other_node);
                }
                id_map.insert(other_id.clone(), existing_id.clone());
            } else {
                // New node: assign a new ID
                let new_id = NodeId::new(self.node_id_counter);
                self.node_id_counter += 1;

                // Insert new node
                self.node_names.insert(name.clone(), new_id.clone());
                self.nodes
                    .insert(new_id.clone(), other_node.clone().with_id(new_id.clone()));

                id_map.insert(other_id.clone(), new_id);
            }
        }

        // Merge edges
        for (other_edge_id, other_edge) in &other.edges {
            // Remap source/target IDs
            let id_map = id_map.clone();
            let new_source = id_map
                .get(&other_edge.source)
                .expect("Source node must exist in merged graph");
            let new_target = id_map
                .get(&other_edge.target)
                .expect("Target node must exist in merged graph");

            // Assign new edge ID
            let new_edge_id = EdgeId::new(self.edge_id_counter);
            self.edge_id_counter += 1;

            let new_edge = Edge {
                id: new_edge_id.clone(),
                source: new_source.clone(),
                target: new_target.clone(),
                labels: other_edge.labels.clone(),
                properties: other_edge.properties.clone(),
            };

            self.edges.insert(new_edge_id.clone(), new_edge);

            // Insert into edge_names if a name exists
            for (name, edge_id) in &other.edge_names {
                if *edge_id == *other_edge_id {
                    self.edge_names.insert(name.clone(), new_edge_id.clone());
                }
            }
        }
    }

    pub fn get_node_by_label(&self, label: &str) -> Result<&Node, PgsError> {
        let id = self.node_names.get(label).ok_or(PgsError::MissingNodeLabel {
            label: label.to_string(),
        })?;
        self.nodes.get(id).ok_or(PgsError::MissingNodeLabel {
            label: label.to_string(),
        })
    }

    pub fn get_node_edge_by_label(&self, label: &str) -> Result<Either<&Node, &Edge>, PgsError> {
        if let Ok(node) = self.get_node_by_label(label) {
            return Ok(Either::Left(node));
        }
        if let Ok(edge) = self.get_edge_by_label(label) {
            return Ok(Either::Right(edge));
        }
        Err(PgsError::MissingNodeEdgeLabel {
            label: label.to_string(),
        })
    }

    pub fn get_edge_by_label(&self, label: &str) -> Result<&Edge, PgsError> {
        let id = self.edge_names.get(label).ok_or(PgsError::MissingEdgeLabel {
            label: label.to_string(),
        })?;
        self.edges.get(id).ok_or(PgsError::MissingEdgeLabel {
            label: label.to_string(),
        })
    }

    pub fn with_nodes(mut self, nodes: HashMap<NodeId, Node>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_edges(mut self, edges: HashMap<EdgeId, Edge>) -> Self {
        self.edges = edges;
        self
    }

    /// Adds a node to the PropertyGraph.
    pub fn add_node(&mut self, name_id: String, labels: impl IntoIterator<Item = LabelName>, record: Record) {
        let id = NodeId::new(self.node_id_counter);
        self.node_id_counter += 1;
        self.node_names.insert(name_id, id.clone());
        let node = Node::new(id.clone()).with_labels(labels).with_content(&record);
        self.nodes.insert(id, node);
    }

    pub fn get_node_id(&self, label: &str) -> Result<NodeId, PgsError> {
        self.node_names.get(label).cloned().ok_or(PgsError::MissingNodeLabel {
            label: label.to_string(),
        })
    }

    /// Adds an edge to the PropertyGraph.
    pub fn add_edge(
        &mut self,
        name_id: Option<String>,
        source: String,
        labels: HashSet<LabelName>,
        record: Record,
        target: String,
    ) -> Result<(), PgsError> {
        let id = EdgeId::new(self.edge_id_counter);
        self.edge_id_counter += 1;
        self.edge_names.insert(name_id.unwrap_or_default(), id.clone());
        let source_id = self.get_node_id(&source)?;
        let target_id = self.get_node_id(&target)?;
        let edge = Edge {
            id: id.clone(),
            source: source_id,
            labels,
            properties: record,
            target: target_id,
        };
        self.edges.insert(id, edge);
        Ok(())
    }
}

impl Display for PropertyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node_id, node) in self.nodes.iter() {
            let node_id_str = node_id.to_string();
            let node_label = self
                .node_names
                .iter()
                .find(|(_, id)| *id == node_id)
                .map(|(label, _)| label)
                .unwrap_or(&node_id_str);
            writeln!(f, "Node {}: {}", node_label, node)?;
        }
        for (edge_id, edge) in self.edges.iter() {
            let edge_id_str = edge_id.to_string();
            let edge_label = self
                .edge_names
                .iter()
                .find(|(_, id)| *id == edge_id)
                .map(|(label, _)| label)
                .unwrap_or(&edge_id_str);
            writeln!(f, "Edge {}: {}", edge_label, edge)?;
        }
        Ok(())
    }
}
