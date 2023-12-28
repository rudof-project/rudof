use crate::{node_shape::NodeShape, property_shape::PropertyShape};


#[derive(Debug, Clone)]
pub enum Shape {
    NodeShape(NodeShape),
    PropertyShape(PropertyShape)
}