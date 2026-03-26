//! SHACL IR (Internal Representation)
//! Represents SHACL Internal representation which is used for validation


mod shape_label_idx;
mod error;
mod shape;
mod node_shape;
mod property_shape;
mod reifier_info;

pub(crate) use reifier_info::ReifierInfo;
