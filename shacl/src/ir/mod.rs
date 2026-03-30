//! SHACL IR (Internal Representation)
//! Represents SHACL Internal representation which is used for validation


mod components;
mod shape_label_idx;
mod error;
mod shape;
mod node_shape;
mod property_shape;
mod dependency_graph;
mod schema;
mod reifier_info;
mod test;

pub(crate) use reifier_info::ReifierInfo;
pub(crate) use schema::IRSchema;
