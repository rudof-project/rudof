use prefixmap::PrefixMap;
use rbe::{Cardinality, RbeError};
use shex_ast::{Node, Pred, ShapeLabelIdx, cond_kind::CondKind, ir::semantic_action_context::SemanticActionContext};

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
    pub fn show_qualified(&self, nodes_prefixmap: &PrefixMap) -> String {
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
            } => format!(
                "Condition failed for predicate {} on node {}: {}",
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
