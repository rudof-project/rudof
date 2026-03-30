use crate::ast::{ASTSchema, ASTShape};
use crate::ir::component::IRComponent;
use crate::ir::dependency_graph::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::node_shape::IRNodeShape;
use crate::ir::property_shape::IRPropertyShape;
use crate::ir::schema::IRSchema;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::ir::ReifierInfo;
use crate::types::{Severity, Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{BuildRDF, SHACLPath};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) enum IRShape {
    NodeShape(Box<IRNodeShape>),
    PropertyShape(Box<IRPropertyShape>),
}

impl IRShape {
    pub fn deactivated(&self) -> bool {
        match self {
            IRShape::NodeShape(ns) => ns.deactivated(),
            IRShape::PropertyShape(ps) => ps.deactivated(),
        }
    }

    pub fn reifier_info(&self) -> Option<&ReifierInfo> {
        match self {
            IRShape::NodeShape(_) => None,
            IRShape::PropertyShape(ps) => ps.reifier_info(),
        }
    }

    pub fn path(&self) -> Option<&SHACLPath> {
        match self {
            IRShape::NodeShape(_) => None,
            IRShape::PropertyShape(ps) => Some(ps.path())
        }
    }

    pub fn severity(&self) -> Severity {
        match self {
            IRShape::NodeShape(ns) => ns.severity(),
            IRShape::PropertyShape(ps) => ps.severity(),
        }
    }

    pub fn id(&self) -> &Object {
        match self {
            IRShape::NodeShape(ns) => ns.id(),
            IRShape::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<Target> {
        match self {
            IRShape::NodeShape(ns) => ns.targets(),
            IRShape::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<IRComponent> {
        match self {
            IRShape::NodeShape(ns) => ns.components(),
            IRShape::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<ShapeLabelIdx> {
        match self {
            IRShape::NodeShape(ns) => ns.property_shapes(),
            IRShape::PropertyShape(ps) => ps.property_shapes(),
        }
    }

    pub fn closed(&self) -> bool {
        match self {
            IRShape::NodeShape(ns) => ns.closed(),
            IRShape::PropertyShape(ps) => ps.closed(),
        }
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        match self {
            IRShape::NodeShape(ns) => ns.allowed_properties(),
            IRShape::PropertyShape(ps) => ps.allowed_properties(),
        }
    }
}

impl IRShape {
    pub fn compile(shape: &ASTShape, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let shape = match shape {
            ASTShape::NodeShape(shape) => {
                let shape = IRNodeShape::compile(shape, ast, ir)?;
                IRShape::NodeShape(Box::new(shape))
            }
            ASTShape::PropertyShape(shape) => {
                let shape = IRPropertyShape::compile(shape, ast, ir)?;
                IRShape::PropertyShape(Box::new(shape))
            }
        };
        Ok(shape)
    }
}

impl Display for IRShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IRShape::NodeShape(_) => writeln!(f, "NodeShape")?,
            IRShape::PropertyShape(shape) => {
                writeln!(f, "PropertyShape")?;
                writeln!(f, " path: {}", shape.path())?;
                if let Some(reifier_info) = shape.reifier_info() {
                    writeln!(f, " reifier info: reification required: {}, reifier shapes: [{}]",
                        reifier_info.reification_required(),
                        reifier_info.reifier_shape()
                                 .iter()
                                 .map(|s| s.to_string())
                                 .collect::<Vec<_>>()
                                 .join(", ")
                    )?;
                }
            },
        }
        if self.deactivated() {
            writeln!(f, " Deactivated: {}", self.deactivated())?;
        }
        if self.severity() != Severity::Violation {
            writeln!(f, " Severity: {}", self.severity())?;
        }
        if self.closed() {
            writeln!(f, " closed: {}", self.closed())?;
        }
        let mut components = self.components().iter().peekable();
        if components.peek().is_some() {
            writeln!(f, "Components:")?;
            for component in components {
                writeln!(f, " - {component}")?;
            }
        }
        let mut targets = self.targets().iter().peekable();
        if targets.peek().is_some() {
            writeln!(f, "Targets:")?;
            for target in targets {
                writeln!(f, " - {target}")?;
            }
        }
        let mut property_shapes = self.property_shapes().iter().peekable();
        if property_shapes.peek().is_some() {
            writeln!(f, " Property Shapes: [{}]",
                property_shapes
                    .map(|ps| ps.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        Ok(())
    }
}

impl IRShape {

    // TODO - Maybe change error to IRError
    pub fn register<RDF: BuildRDF>(&self, graph: &mut RDF, shapes_map: &HashMap<ShapeLabelIdx, IRShape>) -> Result<(), RDF::Err> {
        match self {
            IRShape::NodeShape(ns) => ns.register(graph, shapes_map),
            IRShape::PropertyShape(ps) => ps.register(graph, shapes_map),
        }
    }
}

impl IRShape {
    pub fn add_edges(&self, idx: ShapeLabelIdx, dg: &mut DependencyGraph, posneg: PosNeg, ir: &IRSchema, cache: &mut HashSet<ShapeLabelIdx>) {
        match self {
            IRShape::NodeShape(ns) => ns.add_edges(idx, dg, posneg, ir, cache),
            IRShape::PropertyShape(ps) => ps.add_edges(idx, dg, posneg, ir, cache),
        }
    }
}
