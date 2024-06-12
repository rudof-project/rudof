use srdf::SRDFBuilder;
use std::fmt::Display;

use crate::{node_shape::NodeShape, property_shape::PropertyShape};

#[derive(Debug, Clone)]
pub enum Shape {
    NodeShape(Box<NodeShape>),
    PropertyShape(PropertyShape),
}

impl Shape {
    pub fn write<RDF>(&self, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
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
