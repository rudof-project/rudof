use crate::shapemap::ValidationStatus;
use crate::{Node, ir::shape_label::ShapeLabel};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShapemapError {
    #[error(
        "Trying to create an inconsistent status on node {node} and shape {label}. Old status: {old_status}, new status: {new_status}"
    )]
    InconsistentStatus {
        node: Box<Node>,
        label: Box<ShapeLabel>,
        old_status: ValidationStatus,
        new_status: ValidationStatus,
    },
}
