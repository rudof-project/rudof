use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Evidence {
    LabelsContentConforms {
        labels: String,
        record: String,
        type_content: String,
    },
    Any {
        values: String,
    },
    ConditionPassed {
        condition: String,
        values: String,
    },
}

impl Display for Evidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Evidence::LabelsContentConforms {
                labels,
                record,
                type_content,
            } => write!(
                f,
                "Labels match {labels} and record: {record} conforms to {type_content}",
            ),
            Evidence::Any { values } => write!(f, "Values {:?} conform to ANY", values),
            Evidence::ConditionPassed { condition, values } => {
                write!(f, "Condition {condition} passed for values: {values}")
            },
        }
    }
}
