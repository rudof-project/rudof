use crate::shapemap::{ConformantInfo, NonConformantInfo};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Display;

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

    pub fn conformant(reasons: &[String], values: &[Value]) -> ValidationStatus {
        ValidationStatus::Conformant(ConformantInfo::new(reasons, values))
    }

    pub fn non_conformant(errors: &[String], values: &[Value]) -> ValidationStatus {
        ValidationStatus::NonConformant(NonConformantInfo::new(errors, values))
    }

    pub fn pending() -> ValidationStatus {
        ValidationStatus::Pending
    }

    pub fn code(&self) -> String {
        match self {
            ValidationStatus::Conformant(_) => "conformant".to_string(),
            ValidationStatus::NonConformant(_) => "nonconformant".to_string(),
            ValidationStatus::Pending => "pending".to_string(),
            ValidationStatus::Inconsistent(_, _) => "inconsistent".to_string(),
        }
    }

    pub fn app_info(&self) -> Value {
        match self {
            ValidationStatus::Conformant(conformant_info) => {
                serde_json::json!({
                    "status": "conformant",
                    "reason": conformant_info.reason(),
                    "info": conformant_info.app_info()
                })
            }
            ValidationStatus::NonConformant(non_conformant_info) => {
                serde_json::json!({
                    "status": "nonconformant",
                    "reason": non_conformant_info.reason(),
                    "info": non_conformant_info.app_info()
                })
            }
            ValidationStatus::Pending => serde_json::json!({ "status": "pending" }),
            ValidationStatus::Inconsistent(conformant, non_conformant) => {
                serde_json::json!({
                    "status": "inconsistent",
                    "conformant": conformant.app_info(),
                    "non_conformant": non_conformant.app_info()
                })
            }
        }
    }

    pub fn reason(&self) -> String {
        match self {
            ValidationStatus::Conformant(conformant_info) => conformant_info.reason().to_string(),
            ValidationStatus::NonConformant(non_conformant_info) => {
                non_conformant_info.reason().to_string()
            }
            ValidationStatus::Pending => "Pending".to_string(),
            ValidationStatus::Inconsistent(conformant, non_conformant) => {
                format!(
                    "Conformant: {}, Non-conformant: {}",
                    conformant.reason(),
                    non_conformant.reason()
                )
            }
        }
    }

    pub fn merge(&mut self, other: ValidationStatus) {
        match (&self, other) {
            (ValidationStatus::Conformant(c1), ValidationStatus::Conformant(c2)) => {
                *self = ValidationStatus::Conformant(c1.clone().merge(c2))
            }
            (ValidationStatus::NonConformant(nc1), ValidationStatus::NonConformant(nc2)) => {
                *self = ValidationStatus::NonConformant(nc1.clone().merge(nc2));
            }
            (ValidationStatus::Pending, _v) => *self = ValidationStatus::Pending,
            (_v, ValidationStatus::Pending) => *self = ValidationStatus::Pending,
            (ValidationStatus::Conformant(c), ValidationStatus::NonConformant(nc)) => {
                *self = ValidationStatus::Inconsistent(c.clone(), nc.clone())
            }
            (ValidationStatus::NonConformant(nc), ValidationStatus::Conformant(c)) => {
                *self = ValidationStatus::Inconsistent(c.clone(), nc.clone())
            }
            (ValidationStatus::Inconsistent(c1, nc1), ValidationStatus::Inconsistent(c2, nc2)) => {
                *self = ValidationStatus::Inconsistent(
                    c1.clone().merge(c2.clone()),
                    nc1.clone().merge(nc2.clone()),
                )
            }
            (ValidationStatus::Inconsistent(c, nc), ValidationStatus::Conformant(c2)) => {
                *self = ValidationStatus::Inconsistent(c.clone().merge(c2.clone()), nc.clone())
            }
            (ValidationStatus::Inconsistent(c, nc), ValidationStatus::NonConformant(nc2)) => {
                *self = ValidationStatus::Inconsistent(c.clone(), nc.clone().merge(nc2.clone()))
            }
            (ValidationStatus::Conformant(c), ValidationStatus::Inconsistent(c2, nc2)) => {
                *self = ValidationStatus::Inconsistent(c.clone().merge(c2.clone()), nc2.clone());
            }
            (ValidationStatus::NonConformant(nc), ValidationStatus::Inconsistent(c2, nc2)) => {
                *self = ValidationStatus::Inconsistent(c2.clone(), nc.clone().merge(nc2.clone()));
            }
        }
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
