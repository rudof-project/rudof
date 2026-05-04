use crate::{PartitionsDisplay, Reasons, ValidatorErrors};
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;
use serde::{Serialize, ser::SerializeMap};
use shex_ast::{
    Node, ShapeLabelIdx,
    ir::{node_constraint::NodeConstraint, schema_ir::SchemaIR, shape::Shape, shape_expr::ShapeExpr},
};
use std::{fmt::Display, io};
use termtree::Tree;

/// Reason represents justifications about why a node conforms to some shape
#[derive(Debug, Clone)]
pub enum Reason {
    DescendantShape {
        node: Node,
        shape: ShapeLabelIdx,
        reasons: Reasons,
    },
    ShapeExtends {
        node: Node,
        shape: Box<Shape>,
        reasons: Reasons,
    },
    NodeConstraint {
        node: Node,
        nc: NodeConstraint,
    },
    ShapeAnd {
        node: Node,
        se: Box<ShapeExpr>,
        reasons: Vec<Vec<Reason>>,
    },
    Empty {
        node: Node,
    },
    External {
        node: Node,
    },
    ShapeOr {
        node: Node,
        shape_expr: ShapeLabelIdx,
        reasons: Reasons,
    },
    ShapeNot {
        node: Node,
        shape_expr: ShapeExpr,

        // Errors that are evidences that the negation passess
        errors_evidences: ValidatorErrors,
    },
    Shape {
        node: Node,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
    },
    ShapeRef {
        node: Node,
        idx: ShapeLabelIdx,
    },
    PartitionComponent {
        node: Node,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        maybe_label: Option<ShapeLabelIdx>,
        partition_idx: usize,
        partition: PartitionsDisplay,
        neighs: String,
        reasons: Reasons,
    },
    Partition {
        node: Node,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        partition: String,
        reasons: Reasons,
    },
}

impl Reason {
    // Build a tree representation of the reason,
    // where the root is the main reason and the leaves are the sub-reasons
    fn build_tree(
        &self,
        tree: &mut Tree<String>,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<(), PrefixMapError> {
        match self {
            Reason::NodeConstraint { .. }
            | Reason::Empty { .. }
            | Reason::External { .. }
            | Reason::Shape { .. }
            | Reason::ShapeRef { .. } => Ok(()),
            Reason::ShapeAnd { reasons, .. } => {
                for reason_group in reasons {
                    for r in reason_group {
                        let child_root = r.root_qualified(nodes_prefixmap, schema, width)?;
                        let mut child_tree = Tree::new(child_root);
                        r.build_tree(&mut child_tree, nodes_prefixmap, schema, width)?;
                        tree.leaves.push(child_tree);
                    }
                }
                Ok(())
            },
            Reason::ShapeOr { reasons, .. } => add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width),
            Reason::ShapeNot { errors_evidences, .. } => {
                for err in errors_evidences.iter() {
                    let err_str = err.show_qualified(nodes_prefixmap, schema, width)?;
                    tree.leaves.push(Tree::new(err_str));
                }
                Ok(())
            },
            Reason::ShapeExtends { reasons, .. } => add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width),
            Reason::DescendantShape { reasons, .. } => {
                add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width)
            },
            Reason::PartitionComponent { reasons, .. } => {
                add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width)
            },
            Reason::Partition { reasons, .. } => add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width),
        }
    }

    pub fn root_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        match self {
            Reason::NodeConstraint { node, nc } => Ok(format!(
                "Node {} passes node constraint {nc}",
                node.show_qualified(nodes_prefixmap),
            )),
            Reason::ShapeAnd { node, se, .. } => {
                let s = format!(
                    "AND passed. Node {}, and: {}",
                    node.show_qualified(nodes_prefixmap),
                    schema.show_shape_expr(se, width)
                );
                Ok(s)
            },
            Reason::Shape { node, idx, .. } => {
                let se_str = schema.show_shape_idx(idx, width);
                Ok(format!(
                    "Shape passed {}@{}: {}",
                    node.show_qualified(nodes_prefixmap),
                    schema.show_shape_idx(idx, width),
                    se_str
                ))
            },
            Reason::ShapeExtends { node, shape, .. } => Ok(format!(
                "Extends passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape(shape, width),
            )),
            Reason::DescendantShape { node, shape, .. } => Ok(format!(
                "Descendant shape passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape_idx(shape, width),
            )),
            Reason::Empty { node } => Ok(format!(
                "Node {} passes empty shape",
                node.show_qualified(nodes_prefixmap)
            )),
            Reason::External { node } => Ok(format!(
                "{} passes external shape",
                node.show_qualified(nodes_prefixmap)
            )),
            Reason::ShapeOr { node, shape_expr, .. } => Ok(format!(
                "OR passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape_idx(shape_expr, width),
            )),
            Reason::ShapeNot { node, shape_expr, .. } => Ok(format!(
                "NOT passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape_expr(shape_expr, width),
            )),
            Reason::ShapeRef { node, idx } => Ok(format!(
                "ShapeRef passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape_idx(idx, width)
            )),
            Reason::PartitionComponent {
                node,
                // shape,
                maybe_label,
                partition,
                // neighs,
                ..
            } => Ok(format!(
                "Partition component for {} matches {}: {}",
                node.show_qualified(nodes_prefixmap),
                // schema.show_shape(shape, width),
                maybe_label.map(|l| schema.show_idx(&l)).unwrap_or("Base".to_string()),
                partition.show_qualified(nodes_prefixmap, schema, width)?,
                // neighs,
            )),
            Reason::Partition { node, shape, .. } => Ok(format!(
                "Partition passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape(shape, width),
            )),
        }
    }

    pub fn write_qualified<W: io::Write>(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
        writer: &mut W,
    ) -> Result<(), PrefixMapError> {
        let root_str = self.root_qualified(nodes_prefixmap, schema, width)?;
        let mut tree = Tree::new(root_str);
        self.build_tree(&mut tree, nodes_prefixmap, schema, width)?;
        write!(writer, "{}", tree).map_err(|e| PrefixMapError::IOError { error: e.to_string() })?;
        Ok(())
    }

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        let mut v = Vec::new();
        self.write_qualified(nodes_prefixmap, schema, width, &mut v)?;
        let s = String::from_utf8(v).map_err(|e| PrefixMapError::IOError { error: e.to_string() })?;
        Ok(s)
    }
}

