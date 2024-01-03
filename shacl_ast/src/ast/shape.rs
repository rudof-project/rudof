use std::fmt::Display;

use crate::{node_shape::NodeShape, property_shape::PropertyShape};

#[derive(Debug, Clone)]
pub enum Shape {
    NodeShape(NodeShape),
    PropertyShape(PropertyShape),
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Shape::NodeShape(ns) => write!(f, "{ns}")?,
            Shape::PropertyShape(ps) => write!(f, "{ps}")?,
        };
        Ok(())
    }
}
