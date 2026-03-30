use crate::ast::node_shape::ASTNodeShape;
use crate::ast::property_shape::ASTPropertyShape;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum ASTShape {
    NodeShape(Box<ASTNodeShape>),
    PropertyShape(Box<ASTPropertyShape>),
}

impl ASTShape {
    /// Creates a node shape
    pub fn node_shape(ns: ASTNodeShape) -> Self {
        Self::NodeShape(Box::new(ns))
    }

    /// Creates a property shape
    pub fn property_shape(ps: ASTPropertyShape) -> Self {
        Self::PropertyShape(Box::new(ps))
    }
}

impl Display for ASTShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTShape::NodeShape(ns) => write!(f, "{ns}"),
            ASTShape::PropertyShape(ps) => write!(f, "{ps}"),
        }
    }
}

impl Clone for ASTShape {
    fn clone(&self) -> Self {
        match self {
            ASTShape::NodeShape(ns) => Self::NodeShape((*ns).clone()),
            ASTShape::PropertyShape(ps) => Self::PropertyShape((*ps).clone()),
        }
    }
}

impl PartialEq for ASTShape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NodeShape(l), Self::NodeShape(r)) => l == r,
            (Self::PropertyShape(l), Self::PropertyShape(r)) => l == r,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNodeShape, ASTShape};
    use iri_s::iri;
    use rudof_rdf::rdf_core::term::Object;

    fn test_clone() {
        let ns = ASTNodeShape::new(Object::Iri(iri!("http://example.org/id")));
        let s1 = ASTShape::node_shape(ns);
        let s2 = s1.clone();
        assert_eq!(s1, s2)
    }
}