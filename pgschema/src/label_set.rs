use crate::type_name::LabelName;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
};

/// A set of labels, represented as a BTreeSet for deterministic ordering and efficient operations
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LabelSet {
    labels: BTreeSet<LabelName>,
}

impl LabelSet {
    pub fn new() -> Self {
        LabelSet {
            labels: BTreeSet::new(),
        }
    }

    pub fn from(labels: impl IntoIterator<Item = LabelName>) -> Self {
        LabelSet {
            labels: labels.into_iter().collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &LabelName> {
        self.labels.iter()
    }

    pub fn insert(&mut self, label: LabelName) {
        self.labels.insert(label);
    }

    pub fn extend(&mut self, labels: impl IntoIterator<Item = LabelName>) {
        self.labels.extend(labels);
    }

    pub fn labels(&self) -> &BTreeSet<LabelName> {
        &self.labels
    }

    pub fn contains(&self, label: &LabelName) -> bool {
        self.labels.contains(label)
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}

impl Default for LabelSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for LabelSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let labels_str = self
            .labels
            .iter()
            .map(|label| label.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{{{}}}", labels_str)
    }
}

impl Display for LabelSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let labels_str = self
            .labels
            .iter()
            .map(|label| label.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{{{}}}", labels_str)
    }
}
