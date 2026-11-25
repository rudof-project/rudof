use crate::{PartitionsDisplay, Reasons, ValidatorErrors};
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;
use serde::{Serialize, ser::SerializeMap};
use shex_ast::{
    Node, Pred, ShapeLabelIdx,
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
    ParentShapeMainShapePassed {
        node: Node,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
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
    ParentShapeNodeConstraint {
        node: Node,
        idx: ShapeLabelIdx,
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
        resolver: String,
        rationale: String,
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
    ParentShapePassed {
        node: Node,
        idx: ShapeLabelIdx,
        reasons: Reasons,
    },
    Shape {
        node: Node,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        /// The arcs of `node` the triple expression matched, and the pairs they refer to.
        matched: Matched,
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

/// The part of the data that a successful match of a triple expression used: the
/// arcs of the focus node it matched (an inverse predicate means the value is the
/// subject of the triple) and the `(value, shape)` pairs those arcs refer to, which
/// must conform too.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Matched {
    pub arcs: Vec<(Pred, Node)>,
    pub refs: Vec<(Node, ShapeLabelIdx)>,
}

impl Matched {
    pub fn extend(&mut self, other: Matched) {
        self.arcs.extend(other.arcs);
        self.refs.extend(other.refs);
    }
}

impl Reason {
    /// Collects the data this reason justifies the conformance with: the arcs matched
    /// by its triple expressions, as `(focus node, arcs)`, into `arcs`, and the pairs
    /// it relies on (shape references, and the references of matched arcs) into
    /// `refs`. A negation passes because something does *not* match, so it justifies
    /// nothing with the data.
    pub fn collect_matched(&self, arcs: &mut Vec<(Node, Vec<(Pred, Node)>)>, refs: &mut Vec<(Node, ShapeLabelIdx)>) {
        match self {
            Reason::Shape { node, matched, .. } => {
                arcs.push((node.clone(), matched.arcs.clone()));
                refs.extend(matched.refs.iter().cloned());
            },
            Reason::ShapeRef { node, idx } => refs.push((node.clone(), *idx)),
            Reason::ShapeAnd { reasons, .. } => {
                for r in reasons.iter().flatten() {
                    r.collect_matched(arcs, refs);
                }
            },
            Reason::DescendantShape { reasons, .. }
            | Reason::ParentShapeMainShapePassed { reasons, .. }
            | Reason::ShapeExtends { reasons, .. }
            | Reason::ShapeOr { reasons, .. }
            | Reason::ParentShapePassed { reasons, .. }
            | Reason::PartitionComponent { reasons, .. }
            | Reason::Partition { reasons, .. } => {
                for r in reasons.iter() {
                    r.collect_matched(arcs, refs);
                }
            },
            Reason::ShapeNot { .. }
            | Reason::NodeConstraint { .. }
            | Reason::ParentShapeNodeConstraint { .. }
            | Reason::Empty { .. }
            | Reason::External { .. } => {},
        }
    }

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
            | Reason::ParentShapeNodeConstraint { .. }
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
            Reason::ParentShapePassed { reasons, .. } => {
                add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width)
            },
            Reason::ShapeExtends { reasons, .. } => add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width),
            Reason::DescendantShape { reasons, .. } => {
                add_reasons_to_tree(tree, reasons, nodes_prefixmap, schema, width)
            },
            Reason::ParentShapeMainShapePassed { reasons, .. } => {
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
            Reason::Shape { node, idx, .. } => Ok(format!(
                "Shape passed {}@{}",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape_idx(idx, width),
            )),
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
            Reason::External {
                node,
                resolver,
                rationale,
            } => Ok(format!(
                "{} passes EXTERNAL via '{resolver}': {rationale}",
                node.show_qualified(nodes_prefixmap),
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
                "Reference to {} passes node {}",
                schema.show_shape_idx(idx, width),
                node.show_qualified(nodes_prefixmap)
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
                maybe_label.map_or("Base".to_owned(), |l| schema.show_idx(&l)),
                partition.show_qualified(nodes_prefixmap, schema, width)?,
                // neighs,
            )),
            Reason::Partition { node, shape, .. } => Ok(format!(
                "Partition passed ({}@{})",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape(shape, width),
            )),
            Reason::ParentShapeNodeConstraint { node, idx, nc } => Ok(format!(
                "Node {} passes node constraint {nc} of shape {}",
                node.show_qualified(nodes_prefixmap),
                schema.show_shape_idx(idx, width),
            )),
            Reason::ParentShapeMainShapePassed { node, shape, idx, .. } => {
                let s = format!(
                    "Parent shape {} of shape {} passed for node {}",
                    schema.show_shape(shape, width),
                    schema.show_shape_idx(idx, width),
                    node.show_qualified(nodes_prefixmap),
                );
                Ok(s)
            },
            Reason::ParentShapePassed { node, idx, .. } => Ok(format!(
                "Parent shape {} passed for node {}",
                schema.show_shape_idx(idx, width),
                node.show_qualified(nodes_prefixmap),
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
        write!(writer, "{tree}").map_err(|e| PrefixMapError::IOError { error: e.to_string() })?;
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
            Reason::Shape { node, shape, idx, .. } => {
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
            Reason::External {
                node,
                resolver,
                rationale,
            } => write!(f, "Shape External passed for node {node} via '{resolver}': {rationale}"),
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
                maybe_label.map_or("None".to_owned(), |l| l.to_string()),
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
                "Partition passed. Node {node}, shape: {shape}, idx: {idx}, partition: {partition}, reasons: {reasons}",
            ),
            Reason::ParentShapeNodeConstraint { node, idx, nc } => write!(
                f,
                "Node constraint of parent shape passed. Node {node}, idx: {idx}, node constraint: {nc}",
            ),
            Reason::ParentShapeMainShapePassed {
                node,
                shape,
                idx,
                reasons,
            } => write!(
                f,
                "Parent shape main shape passed. Node {node}, parent shape: {shape}, idx: {idx}, reasons: {reasons}",
            ),
            Reason::ParentShapePassed { node, idx, reasons } => {
                write!(f, "Parent shape passed. Node {node}, idx: {idx}, reasons: {reasons}",)
            },
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
