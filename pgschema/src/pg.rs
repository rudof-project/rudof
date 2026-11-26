use crate::{
    edge::Edge, edge_id::EdgeId, label_set::LabelSet, node::Node, node_id::NodeId, pgs_error::PgsError, record::Record,
    type_name::LabelName, value::Value,
};
use either::Either;
use std::{collections::HashMap, fmt::Display};

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

    /// Returns the number of nodes in this graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in this graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
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

    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn edge(&self, id: &EdgeId) -> Option<&Edge> {
        self.edges.get(id)
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
        labels: impl IntoIterator<Item = LabelName>,
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
            labels: LabelSet::from(labels),
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

impl PropertyGraph {
    /// Serializes the graph in YARS-PG, the syntax it is parsed from: one
    /// node or edge per line, sorted by name, with sorted labels and keys, so
    /// that the output is stable and can be parsed again.
    pub fn to_yarspg(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        for (name, node) in self.named_nodes() {
            lines.push(format!(
                "({}{})",
                yarspg_identifier(name),
                yarspg_labels_record(node.labels(), node.content())
            ));
        }
        for (name, edge) in self.named_edges() {
            let node_name = |id: &NodeId| {
                self.node_name(id)
                    .map(yarspg_identifier)
                    .unwrap_or_else(|| id.to_string())
            };
            let id = name.map(|n| format!("{} ", yarspg_identifier(n))).unwrap_or_default();
            lines.push(format!(
                "({}) - ({}{}) -> ({})",
                node_name(edge.source()),
                id,
                yarspg_labels_record(edge.labels(), edge.content()).trim_start(),
                node_name(edge.target())
            ));
        }
        lines.join("\n")
    }

    /// The graph as JSON: `{"nodes": [...], "edges": [...]}`, where each node
    /// has an `id`, `labels` and `properties` (each key with the list of its
    /// values), and each edge also a `source` and a `target`.
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::json;
        let nodes: Vec<_> = self
            .named_nodes()
            .into_iter()
            .map(|(name, node)| {
                json!({
                    "id": name,
                    "labels": node.labels().iter().collect::<Vec<_>>(),
                    "properties": json_record(node.content()),
                })
            })
            .collect();
        let edges: Vec<_> = self
            .named_edges()
            .into_iter()
            .map(|(name, edge)| {
                json!({
                    "id": name,
                    "source": self.node_name(edge.source()),
                    "target": self.node_name(edge.target()),
                    "labels": edge.labels().iter().collect::<Vec<_>>(),
                    "properties": json_record(edge.content()),
                })
            })
            .collect();
        json!({ "nodes": nodes, "edges": edges })
    }

    fn node_name(&self, id: &NodeId) -> Option<&str> {
        self.node_names
            .iter()
            .find(|(_, node_id)| *node_id == id)
            .map(|(name, _)| name.as_str())
    }

    /// Nodes with their names, sorted by name.
    fn named_nodes(&self) -> Vec<(&str, &Node)> {
        let mut nodes: Vec<_> = self
            .node_names
            .iter()
            .filter_map(|(name, id)| self.nodes.get(id).map(|node| (name.as_str(), node)))
            .collect();
        nodes.sort_by_key(|(name, _)| *name);
        nodes
    }

    /// Edges with their names (`None` for edges without one), sorted by name,
    /// then by source and target.
    fn named_edges(&self) -> Vec<(Option<&str>, &Edge)> {
        let mut edges: Vec<_> = self
            .edges
            .iter()
            .map(|(id, edge)| {
                let name = self
                    .edge_names
                    .iter()
                    .find(|(name, edge_id)| *edge_id == id && !name.is_empty())
                    .map(|(name, _)| name.as_str());
                (name, edge)
            })
            .collect();
        edges.sort_by_key(|(name, edge)| {
            (
                *name,
                self.node_name(edge.source()).map(str::to_string),
                self.node_name(edge.target()).map(str::to_string),
            )
        });
        edges
    }
}

/// A name or label: bare when it is a YARS-PG identifier, quoted otherwise.
fn yarspg_identifier(name: &str) -> String {
    if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        name.to_string()
    } else {
        yarspg_string(name)
    }
}

fn yarspg_string(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\\\""))
}

