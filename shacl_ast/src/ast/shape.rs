use srdf::BuildRDF;
use std::fmt::Display;

use crate::{node_shape::NodeShape, property_shape::PropertyShape};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    NodeShape(Box<NodeShape>),
    PropertyShape(Box<PropertyShape>),
}

impl Shape {
    // Create a node shape
    pub fn node_shape(ns: NodeShape) -> Self {
        Shape::NodeShape(Box::new(ns))
    }

    // Creates a property shape
    pub fn property_shape(ps: PropertyShape) -> Self {
        Shape::PropertyShape(Box::new(ps))
    }
    pub fn write<RDF>(&self, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        match self {
            Shape::NodeShape(ns) => {
                ns.write(rdf)?;
            }
            Shape::PropertyShape(ps) => {
                ps.write(rdf)?;
            }
        }
        Ok(())
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Shape::NodeShape(ns) => write!(f, "{ns}"),
            Shape::PropertyShape(ps) => write!(f, "{ps}"),
        }
    }
}
