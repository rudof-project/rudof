use std::collections::HashSet;

use either::Either;

use crate::{
    evidence::Evidence,
    pgs_error::PgsError,
    record::Record,
    record_type::RecordType,
    type_name::{LabelName, Name},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalBaseType {
    labels: HashSet<LabelName>,
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

    /// Adds a label to the FormalBaseType.
    pub fn with_label(mut self, label: &str) -> Self {
        self.labels.insert(label.to_string());
        self
    }

    /// Sets the labels for the FormalBaseType.
    pub fn with_labels(mut self, labels: HashSet<LabelName>) -> Self {
        self.labels = labels;
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
    pub fn conforms(&self, labels: &HashSet<LabelName>, content: &Record) -> Either<Vec<PgsError>, Vec<Evidence>> {
        if self.labels != *labels {
            // TODO: Check openness of labels
            return Either::Left::<Vec<PgsError>, Vec<Evidence>>(vec![PgsError::LabelsDifferent {
                record_labels: labels.iter().cloned().collect::<Vec<_>>().join(", ").to_string(),
                type_labels: self.labels.iter().cloned().collect::<Vec<_>>().join(", ").to_string(),
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

    /// Creates a FormalBaseType from a label.
    pub fn from_label(label: Name) -> Self {
        FormalBaseType::new()
            .with_labels(HashSet::from([label]))
            .with_content(HashSet::from([RecordType::empty()]))
    }

    pub fn add_label(&mut self, label: Name) {
        self.labels.insert(label);
    }

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
        let labels: HashSet<_> = self.labels.union(&other.labels).cloned().collect();
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

fn combine_set_records(set1: &HashSet<RecordType>, set2: &HashSet<RecordType>) -> HashSet<RecordType> {
    let mut combined: HashSet<RecordType> = HashSet::new();
    for record1 in set1 {
        for record2 in set2 {
            let combined_record = record1.combine(record2);
            combined.insert(combined_record);
        }
    }
    combined
}
