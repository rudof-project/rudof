use shex_ast::{compiled::shape_label::ShapeLabel, Node};
use thiserror::Error;

use crate::ValidationStatus;

#[derive(Error, Debug)]
pub enum ShapemapError {
    #[error("Trying to create an inconsistent status on node {node} and shape {label}. Old status: {old_status}, new status: {new_status}")]
    InconsistentStatus {
        node: Node,
        label: ShapeLabel,
        old_status: ValidationStatus,
        new_status: ValidationStatus,
    },
}