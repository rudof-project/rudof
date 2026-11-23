use prefixmap::PrefixMap;
use rbe::{Cardinality, Component, Max, RbeError};
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::shexpath::{SchemaPath, SchemaStep};
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

    /// The neighbours this reason was found with: the assignment of triples to the triple
    /// expression that was tried.
    pub fn candidate(&self) -> &[(Pred, Node)] {
        match self {
            NoMatchReason::ConditionFailed { candidate, .. }
            | NoMatchReason::CardinalityFailed { candidate, .. }
            | NoMatchReason::Other { candidate, .. } => candidate,
        }
    }

    /// The reason in one line, without the candidate it was found with, ending with where the
    /// triple constraint is in the schema: `property :name: expected exactly 1 value, found 0
    /// (at @:Person~:name)`.
    pub fn show_qualified(&self, nodes_prefixmap: &PrefixMap, schema: &SchemaIR) -> String {
        let at = self.path(schema).map(|path| show_at(&path, schema)).unwrap_or_default();
        let show_pred = |p: &Pred| nodes_prefixmap.qualify(p.iri());
        let show_node = |n: &Node| n.show_qualified(nodes_prefixmap);
        match self {
            NoMatchReason::ConditionFailed {
                predicate,
                value,
                error,
                ..
            } => match error {
                // A datatype error already shows the value, as its lexical form.
                RbeError::CondFailed { .. } => format!(
                    "property {}: {}{at}",
                    show_pred(predicate),
                    error.show_qualified(&show_pred, &show_node)
                ),
                _ => format!(
                    "property {} value {}: {}{at}",
                    show_pred(predicate),
                    show_node(value),
                    error.show_qualified(&show_pred, &show_node)
                ),
            },
            NoMatchReason::CardinalityFailed {
                predicate,
                expected,
                current,
                ..
            } => format!(
                "property {}: expected {}, found {current}{at}",
                show_pred(predicate),
                show_cardinality(expected)
            ),
            NoMatchReason::Other { detail, .. } => detail.clone(),
        }
    }
}

/// How many values a cardinality allows, in words: `exactly 1 value`, `at least 2 values`.
pub(crate) fn show_cardinality(cardinality: &Cardinality) -> String {
    let values = |n: usize| {
        if n == 1 {
            "1 value".to_string()
        } else {
            format!("{n} values")
        }
    };
    let min = cardinality.min.value;
    match &cardinality.max {
        Max::Unbounded if min == 0 => "any number of values".to_string(),
        Max::Unbounded => format!("at least {}", values(min)),
        Max::IntMax(0) => "no values".to_string(),
        Max::IntMax(max) if *max == min => format!("exactly {}", values(min)),
        Max::IntMax(max) if min == 0 => format!("at most {}", values(*max)),
        Max::IntMax(max) => format!("between {min} and {max} values"),
    }
}

/// ` (at PATH)`: where in the schema what a line is about is, written with the prefixes of the
/// schema. For one of several triple constraints on the same predicate, which one it is too:
/// ` (2nd :code constraint, at @:Product~:code/self::[1])`.
pub(crate) fn show_at(path: &SchemaPath, schema: &SchemaIR) -> String {
    let prefixmap = schema.prefixmap();
    let shown = path.show_qualified(&prefixmap);
    match path.steps().last() {
        Some(SchemaStep::TripleConstraint {
            predicate,
            nth: Some(nth),
        }) => format!(
            " ({} {} constraint, at {shown})",
            ordinal(nth + 1),
            prefixmap.qualify(predicate)
        ),
        _ => format!(" (at {shown})"),
    }
}

/// `1st`, `2nd`, `3rd`, `4th`, ..., `11th`, `12th`, ..., `21st`, ...
fn ordinal(n: usize) -> String {
    let suffix = match (n % 10, n % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbe::Min;

    fn card(min: usize, max: Max) -> Cardinality {
        Cardinality::from(Min { value: min }, max)
    }

    #[test]
    fn cardinalities_are_shown_in_words() {
        assert_eq!(show_cardinality(&card(1, Max::IntMax(1))), "exactly 1 value");
        assert_eq!(show_cardinality(&card(2, Max::IntMax(2))), "exactly 2 values");
        assert_eq!(show_cardinality(&card(0, Max::IntMax(1))), "at most 1 value");
        assert_eq!(show_cardinality(&card(1, Max::Unbounded)), "at least 1 value");
        assert_eq!(show_cardinality(&card(0, Max::Unbounded)), "any number of values");
        assert_eq!(show_cardinality(&card(2, Max::IntMax(5))), "between 2 and 5 values");
        assert_eq!(show_cardinality(&card(0, Max::IntMax(0))), "no values");
    }

    #[test]
    fn ordinals() {
        let shown: Vec<String> = [1, 2, 3, 4, 11, 12, 13, 21, 22, 101].into_iter().map(ordinal).collect();
        assert_eq!(
            shown,
            [
                "1st", "2nd", "3rd", "4th", "11th", "12th", "13th", "21st", "22nd", "101st"
            ]
        );
    }
}
