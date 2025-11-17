use crate::shapemap::ValidationStatus;
use crate::{Node, ir::shape_label::ShapeLabel};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShapemapError {
    #[error("PrefixMap error for node {node}: {error}")]
    PrefixMapError { node: String, error: String },

    #[error(
        "Trying to create an inconsistent status on node {node} and shape {label}. Old status: {old_status}, new status: {new_status}"
    )]
    InconsistentStatus {
        node: Box<Node>,
        label: Box<ShapeLabel>,
        old_status: Box<ValidationStatus>,
        new_status: Box<ValidationStatus>,
    },

    #[error("Error running query to select nodes. Query: \n{query}\nError: {error}")]
    NodeSelectorQueryError { query: String, error: String },

    #[error("Obtaining IRI from IriRef {iri_ref}: {prefixmap}\nPrefixmap: {prefixmap}")]
    ResolvingIriRef {
        iri_ref: String,
        prefixmap: String,
        error: String,
    },
}
