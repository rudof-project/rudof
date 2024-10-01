use srdf::SRDFBasic;

use super::component::Component;
use super::node_shape::NodeShape;
use super::property_shape::PropertyShape;
use super::target::Target;

#[derive(Hash, PartialEq, Eq)]
pub enum Shape<S: SRDFBasic> {
    NodeShape(NodeShape<S>),
    PropertyShape(PropertyShape<S>),
}

impl<S: SRDFBasic> Shape<S> {
    pub fn is_deactivated(&self) -> &bool {
        match self {
            Shape::NodeShape(ns) => ns.is_deactivated(),
            Shape::PropertyShape(ps) => ps.is_deactivated(),
        }
    }

    pub fn id(&self) -> &S::Term {
        match self {
            Shape::NodeShape(ns) => ns.id(),
            Shape::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<Target<S>> {
        match self {
            Shape::NodeShape(ns) => ns.targets(),
            Shape::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<Component<S>> {
        match self {
            Shape::NodeShape(ns) => ns.components(),
            Shape::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<PropertyShape<S>> {
        match self {
            Shape::NodeShape(ns) => ns.property_shapes(),
            Shape::PropertyShape(ps) => ps.property_shapes(),
        }
    }
}
