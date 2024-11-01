use std::fmt::Display;

use serde_derive::Serialize;
use serde_json::Value;

/// Represents the current status of validation
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ValidationStatus {
    Conformant(ConformantInfo),
    NonConformant(NonConformantInfo),
    Pending,
    Inconsistent(ConformantInfo, NonConformantInfo),
}

impl ValidationStatus {
    pub fn is_conformant(&self) -> bool {
        matches!(self, ValidationStatus::Conformant(_))
    }

    pub fn is_non_conformant(&self) -> bool {
        matches!(self, ValidationStatus::NonConformant(_))
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, ValidationStatus::Pending)
    }

    pub fn conformant(reason: String, value: Value) -> ValidationStatus {
        ValidationStatus::Conformant(ConformantInfo {
            reason,
            app_info: value,
        })
    }

    pub fn non_conformant(reason: String, value: Value) -> ValidationStatus {
        ValidationStatus::NonConformant(NonConformantInfo {
            reason,
            app_info: value,
        })
    }

    pub fn pending() -> ValidationStatus {
        ValidationStatus::Pending
    }
}

impl Display for ValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationStatus::Conformant(conformant_info) => {
                write!(f, "Conformant, reason: {conformant_info}")
            }
            ValidationStatus::NonConformant(non_conformant_info) => {
                write!(f, "Non conformant, reason: {non_conformant_info}")
            }
            ValidationStatus::Pending => {
                write!(f, "Pending")
            }
            ValidationStatus::Inconsistent(conformant, inconformant) => {
                write!(
                    f,
                    "Inconsistent, conformant: {conformant}, inconformant: {inconformant}"
                )
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ConformantInfo {
    reason: String,
    app_info: Value,
}

impl ConformantInfo {
    pub fn merge(&self, other: ConformantInfo) -> ConformantInfo {
        let merged_reason = format!("{}\n{}", self.reason, other.reason);
        ConformantInfo {
            reason: merged_reason,
            app_info: self.app_info.clone(),
        }
    }
}

impl Display for ConformantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct NonConformantInfo {
    reason: String,
    app_info: Value,
}

impl Display for NonConformantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