/*impl TreeItem for Reason {
    type Child = Reason;

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        match self {
            Reason::ShapeAndPassed { reasons, .. } => {
                let v: Vec<Reason> = reasons.iter().flatten().cloned().collect();
                std::borrow::Cow::Owned(v)
            }
            Reason::DescendantShapePassed { reasons, .. } => {
                std::borrow::Cow::Owned(reasons.reasons.clone())
            }
            Reason::ShapeExtendsPassed { reasons, .. } => {
                std::borrow::Cow::Owned(reasons.reasons.clone())
            }
            _ => std::borrow::Cow::Borrowed(&[]),
        }
    }

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        _prefix: &str,
        _last: bool,
    ) -> std::io::Result<()> {
        write!(f, "{}")
    }
}*/

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::NodeConstraint { node, nc } => {
                write!(f, "Node constraint passed. Node: {node}, Constraint: {nc}",)
            },
            Reason::ShapeAnd { node, se, reasons } => {
                write!(f, "AND passed. Node {node}, and: {se}, reasons:")?;
                for reason in reasons {
                    write!(f, "[")?;
                    for r in reason {
                        write!(f, "{r}, ")?;
                    }
                    write!(f, "], ")?;
                }
                Ok(())
            },
            Reason::Shape { node, shape, idx } => {
                write!(f, "Shape passed. Node {node}, shape {idx}: {shape}")
            },
            Reason::ShapeOr {
                node,
                shape_expr,
                reasons,
            } => write!(
                f,
                "Shape OR passed. Node {node}, shape: {shape_expr}, reasons: {reasons}"
            ),
            Reason::ShapeNot {
                node,
                shape_expr,
                errors_evidences,
            } => write!(
                f,
                "Shape NOT passed. Node {node}, shape: {shape_expr}, errors: {errors_evidences}"
            ),
            Reason::External { node } => write!(f, "Shape External passed for node {node}"),
            Reason::Empty { node } => write!(f, "Shape Empty passed for node {node}"),
            Reason::ShapeRef { node, idx } => {
                write!(f, "ShapeRef passed. Node {node}, idx: {idx}")
            },
            Reason::ShapeExtends { node, shape, reasons } => write!(
                f,
                "Shape extends passed. Node {node}, shape: {shape}, reasons: {reasons}"
            ),
            Reason::DescendantShape { node, shape, reasons } => write!(
                f,
                "Descendant shapes passed. Node {node}, shape: {shape}, reasons: {reasons}"
            ),
            Reason::PartitionComponent {
                node,
                shape,
                idx,
                maybe_label,
                partition_idx,
                partition,
                neighs,
                reasons,
            } => write!(
                f,
                "Partition component passed. Node {node}, shape: {shape}, idx: {idx}, maybe_label: {}, partition_idx: {}, partition: {}, neighs: {}, reasons: {reasons}",
                maybe_label.map(|l| l.to_string()).unwrap_or("None".to_string()),
                partition_idx,
                partition,
                neighs,
            ),
            Reason::Partition {
                node,
                shape,
                idx,
                partition,
                reasons,
            } => write!(
                f,
                "Partition passed. Node {node}, shape: {shape}, idx: {idx}, partition: {}, reasons: {reasons}",
                partition,
            ),
        }
    }
}

impl Reason {
    pub fn as_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

impl Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("reason", &self.to_string())?;
        map.end()
    }
}

fn add_reasons_to_tree(
    tree: &mut Tree<String>,
    reasons: &Reasons,
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
    width: usize,
) -> Result<(), PrefixMapError> {
    for reason in reasons.iter() {
        let child_root = reason.root_qualified(nodes_prefixmap, schema, width)?;
        let mut child_tree = Tree::new(child_root);
        reason.build_tree(&mut child_tree, nodes_prefixmap, schema, width)?;
        tree.leaves.push(child_tree);
    }
    Ok(())
}
