use srdf::{BuildRDF, Rdf};
use std::fmt::Display;

use crate::{node_shape::NodeShape, property_shape::PropertyShape};

#[derive(Debug)]
pub enum Shape<RDF: Rdf> {
    NodeShape(Box<NodeShape<RDF>>),
    PropertyShape(Box<PropertyShape<RDF>>),
}

impl<RDF: Rdf> Shape<RDF> {
    // Create a node shape
    pub fn node_shape(ns: NodeShape<RDF>) -> Self {
        Shape::NodeShape(Box::new(ns))
    }

    // Creates a property shape
    pub fn property_shape(ps: PropertyShape<RDF>) -> Self {
        Shape::PropertyShape(Box::new(ps))
    }
    pub fn write<B>(&self, rdf: &mut B) -> Result<(), B::Err>
    where
        B: BuildRDF,
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

impl<RDF: Rdf> Display for Shape<RDF> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Shape::NodeShape(ns) => write!(f, "{ns}"),
            Shape::PropertyShape(ps) => write!(f, "{ps}"),
        }
    }
}

impl<RDF: Rdf> Clone for Shape<RDF> {
    fn clone(&self) -> Self {
        match self {
            Self::NodeShape(ns) => Self::NodeShape((*ns).clone()),
            Self::PropertyShape(ps) => Self::PropertyShape((*ps).clone()),
        }
    }
}

impl<RDF: Rdf> PartialEq for Shape<RDF> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NodeShape(l0), Self::NodeShape(r0)) => l0 == r0,
            (Self::PropertyShape(l0), Self::PropertyShape(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use srdf::SRDFGraph;

    use crate::{node_shape::NodeShape, shape::Shape};

    #[test]
    fn test_clone() {
        let ns: NodeShape<SRDFGraph> =
            NodeShape::new(srdf::Object::Iri(iri!("http://example.org/id")));
        let s1 = Shape::node_shape(ns);
        let s2 = s1.clone();
        assert_eq!(s1, s2)
    }
}
