use crate::PartitionsDisplay;
use crate::Reasons;
use crate::ValidatorErrors;
use crate::no_match_reason::NoMatchReason;
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;
use rbe::RbeError;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use serde::Serialize;
use serde::ser::SerializeMap;
use shape_path::SchemaPath;
use shex_ast::ir::node_constraint::NodeConstraint;
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_action_context::SemanticActionContext;
use shex_ast::ir::shape::Shape;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::{Node, Pred, ShapeExprLabel, ShapeLabelIdx, ast::cond_kind::CondKind, ir::shape_label::ShapeLabel};
use termtree::Tree;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ValidatorError {
    #[error("Failed to record a triple in a validator sink: {error}")]
    ReadSinkFailed { error: String },

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

    #[error(
        "Shape {idx} refuted for node {node}: no assignment of the neighbourhood can satisfy the triple expressions (feasibility check)"
    )]
    TripleExprRefuted { node: Box<Node>, idx: ShapeLabelIdx },

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

    #[error("References failed: Shape pattern matches, but references failed: {}", failed_pending.iter().map(|(n, s, ks, _errs)| format!("({n}, {s}, preds: [{}])", ks.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "))).collect::<Vec<_>>().join(", "))]
    FailedPending {
        failed_pending: Vec<(Node, ShapeLabelIdx, Vec<Pred>, Vec<ValidatorError>)>,
    },
    #[error("Negation cycle error: {}", show_neg_cycles(neg_cycles))]
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
        "Closed shape but found properties {remainder} which are not part of shape declared properties: {declared}"
    )]
    ClosedShapeWithRemainderPreds {
        idx: ShapeLabelIdx,
        remainder: Preds,
        declared: Preds,
    },

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

    #[error(
        "Validation aborted: exceeded the configured limit of {max_steps} step(s) while proving {node}@{idx}. This usually means the shape (or the data it is validated against, e.g. a highly-connected node on a remote SPARQL endpoint) requires far more work than expected; increase the limit if this is legitimate."
    )]
    MaxStepsExceeded {
        node: Box<Node>,
        idx: ShapeLabelIdx,
        max_steps: usize,
    },

    #[error("Validation cancelled while proving {node}@{idx}")]
    Cancelled { node: Box<Node>, idx: ShapeLabelIdx },
}

/// Adds `errors` under `tree`, each as a line with the lines that explain it below.
///
/// When the line of `tree` already says that `node` does not conform to a shape, an error that
/// only says that again (that `node` does not conform to some shape, or that references failed)
/// adds no line of its own: its explanation goes right under `tree`. This keeps chains like
/// "reference failed" / "shape failed" / "no match" down to the line that says why.
fn add_errors_to_tree<'a>(
    tree: &mut Tree<String>,
    errors: impl IntoIterator<Item = &'a ValidatorError>,
    node: Option<&Node>,
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
    width: usize,
) -> Result<(), PrefixMapError> {
    for err in errors {
        let repeats_the_line = match err {
            ValidatorError::FailedPending { .. } => node.is_some(),
            _ => node.is_some() && err.non_conforming_node() == node,
        };
        if repeats_the_line {
            err.build_tree(tree, nodes_prefixmap, schema, width)?;
        } else {
            let mut child_tree = Tree::new(err.root_qualified(nodes_prefixmap, schema, width)?);
            err.build_tree(&mut child_tree, nodes_prefixmap, schema, width)?;
            tree.leaves.push(child_tree);
        }
    }
    Ok(())
}

