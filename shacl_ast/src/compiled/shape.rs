use srdf::SRDFBasic;

use crate::shape::Shape;
use crate::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::node_shape::CompiledNodeShape;
use super::property_shape::CompiledPropertyShape;
use super::target::CompiledTarget;

#[derive(Hash, PartialEq, Eq)]
pub enum CompiledShape<S: SRDFBasic> {
    NodeShape(CompiledNodeShape<S>),
    PropertyShape(CompiledPropertyShape<S>),
}

impl<S: SRDFBasic> CompiledShape<S> {
    pub fn is_deactivated(&self) -> &bool {
        match self {
            CompiledShape::NodeShape(ns) => ns.is_deactivated(),
            CompiledShape::PropertyShape(ps) => ps.is_deactivated(),
        }
    }

    pub fn id(&self) -> &S::Term {
        match self {
            CompiledShape::NodeShape(ns) => ns.id(),
            CompiledShape::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<CompiledTarget<S>> {
        match self {
            CompiledShape::NodeShape(ns) => ns.targets(),
            CompiledShape::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<CompiledComponent<S>> {
        match self {
            CompiledShape::NodeShape(ns) => ns.components(),
            CompiledShape::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape<S>> {
        match self {
            CompiledShape::NodeShape(ns) => ns.property_shapes(),
            CompiledShape::PropertyShape(ps) => ps.property_shapes(),
        }
    }
}

impl<S: SRDFBasic> CompiledShape<S> {
    pub fn compile(shape: Shape, schema: &Schema) -> Result<Self, CompiledShaclError> {
        let shape = match shape {
            Shape::NodeShape(node_shape) => {
                let node_shape = CompiledNodeShape::compile(node_shape, schema)?;
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
