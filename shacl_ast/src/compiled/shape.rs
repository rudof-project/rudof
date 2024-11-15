use std::hash::Hash;

use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

use crate::severity::Severity;
use crate::shape::Shape;
use crate::target::Target;
use crate::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::node_shape::CompiledNodeShape;
use super::property_shape::CompiledPropertyShape;

#[derive(Debug)]
pub enum CompiledShape<R: Rdf> {
    NodeShape(CompiledNodeShape<R>),
    PropertyShape(CompiledPropertyShape<R>),
}

impl<R: Rdf> CompiledShape<R> {
    pub fn is_deactivated(&self) -> &bool {
        match self {
            CompiledShape::NodeShape(ns) => ns.is_deactivated(),
            CompiledShape::PropertyShape(ps) => ps.is_deactivated(),
        }
    }

    pub fn id(&self) -> &Object<R> {
        match self {
            CompiledShape::NodeShape(ns) => ns.id(),
            CompiledShape::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<Target<R>> {
        match self {
            CompiledShape::NodeShape(ns) => ns.targets(),
            CompiledShape::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<CompiledComponent<R>> {
        match self {
            CompiledShape::NodeShape(ns) => ns.components(),
            CompiledShape::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape<R>> {
        match self {
            CompiledShape::NodeShape(ns) => ns.property_shapes(),
            CompiledShape::PropertyShape(ps) => ps.property_shapes(),
        }
    }

    pub fn path(&self) -> Option<Object<R>> {
        match self {
            CompiledShape::NodeShape(_) => None,
            CompiledShape::PropertyShape(_ps) => todo!(),
        }
    }

    pub fn severity(&self) -> &Severity<R> {
        match self {
            CompiledShape::NodeShape(ns) => ns.severity(),
            CompiledShape::PropertyShape(ps) => ps.severity(),
        }
    }
}

impl<R: Rdf + Eq + Clone + Hash> CompiledShape<R> {
    pub fn compile(shape: Shape<R>, schema: &Schema<R>) -> Result<Self, CompiledShaclError> {
        let shape = match shape {
            Shape::NodeShape(node_shape) => {
                let node_shape = CompiledNodeShape::compile(Box::new(node_shape), schema)?;
                CompiledShape::NodeShape(node_shape)
            }
            Shape::PropertyShape(property_shape) => {
                let property_shape = CompiledPropertyShape::compile(property_shape, schema)?;
                CompiledShape::PropertyShape(property_shape)
            }
        };

        Ok(shape)
    }
}
