use super::compiled_shacl_error::CompiledShaclError;
use super::component_ir::ComponentIR;
use super::node_shape::NodeShapeIR;
use super::property_shape::PropertyShapeIR;
use super::target::CompiledTarget;
use crate::dependency_graph::{DependencyGraph, PosNeg};
use crate::reifier_info::ReifierInfo;
use crate::schema_ir::SchemaIR;
use crate::severity::CompiledSeverity;
use crate::shape_label_idx::ShapeLabelIdx;
use iri_s::IriS;
use rdf::rdf_core::{Rdf, SHACLPath, term::Object};
use shacl_ast::Schema;
use shacl_ast::shape::Shape;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ShapeIR {
    NodeShape(Box<NodeShapeIR>),
    PropertyShape(Box<PropertyShapeIR>),
}

impl ShapeIR {
    pub fn deactivated(&self) -> bool {
        match self {
            ShapeIR::NodeShape(ns) => ns.deactivated(),
            ShapeIR::PropertyShape(ps) => ps.deactivated(),
        }
    }

    pub fn reifier_info(&self) -> Option<ReifierInfo> {
        match self {
            ShapeIR::NodeShape(_ns) => None,
            ShapeIR::PropertyShape(ps) => ps.reifier_info(),
        }
    }

    pub fn path(&self) -> Option<SHACLPath> {
        match self {
            ShapeIR::NodeShape(_) => None,
            ShapeIR::PropertyShape(ps) => Some(ps.path().clone()),
        }
    }

    pub fn show_severity(&self) -> String {
        if let Some(severity) = self.severity().into() {
            format!("(severity: {severity})")
        } else {
            "(severity: Violation)".to_string()
        }
    }

    pub fn id(&self) -> &Object {
        match self {
            ShapeIR::NodeShape(ns) => ns.id(),
            ShapeIR::PropertyShape(ps) => ps.id(),
        }
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        match self {
            ShapeIR::NodeShape(ns) => ns.targets(),
            ShapeIR::PropertyShape(ps) => ps.targets(),
        }
    }

    pub fn components(&self) -> &Vec<ComponentIR> {
        match self {
            ShapeIR::NodeShape(ns) => ns.components(),
            ShapeIR::PropertyShape(ps) => ps.components(),
        }
    }

    pub fn property_shapes(&self) -> &Vec<ShapeLabelIdx> {
        match self {
            ShapeIR::NodeShape(ns) => ns.property_shapes(),
            ShapeIR::PropertyShape(ps) => ps.property_shapes(),
        }
    }

    pub fn path_str(&self) -> Option<String> {
        match self {
            ShapeIR::NodeShape(_) => None,
            ShapeIR::PropertyShape(ps) => Some(ps.path().to_string()),
        }
    }

    pub fn severity_iri(&self) -> IriS {
        let iri_s: IriS = match self {
            ShapeIR::NodeShape(ns) => ns.severity().iri(),
            ShapeIR::PropertyShape(ps) => ps.severity().iri(),
        };
        iri_s
    }

    pub fn severity(&self) -> CompiledSeverity {
        match self {
            ShapeIR::NodeShape(ns) => ns.severity(),
            ShapeIR::PropertyShape(ps) => ps.severity(),
        }
    }

    pub fn compile<RDF: Rdf>(
        shape: Shape<RDF>,
        schema: &Schema<RDF>,
        idx: &ShapeLabelIdx,
        schema_ir: &mut SchemaIR,
    ) -> Result<ShapeLabelIdx, Box<CompiledShaclError>> {
        let shape_ir = match shape {
            Shape::NodeShape(node_shape) => {
                let node_shape = NodeShapeIR::compile(node_shape, schema, schema_ir)?;
                ShapeIR::NodeShape(Box::new(node_shape))
            },
            Shape::PropertyShape(property_shape) => {
                let property_shape = PropertyShapeIR::compile(*property_shape, schema, schema_ir)?;
                ShapeIR::PropertyShape(Box::new(property_shape))
            },
        };
        let idx = schema_ir.add_shape(*idx, shape_ir)?;
        Ok(idx)
    }

    pub(crate) fn add_edges(
        &self,
        shape_idx: ShapeLabelIdx,
        dg: &mut DependencyGraph,
        posneg: PosNeg,
        schema_ir: &SchemaIR,
        visited: &mut HashSet<ShapeLabelIdx>,
    ) {
        match self {
            ShapeIR::NodeShape(ns) => {
                ns.add_edges(shape_idx, dg, posneg, schema_ir, visited);
            },
            ShapeIR::PropertyShape(ps) => {
                ps.add_edges(shape_idx, dg, posneg, schema_ir, visited);
            },
        }
    }

    pub fn closed(&self) -> bool {
        match self {
            ShapeIR::NodeShape(ns) => ns.closed(),
            ShapeIR::PropertyShape(ps) => ps.closed(),
        }
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        match self {
            ShapeIR::NodeShape(ns) => ns.allowed_properties(),
            ShapeIR::PropertyShape(ps) => ps.allowed_properties(),
        }
    }
}

impl Display for ShapeIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeIR::NodeShape(_shape) => {
                writeln!(f, "NodeShape")?;
            },
            ShapeIR::PropertyShape(shape) => {
                writeln!(f, "PropertyShape")?;
                writeln!(f, " path: {}", shape.path())?;
                if let Some(reifier_info) = shape.reifier_info() {
                    writeln!(
                        f,
                        " reifier info: reification required: {}, reifier shapes: [{}]",
                        reifier_info.reification_required(),
                        reifier_info
                            .reifier_shape()
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
                writeln!(f, "  - {component}")?;
            }
        }
        let mut targets = self.targets().iter().peekable();
        if targets.peek().is_some() {
            writeln!(f, " Targets:")?;
            for target in targets {
                writeln!(f, "  - {target}")?;
            }
        }
        let mut property_shapes = self.property_shapes().iter().peekable();
        if property_shapes.peek().is_some() {
            writeln!(
                f,
                " Property Shapes: [{}]",
                property_shapes.map(|ps| ps.to_string()).collect::<Vec<_>>().join(", ")
            )?;
        }
        Ok(())
    }
}
