use std::fmt::Display;

use crate::{edge_id::EdgeId, label_set::LabelSet, node_id::NodeId, record::Record, type_name::LabelName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub labels: LabelSet,
    pub properties: Record,
}

impl Edge {
    pub fn new(id: EdgeId, source: NodeId, target: NodeId) -> Self {
        Edge {
            id,
            source,
            target,
            labels: LabelSet::new(),
            properties: Record::new(),
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.labels.insert(label.to_string());
        self
    }

    pub fn with_labels(mut self, labels: impl IntoIterator<Item = LabelName>) -> Self {
        self.labels = LabelSet::from(labels);
        self
    }

    pub fn with_content(mut self, content: &Record) -> Self {
        self.properties = content.clone();
        self
    }

    pub fn labels(&self) -> &LabelSet {
        &self.labels
    }

    pub fn content(&self) -> &Record {
        &self.properties
    }

    pub fn source(&self) -> &NodeId {
        &self.source
    }

    pub fn target(&self) -> &NodeId {
        &self.target
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Edge({}) {}-[{}{}]->{}",
            self.id,
            self.source,
            self.labels.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", "),
            self.properties,
            self.target
        )
    }
}
