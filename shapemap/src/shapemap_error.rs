use shex_ast::{ir::shape_label::ShapeLabel, Node};
use thiserror::Error;

use crate::ValidationStatus;

#[derive(Error, Debug)]
pub enum ShapemapError {
    #[error("Trying to create an inconsistent status on node {node} and shape {label}. Old status: {old_status}, new status: {new_status}")]
    InconsistentStatus {
        node: Box<Node>,
        label: Box<ShapeLabel>,
        old_status: ValidationStatus,
        new_status: ValidationStatus,
    },
}