/// Adds a line under `tree` for each reason why the neighbours of a node did not match a triple
/// expression, without repeating a line. When the reasons were found with different candidate
/// assignments of the neighbours, they are grouped under a line for each candidate.
fn add_no_match_reasons_to_tree(
    tree: &mut Tree<String>,
    reasons: &[NoMatchReason],
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
) {
    // (candidate, lines found with it), in the order the candidates were tried
    let mut groups: Vec<(&[(Pred, Node)], Vec<String>)> = Vec::new();
    for reason in reasons {
        let line = reason.show_qualified(nodes_prefixmap, schema);
        let candidate = reason.candidate();
        match groups.iter_mut().find(|(c, _)| *c == candidate) {
            Some((_, lines)) => {
                if !lines.contains(&line) {
                    lines.push(line)
                }
            },
            None => groups.push((candidate, vec![line])),
        }
    }
    let grouped = groups.iter().filter(|(candidate, _)| !candidate.is_empty()).count() > 1;
    for (candidate, lines) in groups {
        if grouped && !candidate.is_empty() {
            let shown = candidate
                .iter()
                .map(|(p, v)| {
                    format!(
                        "{} {}",
                        nodes_prefixmap.qualify(p.iri()),
                        v.show_qualified(nodes_prefixmap)
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            let mut candidate_tree = Tree::new(format!("with {shown}"));
            candidate_tree.leaves.extend(lines.into_iter().map(Tree::new));
            tree.leaves.push(candidate_tree);
        } else {
            tree.leaves.extend(lines.into_iter().map(Tree::new));
        }
    }
}

impl ValidatorError {
    /// This error with the explanations of the references it mentions left out:
    /// a failed reference keeps its node and shape, not why that node failed.
    ///
    /// A failing reference embeds the referenced node's errors, so explanations
    /// nest along the references. On cyclic data with several references per
    /// node that tree has exponentially many paths, so an embedded explanation
    /// keeps one level: the referenced node's own errors. The rest is in each
    /// node's own result.
    pub(crate) fn without_nested_references(&self) -> ValidatorError {
        let mut error = self.clone();
        match &mut error {
            ValidatorError::ShapeRefFailed { errors, .. } => *errors = ValidatorErrors::new(Vec::new()),
            ValidatorError::FailedPending { failed_pending } => {
                for (_, _, _, errors) in failed_pending {
                    errors.clear();
                }
            },
            ValidatorError::PartitionComponentFailed { errors, .. }
            | ValidatorError::PartitionFailed { errors, .. }
            | ValidatorError::AbstractShapeError { errors, .. }
            | ValidatorError::DescendantShapeError { errors, .. }
            | ValidatorError::DescendantsShapeError { errors, .. }
            | ValidatorError::ShapeAndError { errors, .. }
            | ValidatorError::ParentShapeMainShapeFailed { errors, .. }
            | ValidatorError::ParentShapeFailed { errors, .. }
            | ValidatorError::ShapeExtendsError { errors, .. } => *errors = errors.without_nested_references(),
            ValidatorError::ShapeOrError { errors, .. } => {
                for (_, branch) in errors {
                    *branch = branch.without_nested_references();
                }
            },
            ValidatorError::ShapeFailed { errors, .. } => {
                *errors = errors.iter().map(|e| e.without_nested_references()).collect();
            },
            _ => {},
        }
        error
    }

    /// The ShExPaths of the shape expressions and triple constraints of the schema this error
    /// names, its own shape first and then those of the errors and reasons it contains, without
    /// repeats.
    pub fn paths(&self, schema: &SchemaIR) -> Vec<SchemaPath> {
        let mut paths = Vec::new();
        self.collect_paths(schema, &mut paths, &mut Vec::new());
        paths
    }

    /// The ShExPath that says best where this error is: the triple constraint that failed when
    /// there is one, and otherwise the shape the error is about.
    pub fn path(&self, schema: &SchemaIR) -> Option<SchemaPath> {
        let mut constraints = Vec::new();
        self.collect_paths(schema, &mut Vec::new(), &mut constraints);
        match constraints.as_slice() {
            [constraint] => Some(constraint.clone()),
            _ => self.shape_idx().and_then(|idx| schema.path_of(idx)),
        }
    }

    /// The node this error says does not conform to a shape, when that and why is all it says.
    fn non_conforming_node(&self) -> Option<&Node> {
        match self {
            ValidatorError::ShapeFailed { node, .. }
            | ValidatorError::NoMatchesFound { node, .. }
            | ValidatorError::ShapeRefFailed { node, .. } => Some(node),
            _ => None,
        }
    }

    /// The shape expression this error is about, if it is about one.
    fn shape_idx(&self) -> Option<&ShapeLabelIdx> {
        match self {
            ValidatorError::ParentShapeFailed { idx, .. }
            | ValidatorError::ShapeExtendsNoMainShape { idx, .. }
            | ValidatorError::ParentShapeNodeConstraintFailed { idx, .. }
            | ValidatorError::ParentShapeMainShapeFailed { idx, .. }
            | ValidatorError::PartitionComponentFailed { idx, .. }
            | ValidatorError::PartitionFailed { idx, .. }
            | ValidatorError::TripleExprRefuted { idx, .. }
            | ValidatorError::ShapeFailedNoPartitions { idx, .. }
            | ValidatorError::AbstractShapeError { idx, .. }
            | ValidatorError::DescendantsShapeError { idx, .. }
            | ValidatorError::AbstractShapeNoDescendants { idx }
            | ValidatorError::ClosedShapeWithRemainderPreds { idx, .. }
            | ValidatorError::ShapeExtendsError { idx, .. }
            | ValidatorError::ShapeAndError { shape_expr: idx, .. }
            | ValidatorError::ShapeFailed { idx, .. }
            | ValidatorError::NoMatchesFound { idx, .. }
            | ValidatorError::ShapeRefFailed { idx, .. }
            | ValidatorError::StartActFailed { idx, .. }
            | ValidatorError::ExternalShapeRejected { idx, .. }
            | ValidatorError::ExternalShapeUnresolved { idx, .. }
            | ValidatorError::MaxStepsExceeded { idx, .. }
            | ValidatorError::Cancelled { idx, .. } => Some(idx),
            ValidatorError::DescendantShapeError { desc, .. } => Some(desc),
            _ => None,
        }
    }

    /// Adds to `paths` the paths this error names and to `constraints` those of the triple
    /// constraints among them.
    fn collect_paths(&self, schema: &SchemaIR, paths: &mut Vec<SchemaPath>, constraints: &mut Vec<SchemaPath>) {
        fn add(path: Option<SchemaPath>, to: &mut Vec<SchemaPath>) {
            if let Some(path) = path
                && !to.contains(&path)
            {
                to.push(path);
            }
        }
        add(self.shape_idx().and_then(|idx| schema.path_of(idx)), paths);
        match self {
            ValidatorError::NoMatchesFound { reasons, .. } => {
                for reason in reasons {
                    add(reason.path(schema), paths);
                    add(reason.path(schema), constraints);
                }
            },
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
                for error in errors.iter() {
                    error.collect_paths(schema, paths, constraints);
                }
            },
            ValidatorError::ShapeFailed { errors, .. } => {
                for error in errors {
                    error.collect_paths(schema, paths, constraints);
                }
            },
            ValidatorError::ShapeOrError { errors, .. } => {
                for (idx, branch) in errors {
                    add(schema.path_of(idx), paths);
                    for error in branch.iter() {
                        error.collect_paths(schema, paths, constraints);
                    }
                }
            },
            ValidatorError::FailedPending { failed_pending } => {
                for (_, idx, _, errors) in failed_pending {
                    add(schema.path_of(idx), paths);
                    for error in errors {
                        error.collect_paths(schema, paths, constraints);
                    }
                }
            },
            _ => {},
        }
    }

    /// How the shape expression at `idx` is named in a message, see [`show_label`].
    fn show_idx(idx: &ShapeLabelIdx, schema: &SchemaIR) -> String {
        show_label(idx, schema, 80)
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
            ValidatorError::ShapeFailed { node, idx, .. }
            | ValidatorError::NoMatchesFound { node, idx, .. }
            | ValidatorError::ShapeRefFailed { node, idx, .. } => {
                format!(
                    "{} does not conform to {}",
                    show_node(node),
                    show_label(idx, schema, width)
                )
            },
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
            ValidatorError::TripleExprRefuted { node, idx } => format!(
                "Shape {} refuted for node {}: no assignment of the neighbourhood can satisfy the triple expressions",
                show_idx(idx),
                show_node(node)
            ),
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
                "{} does not conform to {}, which an AND requires",
                show_node(node),
                show_label(shape_expr, schema, width)
            ),
            ValidatorError::ShapeOrError { node, .. } => {
                format!("{} does not conform to any branch of the OR", show_node(node))
            },
            ValidatorError::ShapeNotError { node, shape_expr, .. } => {
                format!(
                    "{} conforms to {}, which a NOT forbids",
                    show_node(node),
                    show_shape_expr(shape_expr, schema, width)
                )
            },
            ValidatorError::FailedPending { .. } => "References failed".to_string(),
            ValidatorError::RbeError(err) => {
                let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
                let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
                err.show_qualified(&show_pred, &show_node)
            },
            ValidatorError::ClosedShapeWithRemainderPreds {
                idx,
                remainder,
                declared,
            } => {
                let show_preds = |preds: &Preds| {
                    preds
                        .iter()
                        .map(|p| nodes_prefixmap.qualify(p.iri()))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                format!(
                    "Closed shape {} does not allow {} (allowed: {})",
                    show_idx(idx),
                    show_preds(remainder),
                    show_preds(declared)
                )
            },
            ValidatorError::ParentShapeFailed { .. } => "Parent shape failed".to_string(),
            ValidatorError::ShapeExtendsNoMainShape { .. } => {
                "Shape extends another shape but the parent shape has no main shape".to_string()
            },
            ValidatorError::ParentShapeNodeConstraintFailed { .. } => "Parent shape node constraint failed".to_string(),
            ValidatorError::ParentShapeMainShapeFailed { .. } => "Parent shape main shape failed".to_string(),
            ValidatorError::FillingShapeMapNodes { .. } => {
                "Filling node from node selector to validate failed".to_string()
            },
            ValidatorError::AbstractShapeNoDescendants { .. } => "Abstract shape has no descendants".to_string(),
            ValidatorError::NodeShapeError { .. } => "Creating shapemap from node and shape failed".to_string(),
            ValidatorError::TermToRDFNodeFailed { .. } => "Converting Term to RDFNode failed".to_string(),
            ValidatorError::ReasonSerializationError { .. } => "Serialization of reason failed".to_string(),
            ValidatorError::ErrorSerializationError { .. } => "Serialization of error failed".to_string(),
            ValidatorError::NegCycleError { .. } => "Negation cycle error".to_string(),
            ValidatorError::SRDFError { .. } => "SRDF error".to_string(),
            ValidatorError::ReadSinkFailed { .. } => "Recording a triple in a sink failed".to_string(),
            ValidatorError::NotFoundShapeLabel { .. } => "Shape label not found".to_string(),
            ValidatorError::NotFoundShapeLabelWithIndex { .. } => "Shape label not found with index".to_string(),
            ValidatorError::ConversionObjectIri { .. } => "Conversion of object IRI failed".to_string(),
            ValidatorError::SchemaIRError { .. } => "Schema IRI error".to_string(),
            ValidatorError::ShapeMapError { .. } => "Shape map error".to_string(),
            ValidatorError::RbeFailed() => "Regular expression failed".to_string(),
            ValidatorError::PrefixMapError(err) => format!("Prefix map error: {}", err),
            ValidatorError::ShapeLabelNotFoundError { .. } => "Shape label not found".to_string(),
            ValidatorError::ShapeExtendsError { .. } => "Shape extends error".to_string(),
            ValidatorError::AddingNonConformantError { node, label, error } => {
                format!(
                    "Adding non-conformant for node: {} and label: {}, error: {}",
                    node, label, error
                )
            },
            ValidatorError::AddingConformantError { node, label, error } => format!(
                "Adding conformant for node: {} and label: {}, error: {}",
                node, label, error
            ),
            ValidatorError::AddingPendingError { node, label, error } => format!(
                "Adding pending for node: {} and label: {}, error: {}",
                node, label, error
            ),
            ValidatorError::ShapeExprNotFound { idx } => {
                format!("Shape expression {} not found", show_label(idx, schema, width))
            },
            ValidatorError::StartActFailed { node, idx } => format!(
                "Start action failed for node: {} and shape: {}",
                show_node(node),
                show_label(idx, schema, width)
            ),
            // The node is said by the line above or the row of the result.
            ValidatorError::ExternalShapeRejected {
                idx,
                resolver,
                rationale,
                ..
            } => format!(
                "resolver {resolver} rejected EXTERNAL shape {}: {rationale}",
                show_label(idx, schema, width),
            ),
            ValidatorError::ExternalShapeUnresolved { idx, .. } => format!(
                "no registered resolver resolved EXTERNAL shape {}",
                show_label(idx, schema, width),
            ),
            ValidatorError::MaxStepsExceeded { node, idx, max_steps } => format!(
                "Validation aborted: exceeded the configured limit of {} step(s) while proving {}@{}",
                max_steps,
                show_node(node),
                show_idx(idx)
            ),
            ValidatorError::Cancelled { node, idx } => format!(
                "Validation cancelled while proving {}@{}",
                show_node(node),
                show_idx(idx)
            ),
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
            | ValidatorError::AbstractShapeError { errors, .. }
            | ValidatorError::DescendantShapeError { errors, .. }
            | ValidatorError::DescendantsShapeError { errors, .. }
            | ValidatorError::ParentShapeMainShapeFailed { errors, .. }
            | ValidatorError::ParentShapeFailed { errors, .. }
            | ValidatorError::ShapeExtendsError { errors, .. } => {
                add_errors_to_tree(tree, errors.iter(), None, nodes_prefixmap, schema, width)
            },
            // The line of these errors already says that the node does not conform to the shape.
            ValidatorError::ShapeRefFailed { node, errors, .. }
            | ValidatorError::ShapeAndError { node, errors, .. } => {
                add_errors_to_tree(tree, errors.iter(), Some(node), nodes_prefixmap, schema, width)
            },
            ValidatorError::ShapeOrError { node, errors, .. } => {
                for (index, (idx, errs)) in errors.iter().enumerate() {
                    let mut branch_tree = Tree::new(format!("branch {index} ({})", Self::show_idx(idx, schema)));
                    add_errors_to_tree(
                        &mut branch_tree,
                        errs.iter(),
                        Some(node),
                        nodes_prefixmap,
                        schema,
                        width,
                    )?;
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
            ValidatorError::ShapeFailed { node, errors, .. } => {
                add_errors_to_tree(tree, errors.iter(), Some(node), nodes_prefixmap, schema, width)
            },
            ValidatorError::NoMatchesFound { reasons, .. } => {
                add_no_match_reasons_to_tree(tree, reasons, nodes_prefixmap, schema);
                Ok(())
            },
            // Its line says everything.
            ValidatorError::ClosedShapeWithRemainderPreds { .. } => Ok(()),
            ValidatorError::FailedPending { failed_pending } => {
                let show_pred = |p: &IriS| nodes_prefixmap.qualify(p);
                for (n, s, ks, errs) in failed_pending {
                    let properties = match ks.len() {
                        0 => String::new(),
                        1 => format!("property {} ", show_pred(ks[0].iri())),
                        _ => format!(
                            "properties {} ",
                            ks.iter().map(|k| show_pred(k.iri())).collect::<Vec<_>>().join(", ")
                        ),
                    };
                    let ref_root = format!(
                        "{properties}value {} does not conform to {}",
                        n.show_qualified(nodes_prefixmap),
                        show_label(s, schema, width)
                    );
                    let mut ref_tree = Tree::new(ref_root);
                    add_errors_to_tree(&mut ref_tree, errs.iter(), Some(n), nodes_prefixmap, schema, width)?;
                    tree.leaves.push(ref_tree);
                }
                Ok(())
            },
            ValidatorError::ShapeExtendsNoMainShape { .. }
            | ValidatorError::ParentShapeNodeConstraintFailed { .. }
            | ValidatorError::ShapeFailedNoPartitions { .. }
            | ValidatorError::TripleExprRefuted { .. }
            | ValidatorError::FillingShapeMapNodes { .. }
            | ValidatorError::AbstractShapeNoDescendants { .. }
            | ValidatorError::NodeShapeError { .. }
            | ValidatorError::TermToRDFNodeFailed { .. }
            | ValidatorError::ReasonSerializationError { .. }
            | ValidatorError::ErrorSerializationError { .. }
            | ValidatorError::NegCycleError { .. }
            | ValidatorError::SRDFError { .. }
            | ValidatorError::ReadSinkFailed { .. }
            | ValidatorError::NotFoundShapeLabel { .. }
            | ValidatorError::NotFoundShapeLabelWithIndex { .. }
            | ValidatorError::ConversionObjectIri { .. }
            | ValidatorError::SchemaIRError { .. }
            | ValidatorError::ShapeMapError { .. }
            | ValidatorError::RbeFailed()
            | ValidatorError::RbeError(..)
            | ValidatorError::PrefixMapError(..)
            | ValidatorError::ShapeLabelNotFoundError { .. }
            | ValidatorError::AddingNonConformantError { .. }
            | ValidatorError::AddingConformantError { .. }
            | ValidatorError::AddingPendingError { .. }
            | ValidatorError::ShapeExprNotFound { .. }
            | ValidatorError::ExternalShapeRejected { .. }
            | ValidatorError::ExternalShapeUnresolved { .. }
            | ValidatorError::MaxStepsExceeded { .. }
            | ValidatorError::Cancelled { .. }
            | ValidatorError::StartActFailed { .. } => Ok(()),
        }
    }

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        // Datatype IRIs and predicates in an error (e.g. xsd:int) come from the schema, not
        // necessarily from the validated data, so they may be unqualified in `nodes_prefixmap`
        // alone (e.g. a data file that never mentions xsd:). Fall back to the schema's prefix
        // map, keeping the data's own prefixes (for the nodes and values themselves) as the
        // priority, at every level of the error.
        let mut combined = schema.prefixmap();
        combined.merge(nodes_prefixmap.clone());
        let nodes_prefixmap = &combined;
        let root_str = self.root_qualified(nodes_prefixmap, schema, width)?;
        let mut tree = Tree::new(root_str);
        self.build_tree(&mut tree, nodes_prefixmap, schema, width)?;
        // An error explained by a single line is shown in one line: `:x does not conform to :S:
        // property :p: ...`, instead of a line and a tree with one leaf.
        if let [leaf] = tree.leaves.as_slice()
            && leaf.leaves.is_empty()
        {
            return Ok(format!("{}: {}", tree.root, leaf.root));
        }
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

/// Shows each negative edge of each cycle as `:S -> NOT :T in cycle [:S, :T]`
fn show_neg_cycles(neg_cycles: &[Vec<(String, String, Vec<String>)>]) -> String {
    neg_cycles
        .iter()
        .flat_map(|cycle| {
            cycle
                .iter()
                .map(|(source, target, shapes)| format!("{source} -> NOT {target} in cycle [{}]", shapes.join(", ")))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// How the shape expression at `idx` is named in a message: by its label, or, for an anonymous
/// one, by where it is in the schema, its ShExPath (`@:S~:q/valueExpr`). An anonymous shape
/// expression that a ShExPath can't name, inside one labelled with a blank node, is shown as it
/// is written.
fn show_label(idx: &ShapeLabelIdx, schema: &SchemaIR, width: usize) -> String {
    if let Some(label) = schema.shape_label_from_idx(idx) {
        schema.show_label(label)
    } else if let Some(path) = show_path(idx, schema) {
        path
    } else if let Some(info) = schema.find_shape_idx(idx) {
        show_shape_expr(info.expr(), schema, width)
    } else {
        format!("Shape {idx}")
    }
}

/// The ShExPath of the shape expression at `idx`, with the prefixes of the schema.
fn show_path(idx: &ShapeLabelIdx, schema: &SchemaIR) -> Option<String> {
    schema.path_of(idx).map(|path| path.show_qualified(&schema.prefixmap()))
}

fn show_shape_expr(shape_expr: &ShapeExpr, schema: &SchemaIR, width: usize) -> String {
    schema.show_shape_expr(shape_expr, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg_cycle_error_shows_the_shape_labels() {
        let error = ValidatorError::NegCycleError {
            neg_cycles: vec![vec![(
                ":N".to_string(),
                ":N2".to_string(),
                vec![":N".to_string(), ":N2".to_string()],
            )]],
        };
        assert_eq!(
            error.to_string(),
            "Negation cycle error: :N -> NOT :N2 in cycle [:N, :N2]"
        );
    }

    #[test]
    fn closed_shape_error_shows_the_predicates() {
        let error = ValidatorError::ClosedShapeWithRemainderPreds {
            idx: ShapeLabelIdx::from(0),
            remainder: Preds::new(vec![Pred::new_unchecked("http://example.org/p")]),
            declared: Preds::new(vec![Pred::new_unchecked("http://example.org/q")]),
        };
        assert_eq!(
            error.to_string(),
            "Closed shape but found properties [http://example.org/p] which are not part of shape declared properties: [http://example.org/q]"
        );
    }
}
