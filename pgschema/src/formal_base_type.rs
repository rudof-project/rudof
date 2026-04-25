use crate::{
    evidence::Evidence,
    label_set::LabelSet,
    pgs_error::PgsError,
    record::Record,
    record_type::RecordType,
    type_name::{LabelName, Name},
};
use either::Either;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalBaseType {
    labels: HashSet<LabelSet>, // Set of label sets, where each set represents a combination of labels that can be present
    open_labels: bool,
    content: HashSet<RecordType>, // Define the structure of FormalBaseType as needed
}

impl Default for FormalBaseType {
    fn default() -> Self {
        Self::new()
    }
}

impl FormalBaseType {
    /// Creates a new FormalBaseType with no labels or content.
    pub fn new() -> Self {
        FormalBaseType {
            labels: HashSet::new(),
            open_labels: false,
            content: HashSet::new(),
        }
    }

    pub fn with_open(mut self) -> Self {
        self.open_labels = true;
        self
    }

    /// Sets the labels for the FormalBaseType.
    pub fn with_labels(mut self, label_sets: impl IntoIterator<Item = impl IntoIterator<Item = LabelName>>) -> Self {
        for labels in label_sets {
            let label_set = LabelSet::from(labels);
            self.labels.insert(label_set);
        }
        self
    }

    /// Adds a RecordType to the content of the FormalBaseType.
    pub fn with_record_type(mut self, record_type: RecordType) -> Self {
        self.content.insert(record_type);
        self
    }

    /// Sets the content of the FormalBaseType.
    pub fn with_content(mut self, content: HashSet<RecordType>) -> Self {
        self.content = content;
        self
    }

    /// Checks if the FormalBaseType conforms to the given labels and content.
    pub fn conforms(&self, labels: &LabelSet, content: &Record) -> Either<Vec<PgsError>, Vec<Evidence>> {
        /*let conforms_labels = false;
        for label_set in &self.labels {
            // TODO: Check openness of labels
            return Either::Left::<Vec<PgsError>, Vec<Evidence>>(vec![PgsError::LabelsDifferent {
                record_labels: labels.iter().cloned().collect::<Vec<_>>().join(", ").to_string(),
                type_labels: self.labels.iter().cloned().collect::<Vec<_>>().join(", ").to_string(),
            }]);
        }*/
        if !self.labels.contains(labels) {
            let expected_labels = format!(
                "[{}]",
                labels
                    .iter()
                    .map(|lblset| lblset.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            return Either::Left::<Vec<PgsError>, Vec<Evidence>>(vec![PgsError::LabelsDifferent {
                record_labels: labels.iter().cloned().collect::<Vec<_>>().join(", ").to_string(),
                type_labels: expected_labels,
            }]);
        }

        for record_type in &self.content {
            if record_type.conforms(content).is_right() {
                return Either::Right(vec![Evidence::LabelsContentConforms {
                    labels: labels.iter().cloned().collect::<Vec<_>>().join(", ").to_string(),
                    record: format!("{}", content),
                    type_content: format!("{}", record_type),
                }]);
            };
        }
        Either::Left(vec![PgsError::RecordContentFails {
            record: format!("{}", content),
            type_content: self
                .content
                .iter()
                .map(|rt| rt.to_string())
                .collect::<Vec<_>>()
                .join("\n")
                .to_string(),
        }])
    }

    /// Creates a FormalBaseType from a single label.
    pub fn from_label(label: Name) -> Self {
        FormalBaseType::new()
            .with_labels(vec![vec![label]])
            .with_content(HashSet::from([RecordType::empty()]))
    }

    /*pub fn add_label(&mut self, label: Name) {
        self.labels.insert(label);
    }*/

    pub fn add_content(&mut self, record_type: RecordType) {
        self.content.insert(record_type);
    }

    pub fn union(&self, other: &FormalBaseType) -> Self {
        let mut result = FormalBaseType::new();
        result.labels.extend(self.labels.iter().cloned());
        result.labels.extend(other.labels.iter().cloned());
        result.content.extend(self.content.iter().cloned());
        result.content.extend(other.content.iter().cloned());
        result
    }

    pub fn combine(&self, other: &FormalBaseType) -> Self {
        // The following line was using a simple union following the PGSchema paper,
        // but it doesn't capture the intended semantics of combining labels.
        // Instead, we should combine the label sets in a way that reflects the possible combinations of labels from both types.
        // let labels: HashSet<_> = self.labels.union(&other.labels).cloned().collect();
        let labels = combine_label_sets(&self.labels, &other.labels);

        let content = combine_set_records(&self.content, &other.content);
        FormalBaseType {
            labels,
            open_labels: combine_openness(self.open_labels, other.open_labels),
            content,
        }
    }

    pub fn type_0() -> FormalBaseType {
        let mut content = HashSet::new();
        content.insert(RecordType::empty());
        FormalBaseType {
            labels: HashSet::new(),
            open_labels: false,
            content,
        }
    }
}

fn combine_openness(open1: bool, open2: bool) -> bool {
    open1 || open2
}

fn combine_label_sets(set1: &HashSet<LabelSet>, set2: &HashSet<LabelSet>) -> HashSet<LabelSet> {
    if set1.is_empty() {
        return set2.clone();
    }
    if set2.is_empty() {
        return set1.clone();
    }
    let mut combined: HashSet<LabelSet> = HashSet::new();
    for lblset1 in set1 {
        for lblset2 in set2 {
            let mut combined_labels = lblset1.labels().clone();
            combined_labels.extend(lblset2.labels().iter().cloned());
            combined.insert(LabelSet::from(combined_labels));
        }
    }
    combined
}

fn combine_set_records(set1: &HashSet<RecordType>, set2: &HashSet<RecordType>) -> HashSet<RecordType> {
    if set1.is_empty() {
        return set2.clone();
    }
    if set2.is_empty() {
        return set1.clone();
    }
    let mut combined: HashSet<RecordType> = HashSet::new();
    for record1 in set1 {
        for record2 in set2 {
            let combined_record = record1.combine(record2);
            combined.insert(combined_record);
        }
    }
    combined
}

impl Display for FormalBaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormalBaseType { labels, content, .. } if labels.is_empty() && content.is_empty() => write!(f, "Empty"),
            FormalBaseType { labels, content, .. } if !labels.is_empty() && content.is_empty() => {
                write!(
                    f,
                    "Labels([{}])",
                    labels
                        .iter()
                        .map(|lblset| lblset.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            FormalBaseType { labels, content, .. } if labels.is_empty() && !content.is_empty() => {
                write!(
                    f,
                    "Content({})",
                    content.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join(", ")
                )
            },
            FormalBaseType { labels, content, .. } => {
                let spec = if !labels.is_empty() {
                    format!(
                        "Labels({})",
                        labels
                            .iter()
                            .map(|lblset| lblset.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else {
                    String::new()
                };
                let value_spec = if !content.is_empty() {
                    format!(
                        "Content({})",
                        content.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join(", ")
                    )
                } else {
                    String::new()
                };
                if !spec.is_empty() && !value_spec.is_empty() {
                    write!(f, "{}, {}", spec, value_spec)
                } else if !spec.is_empty() {
                    write!(f, "{}", spec)
                } else if !value_spec.is_empty() {
                    write!(f, "{}", value_spec)
                } else {
                    write!(f, "Empty")
                }
            },
        }
    }
}
