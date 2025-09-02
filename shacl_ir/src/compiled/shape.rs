use crate::severity::CompiledSeverity;

use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::node_shape::CompiledNodeShape;
use super::property_shape::CompiledPropertyShape;
use super::target::CompiledTarget;
use iri_s::IriS;
use shacl_ast::shape::Shape;
use shacl_ast::Schema;
use srdf::{RDFNode, Rdf, SHACLPath};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum CompiledShape {
    NodeShape(Box<CompiledNodeShape>),
    PropertyShape(Box<CompiledPropertyShape>),
}

impl CompiledShape {
    pub fn deactivated(&self) -> bool {
        match self {
            CompiledShape::NodeShape(ns) => ns.deactivated(),
            CompiledShape::PropertyShape(ps) => ps.deactivated(),
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

    pub fn severity_iri(&self) -> IriS {
        let iri_s: IriS = match self {
            CompiledShape::NodeShape(ns) => ns.severity().iri(),
            CompiledShape::PropertyShape(ps) => ps.severity().iri(),
        };
        iri_s
    }

    pub fn severity(&self) -> CompiledSeverity {
        match self {
            CompiledShape::NodeShape(ns) => ns.severity(),
            CompiledShape::PropertyShape(ps) => ps.severity(),
        }
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

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        match self {
            CompiledShape::NodeShape(ns) => ns.allowed_properties(),
            CompiledShape::PropertyShape(ps) => ps.allowed_properties(),
        }
    }
}

impl Display for CompiledShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledShape::NodeShape(_shape) => {
                writeln!(f, "NodeShape")?;
            }
            CompiledShape::PropertyShape(shape) => {
                writeln!(f, "PropertyShape")?;
                writeln!(f, " path: {}", shape.path())?;
            }
        }
        if self.deactivated() {
            writeln!(f, " Deactivated: {}", self.deactivated())?;
        }
        if self.severity() != CompiledSeverity::Violation {
            writeln!(f, " Severity: {}", self.severity())?;
        }
        if self.closed() {
            writeln!(f, " closed: {}", self.closed())?;
        }
        let mut components = self.components().iter().peekable();
        if components.peek().is_some() {
            writeln!(f, " Components:")?;
            for component in components {
                writeln!(f, "  - {}", component)?;
            }
        }
        let mut targets = self.targets().iter().peekable();
        if targets.peek().is_some() {
            writeln!(f, " Targets:")?;
            for target in targets {
                writeln!(f, "  - {}", target)?;
            }
        }
        let mut property_shapes = self.property_shapes().iter().peekable();
        if property_shapes.peek().is_some() {
            writeln!(
                f,
                " Property Shapes: [{}]",
                property_shapes
                    .map(|ps| ps.id().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
