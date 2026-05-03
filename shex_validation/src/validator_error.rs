use crate::PartitionsDisplay;
use crate::Reasons;
use crate::ValidatorErrors;
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;
use rbe::RbeError;
use rudof_rdf::rdf_core::term::Object;
use serde::Serialize;
use serde::ser::SerializeMap;
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_action_context::SemanticActionContext;
use shex_ast::ir::shape::Shape;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::{Node, Pred, ShapeExprLabel, ShapeLabelIdx, ir::shape_label::ShapeLabel};
use termtree::Tree;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ValidatorError {
    #[error("Partition component failed ({node}@{idx}).\nPartition:\n{partition}\nErrors:\n{errors}")]
    PartitionComponentFailed {
        node: Box<Node>,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        maybe_label: Option<ShapeLabelIdx>,
        partition_idx: usize,
        partition: PartitionsDisplay,
        neighs: String,
        errors: ValidatorErrors,
    },

    #[error("Partition failed {node}@{idx}.\nPartition:\n{partition}\nErrors:\n{errors}")]
    PartitionFailed {
        node: Box<Node>,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        partition: PartitionsDisplay,
        errors: ValidatorErrors,
    },

    #[error("No partitions remaining for {node}@!{idx}")]
    ShapeFailedNoPartitions {
        node: Box<Node>,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
    },
    #[error("Filling node {node} from node selector to validate: {error}")]
    FillingShapeMapNodes { node: String, error: String },
    #[error(
        "Shape {idx} is abstract and cannot be used in validation for node {node}. Descendants failed with errors: {errors}"
    )]
    AbstractShapeError {
        idx: ShapeLabelIdx,
        node: Box<Node>,
        errors: ValidatorErrors,
    },

    #[error("Error in descendant {desc} of shape {current} for node {node}: {errors}")]
    DescendantShapeError {
        current: ShapeLabelIdx,
        desc: ShapeLabelIdx,
        node: Box<Node>,
        errors: ValidatorErrors,
    },

    #[error("All descendants of shape {idx} failed for node {node}: {errors}")]
    DescendantsShapeError {
        idx: ShapeLabelIdx,
        node: Box<Node>,
        errors: ValidatorErrors,
    },

    #[error("Shape {idx} is abstract and has no descendants")]
    AbstractShapeNoDescendants { idx: ShapeLabelIdx },

    #[error("Creating shapemap from node {node} and shape {shape} failed with errors: {error}")]
    NodeShapeError { node: String, shape: String, error: String },
    #[error("Converting Term to RDFNode failed pending {term}")]
    TermToRDFNodeFailed { term: String },

    #[error("Serialization of reason failed: {reason} with error: {error}")]
    ReasonSerializationError { reason: String, error: String },

    #[error("Serialization of error failed: {source_error} with error: {error}")]
    ErrorSerializationError { source_error: String, error: String },

    #[error("References failed: Shape pattern matches, but references failed: {}", failed_pending.iter().map(|(n, s)| format!("({n}, {s})")).collect::<Vec<_>>().join(", "))]
    FailedPending { failed_pending: Vec<(Node, ShapeLabelIdx)> },
    #[error("Negation cycle error: {neg_cycles:?}")]
    NegCycleError {
        neg_cycles: Vec<Vec<(String, String, Vec<String>)>>,
    },

    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel { shape: ShapeLabel },

    #[error("Not found shape label with index {idx}")]
    NotFoundShapeLabelWithIndex { idx: ShapeLabelIdx },

    #[error("Error converting object to iri: {object}")]
    ConversionObjectIri { object: Object },

    #[error("Compiling schema: {error}")]
    SchemaIRError { error: String },

    #[error("Shapemap error: {error}")]
    ShapeMapError { error: String },

    #[error("Failed regular expression")]
    RbeFailed(),

    #[error(
        "Closed shape but found properties {remainder:?} which are not part of shape declared properties: {declared:?}"
    )]
    ClosedShapeWithRemainderPreds { remainder: Preds, declared: Preds },

    #[error(transparent)]
    RbeError(#[from] RbeError<Pred, Node, ShapeLabelIdx, SemanticActionContext>),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),

    #[error("ShapeLabel not found {shape_label}: {error}")]
    ShapeLabelNotFoundError { shape_label: ShapeExprLabel, error: String },

    #[error("And error: shape expression {shape_expr} failed for node {node}: {errors}")]
    ShapeAndError {
        shape_expr: ShapeLabelIdx,
        node: Box<Node>,
        errors: ValidatorErrors,
    },

    #[error("OR error: shape expression {shape_expr} failed for node {node}: all branches failed")]
    ShapeOrError {
        shape_expr: Box<ShapeExpr>,
        node: Box<Node>,
        errors: Vec<(ShapeLabelIdx, ValidatorErrors)>,
    },

    #[error("Shape Not error: failed for node {node} because it passed {shape_expr} with {reasons}")]
    ShapeNotError {
        shape_expr: Box<ShapeExpr>,
        node: Box<Node>,
        reasons: Reasons,
    },

    #[error("Error reading config file from path {path}: {error}")]
    ValidatorConfigFromPathError { path: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    ValidatorConfigTomlError { path: String, error: String },

    #[error("Adding non conformant {node}@{label} error: {error}")]
    AddingNonConformantError { node: String, label: String, error: String },

    #[error("Adding conformant {node}@{label} error: {error}")]
    AddingConformantError { node: String, label: String, error: String },

    #[error("Adding pending {node}@{label} error: {error}")]
    AddingPendingError { node: String, label: String, error: String },

    #[error("Shape not found for index {idx}")]
    ShapeExprNotFound { idx: ShapeLabelIdx },

    #[error("Shape {idx} failed for node {node} with errors {}", errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))]
    ShapeFailed {
        node: Box<Node>,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        errors: Vec<ValidatorError>,
    },

    #[error("Shape {idx} failed for node {node}: no candidates matched the expression against the given neighbors")]
    NoMatchesFound {
        node: Box<Node>,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
    },

    #[error("ShapeRef fails for node {node} with idx: {idx}")]
    ShapeRefFailed { node: Box<Node>, idx: ShapeLabelIdx },
}

fn add_errors_to_tree(
    tree: &mut Tree<String>,
    errors: &ValidatorErrors,
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
) -> Result<(), PrefixMapError> {
    for err in errors.iter() {
        let child_root = err.root_qualified(nodes_prefixmap, schema)?;
        let mut child_tree = Tree::new(child_root);
        err.build_tree(&mut child_tree, nodes_prefixmap, schema)?;
        tree.leaves.push(child_tree);
    }
    Ok(())
}

impl ValidatorError {
    fn show_idx(idx: &ShapeLabelIdx, schema: &SchemaIR) -> String {
        schema
            .shape_label_from_idx(idx)
            .map(|l| schema.show_label(l))
            .unwrap_or_else(|| idx.to_string())
    }

    fn root_qualified(&self, nodes_prefixmap: &PrefixMap, schema: &SchemaIR) -> Result<String, PrefixMapError> {
        let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
        let show_idx = |idx: &ShapeLabelIdx| Self::show_idx(idx, schema);

        let s = match self {
            ValidatorError::ShapeFailed { node, idx, .. } => {
                format!("Shape {} failed for node {}", show_idx(idx), show_node(node))
            },
            ValidatorError::NoMatchesFound { node, idx, .. } => format!(
                "Shape {} failed for node {}: no candidates matched the expression",
                show_idx(idx),
                show_node(node)
            ),
            ValidatorError::PartitionComponentFailed {
                node, idx, partition, ..
            } => {
                format!(
                    "Partition component failed ({}@{}):\nPartition:\n{}",
                    show_node(node),
                    show_idx(idx),
                    partition.show_qualified(nodes_prefixmap, schema)?
                )
            },
            ValidatorError::PartitionFailed {
                node, idx, partition, ..
            } => {
                format!(
                    "Partition failed {}@{}:\nPartition:\n{}",
                    show_node(node),
                    show_idx(idx),
                    partition.show_qualified(nodes_prefixmap, schema)?
                )
            },
            ValidatorError::ShapeFailedNoPartitions { node, idx, .. } => {
                format!("No partitions remaining for {}@!{}", show_node(node), show_idx(idx))
            },
            ValidatorError::AbstractShapeError { idx, node, .. } => format!(
                "Shape {} is abstract and cannot be used in validation for node {}",
                show_idx(idx),
                show_node(node)
            ),
            ValidatorError::DescendantShapeError {
                current, desc, node, ..
            } => format!(
                "Error in descendant {} of shape {} for node {}",
                show_idx(desc),
                show_idx(current),
                show_node(node)
            ),
            ValidatorError::DescendantsShapeError { idx, node, .. } => format!(
                "All descendants of shape {} failed for node {}",
                show_idx(idx),
                show_node(node)
            ),
            ValidatorError::ShapeAndError { shape_expr, node, .. } => format!(
                "And error: shape expression {} failed for node {}",
                show_idx(shape_expr),
                show_node(node)
            ),
            ValidatorError::ShapeOrError { node, .. } => {
                format!("OR error: all branches failed for node {}", show_node(node))
            },
            ValidatorError::ShapeNotError { node, .. } => {
                format!("Not error: failed for node {}", show_node(node))
            },
            ValidatorError::ShapeRefFailed { node, idx } => format!(
                "ShapeRef fails for node {} with shape {}",
                show_node(node),
                show_idx(idx)
            ),
            ValidatorError::FailedPending { failed_pending } => {
                let items: Vec<String> = failed_pending
                    .iter()
                    .map(|(n, s)| format!("({}@{})", show_node(n), show_idx(s)))
                    .collect();
                format!("References failed: {}", items.join(", "))
            },
            _ => format!("{self}"),
        };
        Ok(s)
    }

    fn build_tree(
        &self,
        tree: &mut Tree<String>,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
    ) -> Result<(), PrefixMapError> {
        match self {
            ValidatorError::PartitionComponentFailed { errors, .. }
            | ValidatorError::PartitionFailed { errors, .. }
            | ValidatorError::AbstractShapeError { errors, .. }
            | ValidatorError::DescendantShapeError { errors, .. }
            | ValidatorError::DescendantsShapeError { errors, .. }
            | ValidatorError::ShapeAndError { errors, .. } => add_errors_to_tree(tree, errors, nodes_prefixmap, schema),
            ValidatorError::ShapeOrError { errors, .. } => {
                for (idx, errs) in errors {
                    let label_str = Self::show_idx(idx, schema);
                    let mut branch_tree = Tree::new(format!("Branch {label_str}:"));
                    add_errors_to_tree(&mut branch_tree, errs, nodes_prefixmap, schema)?;
                    tree.leaves.push(branch_tree);
                }
                Ok(())
            },
            ValidatorError::ShapeNotError { reasons, .. } => {
                for reason in reasons.iter() {
                    let reason_str = reason.show_qualified(nodes_prefixmap, schema, 80)?;
                    tree.leaves.push(Tree::new(reason_str));
                }
                Ok(())
            },
            ValidatorError::ShapeFailed { errors, .. } => {
                for err in errors {
                    let child_root = err.root_qualified(nodes_prefixmap, schema)?;
                    let mut child_tree = Tree::new(child_root);
                    err.build_tree(&mut child_tree, nodes_prefixmap, schema)?;
                    tree.leaves.push(child_tree);
                }
                Ok(())
            },
            _ => Ok(()),
        }
    }

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        _width: usize,
    ) -> Result<String, PrefixMapError> {
        let root_str = self.root_qualified(nodes_prefixmap, schema)?;
        let mut tree = Tree::new(root_str);
        self.build_tree(&mut tree, nodes_prefixmap, schema)?;
        Ok(format!("{tree}"))
    }
}

impl Serialize for ValidatorError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("error", &self.to_string())?;
        map.end()
    }
}
