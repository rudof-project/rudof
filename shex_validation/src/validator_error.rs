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
            ValidatorError::ClosedShapeWithRemainderPreds { .. } => {
                "Closed shape but found extra properties".to_string()
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
            ValidatorError::ExternalShapeRejected {
                node,
                idx,
                resolver,
                rationale,
            } => format!(
                "External shape {} rejected for node: {} and resolver {}: {}",
                show_label(idx, schema, width),
                show_node(node),
                resolver,
                rationale
            ),
            ValidatorError::ExternalShapeUnresolved { node, idx } => format!(
                "External shape {} for node {} could not be resolved by any registered resolver",
                show_label(idx, schema, width),
                show_node(node),
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
            ValidatorError::ClosedShapeWithRemainderPreds { declared, remainder } => {
                let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
                let declared_str = declared.iter().map(show_pred).collect::<Vec<_>>().join(", ");
                let remainder_str = remainder.iter().map(show_pred).collect::<Vec<_>>().join(", ");
                tree.leaves
                    .push(Tree::new(format!("Allowed properties: {declared_str}")));
                tree.leaves
                    .push(Tree::new(format!("Extra properties found: {remainder_str}")));
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
            | ValidatorError::TripleExprRefuted { .. }
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
            | ValidatorError::RbeError(..)
            | ValidatorError::PrefixMapError(..)
            | ValidatorError::ShapeLabelNotFoundError { .. }
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
        // A shape made of a single triple constraint rejected on its one
        // and only candidate reduces to a single
        // `ConditionFailed` reason. Render that flat, naming the node and
        // property involved, instead of the generic
        // "Shape ... failed ...: no candidates matched" + one-line tree.
        if let ValidatorError::NoMatchesFound { node, reasons, .. } = self
            && let [NoMatchReason::ConditionFailed { predicate, error, .. }] = reasons.as_slice()
        {
            // Datatype IRIs in the error (e.g. xsd:int) come from the schema, not
            // necessarily from the validated data, so they may be unqualified in
            // `nodes_prefixmap` alone (e.g. a data file that never mentions xsd:).
            // Fall back to the schema's prefix map, keeping the data's own
            // prefixes (for the node/value themselves) as the priority.
            let mut combined = schema.prefixmap();
            combined.merge(nodes_prefixmap.clone());
            return Ok(describe_condition_error(node, Some(predicate), error, &combined));
        }
        let root_str = self.root_qualified(nodes_prefixmap, schema, width)?;
        let mut tree = Tree::new(root_str);
        self.build_tree(&mut tree, nodes_prefixmap, schema, width)?;
        Ok(format!("{tree}"))
    }
}

/// Renders a condition failure (e.g. a datatype mismatch from `CondKind::Datatype`)
/// naming the node and, when known, the property that carried the offending
/// value — e.g. "Datatype error on node :x for property :age: expected xsd:int,
/// found xsd:integer, lexical form "30"^^xsd:integer".
fn describe_condition_error(
    node: &Node,
    predicate: Option<&Pred>,
    error: &RbeError<Pred, Node, ShapeLabelIdx, SemanticActionContext, CondKind>,
    nodes_prefixmap: &PrefixMap,
) -> String {
    let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
    let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
    let location = match predicate {
        Some(p) => format!("on node {} for property {}", show_node(node), show_pred(p)),
        None => format!("on node {}", show_node(node)),
    };
    match error {
        RbeError::CondFailed { prefix, details } => {
            let details_str = details
                .iter()
                .map(|(label, v)| format!("{label} {}", show_node(v)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{prefix} {location}: {details_str}")
        },
        other => format!(
            "Condition failed {location}: {}",
            other.show_qualified(&show_pred, &show_node)
        ),
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
