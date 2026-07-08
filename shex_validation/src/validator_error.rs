use crate::PartitionsDisplay;
use crate::Reasons;
use crate::ValidatorErrors;
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;
use rbe::Cardinality;
use rbe::RbeError;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use serde::Serialize;
use serde::ser::SerializeMap;
use shex_ast::ir::node_constraint::NodeConstraint;
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_action_context::SemanticActionContext;
use shex_ast::ir::shape::Shape;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::{Node, Pred, ShapeExprLabel, ShapeLabelIdx, ast::cond_kind::CondKind, ir::shape_label::ShapeLabel};
use termtree::Tree;
use thiserror::Error;

/// Why a single candidate assignment of neighbors to a triple expression was
/// rejected, attached as children of a [`ValidatorError::NoMatchesFound`]
/// error so the user can see, per candidate, why it didn't work.
#[derive(Debug, Clone)]
pub enum NoMatchReason {
    /// `value` didn't satisfy `predicate`'s own condition (e.g. a node
    /// constraint or shape reference).
    ConditionFailed {
        candidate: Vec<(Pred, Node)>,
        predicate: Pred,
        value: Node,
        error: RbeError<Pred, Node, ShapeLabelIdx, SemanticActionContext, CondKind>,
    },
    /// `predicate` needed to occur `expected` times but occurred `current`
    /// times among the candidate's neighbors.
    CardinalityFailed {
        candidate: Vec<(Pred, Node)>,
        predicate: Pred,
        expected: Cardinality,
        current: usize,
    },
    /// A candidate was rejected for a reason that couldn't be attributed to
    /// a single predicate's cardinality (e.g. `Or`-branch interactions).
    Other {
        candidate: Vec<(Pred, Node)>,
        detail: String,
    },
}

