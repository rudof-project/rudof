use std::collections::HashSet;
use std::fmt::Display;

use iri_s::IriS;
use srdf::{RDFNode, Rdf, SHACLPath};

use shacl_ast::shape::Shape;
use shacl_ast::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::node_shape::CompiledNodeShape;
use super::property_shape::CompiledPropertyShape;
use super::target::CompiledTarget;

#[derive(Debug, Clone)]
pub enum CompiledShape {
    NodeShape(Box<CompiledNodeShape>),
    PropertyShape(Box<CompiledPropertyShape>),
}

impl CompiledShape {
    pub fn is_deactivated(&self) -> &bool {
        match self {
            CompiledShape::NodeShape(ns) => ns.is_deactivated(),
            CompiledShape::PropertyShape(ps) => ps.is_deactivated(),
        }
    }

    pub fn id(&self) -> &RDFNode {
        match self {
            CompiledShape::NodeShape(ns) => ns.id(),
            CompiledShape::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        match self {
            CompiledShape::NodeShape(ns) => ns.targets(),
            CompiledShape::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<CompiledComponent> {
        match self {
            CompiledShape::NodeShape(ns) => ns.components(),
            CompiledShape::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape> {
        match self {
            CompiledShape::NodeShape(ns) => ns.property_shapes(),
            CompiledShape::PropertyShape(ps) => ps.property_shapes(),
        }
    }

    pub fn path(&self) -> Option<SHACLPath> {
        match self {
            CompiledShape::NodeShape(_) => None,
            CompiledShape::PropertyShape(ps) => Some(ps.path().clone()),
        }
    }

    pub fn path_str(&self) -> Option<String> {
        match self {
            CompiledShape::NodeShape(_) => None,
            CompiledShape::PropertyShape(ps) => Some(ps.path().to_string()),
        }
    }

    pub fn severity(&self) -> IriS {
        let iri_s: IriS = match self {
            CompiledShape::NodeShape(ns) => ns.severity().into(),
            CompiledShape::PropertyShape(ps) => ps.severity().into(),
        };
        iri_s
    }

    pub fn compile<RDF: Rdf>(
        shape: Shape<RDF>,
        schema: &Schema<RDF>,
    ) -> Result<Self, CompiledShaclError> {
        let shape = match shape {
            Shape::NodeShape(node_shape) => {
                let node_shape = CompiledNodeShape::compile(node_shape, schema)?;
                CompiledShape::NodeShape(Box::new(node_shape))
            }
            Shape::PropertyShape(property_shape) => {
                let property_shape = CompiledPropertyShape::compile(*property_shape, schema)?;
                CompiledShape::PropertyShape(Box::new(property_shape))
            }
        };

        Ok(shape)
    }

    pub fn closed(&self) -> bool {
        match self {
            CompiledShape::NodeShape(ns) => ns.closed(),
            CompiledShape::PropertyShape(ps) => ps.closed(),
        }
    }

    pub fn ignored_properties(&self) -> HashSet<IriS> {
        match self {
            CompiledShape::NodeShape(ns) => ns.ignored_properties(),
            CompiledShape::PropertyShape(ps) => ps.ignored_properties(),
        }
    }
}

impl Display for CompiledShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledShape::NodeShape(shape) => write!(f, "{shape}"),
            CompiledShape::PropertyShape(shape) => write!(f, "{shape}"),
        }
    }
}
