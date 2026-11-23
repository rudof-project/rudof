use prefixmap::PrefixMap;
use rbe::{Cardinality, Component, RbeError};
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::shexpath::SchemaPath;
use shex_ast::{Node, Pred, ShapeLabelIdx, cond_kind::CondKind, ir::semantic_action_context::SemanticActionContext};

/// The triple constraint a failure is about: the shape whose triple expression it is in and the
/// component it was compiled as. Its ShExPath is
/// [`SchemaIR::component_path`](shex_ast::ir::schema_ir::SchemaIR::component_path).
pub type ConstraintRef = (ShapeLabelIdx, Component);

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
        /// The triple constraint whose condition failed, when a single one is to blame.
        constraint: Option<ConstraintRef>,
    },
    /// `predicate` needed to occur `expected` times but occurred `current`
    /// times among the candidate's neighbors.
    CardinalityFailed {
        candidate: Vec<(Pred, Node)>,
        predicate: Pred,
        expected: Cardinality,
        current: usize,
        /// The triple constraint whose cardinality was not met.
        constraint: Option<ConstraintRef>,
    },
    /// A candidate was rejected for a reason that couldn't be attributed to
    /// a single predicate's cardinality (e.g. `Or`-branch interactions).
    Other {
        candidate: Vec<(Pred, Node)>,
        detail: String,
    },
}

impl NoMatchReason {
    /// The triple constraint this reason is about, when it is known.
    pub fn constraint(&self) -> Option<&ConstraintRef> {
        match self {
            NoMatchReason::ConditionFailed { constraint, .. } | NoMatchReason::CardinalityFailed { constraint, .. } => {
                constraint.as_ref()
            },
            NoMatchReason::Other { .. } => None,
        }
    }

    /// The ShExPath of the triple constraint this reason is about, when it is known.
    pub fn path(&self, schema: &SchemaIR) -> Option<SchemaPath> {
        let (idx, component) = self.constraint()?;
        schema.component_path(idx, component)
    }

    pub fn show_qualified(&self, nodes_prefixmap: &PrefixMap, schema: &SchemaIR) -> String {
        // ` at @:S~:p`, where in the schema the failing triple constraint is
        let at = self
            .path(schema)
            .map(|path| format!(" at {}", path.show_qualified(&schema.prefixmap())))
            .unwrap_or_default();
        let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
        let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
        let show_candidate = |candidate: &[(Pred, Node)]| {
            candidate
                .iter()
                .map(|(p, v)| format!("{} {}", show_pred(p), v.show_qualified(nodes_prefixmap)))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            NoMatchReason::ConditionFailed {
                candidate: _,
                predicate,
                value,
                error,
                ..
            } => format!(
                "Condition failed for predicate {}{at} on node {}: {}",
                // show_candidate(candidate),
                show_pred(predicate),
                value.show_qualified(nodes_prefixmap),
                error.show_qualified(&show_pred, &show_node),
            ),
            NoMatchReason::CardinalityFailed {
                candidate,
                predicate,
                expected,
                current,
                ..
            } => format!(
                "Candidate [{}] rejected: predicate {}{at} required cardinality {expected:?} but got {current}",
                show_candidate(candidate),
                show_pred(predicate),
            ),
            NoMatchReason::Other { candidate, detail } => {
                format!("Candidate [{}] rejected: {detail}", show_candidate(candidate))
            },
        }
    }
}