fn yarspg_value(value: &Value) -> String {
    match value {
        Value::String(s) => yarspg_string(s),
        Value::Integer(i) => i.to_string(),
        Value::Date(d) => format!("DATE \"{d}\""),
        Value::Bool(true) => "TRUE".to_string(),
        Value::Bool(false) => "FALSE".to_string(),
    }
}

/// ` {Label, ...} [key: value, key: [value, ...], ...]`, leaving out empty parts.
fn yarspg_labels_record(labels: &LabelSet, record: &Record) -> String {
    let mut out = String::new();
    if labels.iter().next().is_some() {
        let labels: Vec<_> = labels.iter().map(|l| yarspg_identifier(l)).collect();
        out.push_str(&format!(" {{{}}}", labels.join(", ")));
    }
    let mut properties: Vec<_> = record.iter().collect();
    properties.sort_by_key(|(key, _)| *key);
    if !properties.is_empty() {
        let properties: Vec<_> = properties
            .into_iter()
            .map(|(key, values)| {
                let mut values: Vec<_> = values.iter().collect();
                values.sort();
                let values = match values.as_slice() {
                    [value] => yarspg_value(value),
                    _ => format!(
                        "[{}]",
                        values.iter().map(|v| yarspg_value(v)).collect::<Vec<_>>().join(", ")
                    ),
                };
                format!("{}: {values}", yarspg_identifier(key.str()))
            })
            .collect();
        out.push_str(&format!(" [{}]", properties.join(", ")));
    }
    out
}

fn json_value(value: &Value) -> serde_json::Value {
    match value {
        Value::String(s) => serde_json::Value::from(s.as_str()),
        Value::Integer(i) => serde_json::Value::from(*i),
        Value::Date(d) => serde_json::Value::from(d.to_string()),
        Value::Bool(b) => serde_json::Value::from(*b),
    }
}

fn json_record(record: &Record) -> serde_json::Map<String, serde_json::Value> {
    record
        .iter()
        .map(|(key, values)| {
            let mut values: Vec<_> = values.iter().collect();
            values.sort();
            (
                key.str().to_string(),
                serde_json::Value::Array(values.into_iter().map(json_value).collect()),
            )
        })
        .collect()
}

#[cfg(test)]
mod yarspg_tests {
    use crate::parser::pg_builder::PgBuilder;

    const GRAPH: &str = r#"
(n1 {Person} [ name: "Alice", birthdate: DATE "2010-07-22" ])
(n2 {Person, Student} [ id: 234, name: "Robert \"Bob\" Smith", aliases: ["Bob", "Robbie"], active: TRUE ])
(n3 {Course})
(n1) - (e1 {knows} [since: 2020 ]) -> (n2)
(n2) - ({EnrolledIn}) -> (n3)
"#;

    #[test]
    fn yarspg_round_trip() {
        let graph = PgBuilder::new().parse_pg(GRAPH).unwrap();
        let yarspg = graph.to_yarspg();
        assert_eq!(
            yarspg,
            r#"(n1 {Person} [birthdate: DATE "2010-07-22", name: "Alice"])
(n2 {Person, Student} [active: TRUE, aliases: ["Bob", "Robbie"], id: 234, name: "Robert \"Bob\" Smith"])
(n3 {Course})
(n2) - ({EnrolledIn}) -> (n3)
(n1) - (e1 {knows} [since: 2020]) -> (n2)"#
        );
        let again = PgBuilder::new().parse_pg(&yarspg).unwrap();
        assert_eq!(again.to_yarspg(), yarspg);
        assert_eq!((again.node_count(), again.edge_count()), (3, 2));
    }

    #[test]
    fn json() {
        let graph = PgBuilder::new().parse_pg(GRAPH).unwrap();
        let json = graph.to_json();
        assert_eq!(json["nodes"].as_array().unwrap().len(), 3);
        assert_eq!(json["nodes"][1]["id"], "n2");
        assert_eq!(
            json["nodes"][1]["properties"]["aliases"],
            serde_json::json!(["Bob", "Robbie"])
        );
        assert_eq!(json["nodes"][1]["properties"]["id"], serde_json::json!([234]));
        assert_eq!(json["edges"][1]["id"], "e1");
        assert_eq!(json["edges"][1]["source"], "n1");
        assert_eq!(json["edges"][0]["id"], serde_json::Value::Null);
    }
}