impl NoMatchReason {
    fn show_qualified(&self, nodes_prefixmap: &PrefixMap) -> String {
        let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
        let show_candidate = |candidate: &[(Pred, Node)]| {
            candidate
                .iter()
                .map(|(p, v)| format!("{} {}", show_pred(p), v.show_qualified(nodes_prefixmap)))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            NoMatchReason::ConditionFailed {
                candidate,
                predicate,
                value,
                error,
            } => format!(
                "Candidate [{}] rejected: {} {}: {error}",
                show_candidate(candidate),
                show_pred(predicate),
                value.show_qualified(nodes_prefixmap),
            ),
            NoMatchReason::CardinalityFailed {
                candidate,
                predicate,
                expected,
                current,
            } => format!(
                "Candidate [{}] rejected: predicate {} required cardinality {expected:?} but got {current}",
                show_candidate(candidate),
                show_pred(predicate),
            ),
            NoMatchReason::Other { candidate, detail } => {
                format!("Candidate [{}] rejected: {detail}", show_candidate(candidate))
            },
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ValidatorError {
    #[error("Parent shape {idx} failed for node {node} with errors: {errors}")]
    ParentShapeFailed {
        node: Box<Node>,
        idx: ShapeLabelIdx,
        errors: ValidatorErrors,
    },
    #[error(
        "Shape {idx} failed for node {node}. Shape doesn't have a main shape which indicates this is a non-extendable shape, but it is being extended by another shape."
    )]
    ShapeExtendsNoMainShape { idx: ShapeLabelIdx, node: Box<Node> },

    #[error("Parent shape node constraint failed for node {node}@{idx}: Node constraint: {nc}")]
    ParentShapeNodeConstraintFailed {
        node: Box<Node>,
        idx: ShapeLabelIdx,
        nc: Box<NodeConstraint>,
        error: String,
    },

    #[error("Main shape failed for node {node}@{idx}.\nShape: {shape}\nErrors:\n{errors}")]
    ParentShapeMainShapeFailed {
        node: Box<Node>,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        errors: ValidatorErrors,
    },

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

    #[error("Partition failed {node}@{idx}.\nErrors:\n{errors}")]
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

    #[error("References failed: Shape pattern matches, but references failed: {}", failed_pending.iter().map(|(n, s, ks, _errs)| format!("({n}, {s}, preds: [{:?}])", ks.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "))).collect::<Vec<_>>().join(", "))]
    FailedPending {
        failed_pending: Vec<(Node, ShapeLabelIdx, Vec<Pred>, Vec<ValidatorError>)>,
    },
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
    RbeError(#[from] RbeError<Pred, Node, ShapeLabelIdx, SemanticActionContext, CondKind>),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),

    #[error("Shape label {shape_label} not found: {error}")]
    ShapeLabelNotFoundError { shape_label: ShapeExprLabel, error: String },

    #[error("Shape {idx} failed parent {extends} for node {node} with errors: {errors}")]
    ShapeExtendsError {
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
        node: Box<Node>,
        extends: ShapeLabelIdx,
        errors: ValidatorErrors,
    },

    #[error("And error: {shape_expr} failed for node {node}: {errors}")]
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
        // Why each candidate assignment of neighbors to the triple expression
        // was rejected, shown (qualified against the prefix map) as children
        // of this error in the tree-formatted output.
        reasons: Vec<NoMatchReason>,
    },

    #[error("ShapeRef fails for node {node} with idx: {idx}, errors: {errors}")]
    ShapeRefFailed {
        node: Box<Node>,
        idx: ShapeLabelIdx,
        errors: ValidatorErrors,
    },

    #[error("StartAct failed for node {node} with idx: {idx}")]
    StartActFailed { node: Box<Node>, idx: ShapeLabelIdx },

    #[error("EXTERNAL shape {idx} rejected for node {node} by resolver '{resolver}': {rationale}")]
    ExternalShapeRejected {
        node: Box<Node>,
        idx: ShapeLabelIdx,
        resolver: String,
        rationale: String,
    },

    #[error("EXTERNAL shape {idx} for node {node} could not be resolved by any registered resolver")]
    ExternalShapeUnresolved { node: Box<Node>, idx: ShapeLabelIdx },
}

fn add_errors_to_tree(
    tree: &mut Tree<String>,
    errors: &ValidatorErrors,
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
    width: usize,
) -> Result<(), PrefixMapError> {
    for err in errors.iter() {
        let child_root = err.root_qualified(nodes_prefixmap, schema, width)?;
        let mut child_tree = Tree::new(child_root);
        err.build_tree(&mut child_tree, nodes_prefixmap, schema, width)?;
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

    // This method generates a string representation of the error, showing the root error message
    // The root message is the main error message, and the tree structure is built from the nested errors in `build_tree`
    fn root_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
        let show_idx = |idx: &ShapeLabelIdx| Self::show_idx(idx, schema);

        let s = match self {
            ValidatorError::ShapeFailed { node, idx, .. } => {
                format!("Shape {} failed for node {}", show_idx(idx), show_node(node))
            },
            ValidatorError::NoMatchesFound { node, idx, .. } => format!(
                "Shape {} failed for node {}: no candidates matched the expression",
                show_label(idx, schema, width),
                show_node(node)
            ),
            ValidatorError::PartitionComponentFailed { node, idx, .. } => {
                format!(
                    "Partition component failed ({}@{})",
                    show_node(node),
                    show_label(idx, schema, width),
                    // partition.show_qualified(nodes_prefixmap, schema, width)?
                )
            },
            ValidatorError::PartitionFailed {
                node, idx, partition, ..
            } => {
                format!(
                    "Partition failed {}@{}:\nPartition:\n{}",
                    show_node(node),
                    show_idx(idx),
                    partition.show_qualified(nodes_prefixmap, schema, width)?
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
                show_label(desc, schema, width),
                show_label(current, schema, width),
                show_node(node)
            ),
            ValidatorError::DescendantsShapeError { idx, node, .. } => format!(
                "All descendants of shape {} failed for node {}",
                show_label(idx, schema, width),
                show_node(node)
            ),
            ValidatorError::ShapeAndError { shape_expr, node, .. } => format!(
                "And error: {} failed for node {}",
                show_label(shape_expr, schema, width),
                show_node(node)
            ),
            ValidatorError::ShapeOrError { node, .. } => {
                format!("OR error: all branches failed for node {}", show_node(node))
            },
            ValidatorError::ShapeNotError { node, shape_expr, .. } => {
                format!(
                    "Not {}: failed for node {}",
                    show_shape_expr(shape_expr, schema, width),
                    show_node(node)
                )
            },
            ValidatorError::ShapeRefFailed { node, idx, .. } => {
                format!(
                    "Reference to {} fails for node {}",
                    show_label(idx, schema, width),
                    show_node(node)
                )
            },
            ValidatorError::FailedPending { .. } => "References failed:".to_string(),
            ValidatorError::RbeError(err) => {
                let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
                let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
                err.show_qualified(&show_pred, &show_node)
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
        width: usize,
    ) -> Result<(), PrefixMapError> {
        match self {
            ValidatorError::PartitionComponentFailed { errors, .. }
            | ValidatorError::PartitionFailed { errors, .. }
            | ValidatorError::ShapeRefFailed { errors, .. }
            | ValidatorError::AbstractShapeError { errors, .. }
            | ValidatorError::DescendantShapeError { errors, .. }
            | ValidatorError::DescendantsShapeError { errors, .. }
            | ValidatorError::ShapeAndError { errors, .. }
            | ValidatorError::ParentShapeMainShapeFailed { errors, .. }
            | ValidatorError::ParentShapeFailed { errors, .. }
            | ValidatorError::ShapeExtendsError { errors, .. } => {
                add_errors_to_tree(tree, errors, nodes_prefixmap, schema, width)
            },
            ValidatorError::ShapeOrError { errors, .. } => {
                for (idx, errs) in errors {
                    let label_str = Self::show_idx(idx, schema);
                    let mut branch_tree = Tree::new(format!("Branch {label_str}:"));
                    add_errors_to_tree(&mut branch_tree, errs, nodes_prefixmap, schema, width)?;
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
                    let child_root = err.root_qualified(nodes_prefixmap, schema, width)?;
                    let mut child_tree = Tree::new(child_root);
                    err.build_tree(&mut child_tree, nodes_prefixmap, schema, width)?;
                    tree.leaves.push(child_tree);
                }
                Ok(())
            },
            ValidatorError::NoMatchesFound { reasons, .. } => {
                for reason in reasons {
                    tree.leaves.push(Tree::new(reason.show_qualified(nodes_prefixmap)));
                }
                Ok(())
            },
            ValidatorError::FailedPending { failed_pending } => {
                let show_pred = |p: &IriS| nodes_prefixmap.qualify(p);
                for (n, s, ks, errs) in failed_pending {
                    let keys = match ks.len() {
                        0 => String::new(),
                        1 => format!("Predicate {}", show_pred(ks[0].iri())),
                        _ => format!(
                            "Predicates {}",
                            ks.iter().map(|k| show_pred(k.iri())).collect::<Vec<_>>().join(", ")
                        ),
                    };
                    let ref_root = format!(
                        "{} -> {} as {}",
                        keys,
                        n.show_qualified(nodes_prefixmap),
                        Self::show_idx(s, schema)
                    );
                    let mut ref_tree = Tree::new(ref_root);
                    add_errors_to_tree(
                        &mut ref_tree,
                        &ValidatorErrors::new(errs.clone()),
                        nodes_prefixmap,
                        schema,
                        width,
                    )?;
                    tree.leaves.push(ref_tree);
                }
                Ok(())
            },
            ValidatorError::ShapeExtendsNoMainShape { .. }
            | ValidatorError::ParentShapeNodeConstraintFailed { .. }
            | ValidatorError::ShapeFailedNoPartitions { .. }
            | ValidatorError::FillingShapeMapNodes { .. }
            | ValidatorError::AbstractShapeNoDescendants { .. }
            | ValidatorError::NodeShapeError { .. }
            | ValidatorError::TermToRDFNodeFailed { .. }
            | ValidatorError::ReasonSerializationError { .. }
            | ValidatorError::ErrorSerializationError { .. }
            | ValidatorError::NegCycleError { .. }
            | ValidatorError::SRDFError { .. }
            | ValidatorError::NotFoundShapeLabel { .. }
            | ValidatorError::NotFoundShapeLabelWithIndex { .. }
            | ValidatorError::ConversionObjectIri { .. }
            | ValidatorError::SchemaIRError { .. }
            | ValidatorError::ShapeMapError { .. }
            | ValidatorError::RbeFailed()
            | ValidatorError::ClosedShapeWithRemainderPreds { .. }
            | ValidatorError::RbeError(..)
            | ValidatorError::PrefixMapError(..)
            | ValidatorError::ShapeLabelNotFoundError { .. }
            | ValidatorError::ValidatorConfigFromPathError { .. }
            | ValidatorError::ValidatorConfigTomlError { .. }
            | ValidatorError::AddingNonConformantError { .. }
            | ValidatorError::AddingConformantError { .. }
            | ValidatorError::AddingPendingError { .. }
            | ValidatorError::ShapeExprNotFound { .. }
            | ValidatorError::ExternalShapeRejected { .. }
            | ValidatorError::ExternalShapeUnresolved { .. }
            | ValidatorError::StartActFailed { .. } => Ok(()),
        }
    }

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        let root_str = self.root_qualified(nodes_prefixmap, schema, width)?;
        let mut tree = Tree::new(root_str);
        self.build_tree(&mut tree, nodes_prefixmap, schema, width)?;
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

fn show_label(idx: &ShapeLabelIdx, schema: &SchemaIR, width: usize) -> String {
    if let Some(label) = schema.shape_label_from_idx(idx) {
        schema.show_label(label)
    } else {
        if let Some(info) = schema.find_shape_idx(idx) {
            show_shape_expr(info.expr(), schema, width)
        } else {
            format!("Shape {idx}")
        }
    }
}

fn show_shape_expr(shape_expr: &ShapeExpr, schema: &SchemaIR, width: usize) -> String {
    schema.show_shape_expr(shape_expr, width)
}
