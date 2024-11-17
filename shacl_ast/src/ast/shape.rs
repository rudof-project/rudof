use iri_s::IriS;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;
use srdf::model::rdf::Rdf;
use srdf::model::Iri;

use super::component::Component;
use super::node_shape::NodeShape;
use super::property_shape::PropertyShape;
use super::target::Target;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Shape<R: Rdf> {
    NodeShape(NodeShape<R>),
    PropertyShape(PropertyShape<R>),
}

impl<R: Rdf> Shape<R> {
    pub fn is_deactivated(&self) -> &bool {
        match self {
            Shape::NodeShape(ns) => ns.is_deactivated(),
            Shape::PropertyShape(ps) => ps.is_deactivated(),
        }
    }

    pub fn id(&self) -> &TObject<R> {
        match self {
            Shape::NodeShape(ns) => ns.id(),
            Shape::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<Target<R>> {
        match self {
            Shape::NodeShape(ns) => ns.targets(),
            Shape::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<Component<R>> {
        match self {
            Shape::NodeShape(ns) => ns.components(),
            Shape::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<TObject<R>> {
        match self {
            Shape::NodeShape(ns) => ns.property_shapes(),
            Shape::PropertyShape(ps) => ps.property_shapes(),
        }
    }

    pub fn path(&self) -> Option<TObject<R>> {
        match self {
            Shape::NodeShape(_) => None,
            Shape::PropertyShape(_ps) => todo!(),
        }
    }

    pub fn severity(&self) -> TObject<R> {
        let iri_s: IriS = match self {
            Shape::NodeShape(ns) => ns.severity().into(),
            Shape::PropertyShape(ps) => ps.severity().into(),
        };
        TPredicate::<R>::new(iri_s.as_str()).into()
    }
}
