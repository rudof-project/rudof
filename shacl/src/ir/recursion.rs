//! Cycle-breaking for recursive shapes.
//!
//! A shape can reference itself, directly or through other shapes (e.g. a
//! `Person` shape whose `knows` property must itself be a `Person`).
//! [`RecursionSemantics::None`] rejects a shapes graph with such a cycle
//! outright. The other two values accept it and give it a semantics: while
//! validating `(node, shape)`, if a nested validation call re-enters that
//! very same `(node, shape)` pair, the cycle is cut by assuming a default
//! verdict for that inner reference — instead of recursing forever — and
//! the rest of the shape's constraints are evaluated against that
//! assumption. [`RecursionSemantics::Cautious`] (the default) assumes the
//! cut reference does *not* conform; [`RecursionSemantics::Brave`] assumes
//! it *does*.
//!
//! Only positive recursion (shapes combined via `sh:and`, `sh:or`,
//! `sh:node`, `sh:property`, `sh:minCount`, `sh:closed`, ...) is soundly
//! handled by cautious/brave: those constraints are monotonic, so cutting a
//! cycle can only make the final verdict more permissive or more
//! restrictive in a well-behaved way. Constructs that rely on negation
//! (`sh:not`, `sh:xone`, `sh:qualifiedMaxCount`,
//! `sh:qualifiedValueShapesDisjoint`) can't be soundly handled this way yet
//! — stratified negation is left for a future extension — so a schema whose
//! only cycles pass through one of them is always rejected, regardless of
//! `RecursionSemantics`.
//!
//! This type lives in the IR module (rather than the validator module)
//! because schema compilation (`IRSchema::compile_with_recursion`) needs it
//! to decide whether a cyclic shapes graph is acceptable, and the IR module
//! — unlike the validator — is compiled for the `wasm` target too.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Whether — and how — to accept a recursive (cyclic) shape reference.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecursionSemantics {
    /// Reject a shapes graph that has a cyclic shape reference.
    None,
    /// Least fixpoint (LFP): "believe a node conforms only if that can be
    /// justified without ever assuming the very fact being proven." A
    /// cyclic reference is cut by assuming it does **not** conform. The
    /// default: recursive shapes are accepted, but nothing conforms merely
    /// because a cycle looks self-consistent.
    #[default]
    Cautious,
    /// Greatest fixpoint (GFP): "accept any assignment that is
    /// self-consistent, even if the only reason it holds together is the
    /// cycle itself." A cyclic reference is cut by assuming it **does**
    /// conform.
    Brave,
}

impl RecursionSemantics {
    /// Whether a shapes graph with a (purely positive) cyclic shape
    /// reference should be accepted at all, rather than rejected up front.
    pub fn allows_recursion(self) -> bool {
        self != RecursionSemantics::None
    }
}

impl Display for RecursionSemantics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecursionSemantics::None => write!(f, "none"),
            RecursionSemantics::Cautious => write!(f, "cautious"),
            RecursionSemantics::Brave => write!(f, "brave"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_cautious() {
        assert_eq!(RecursionSemantics::default(), RecursionSemantics::Cautious);
        assert!(!RecursionSemantics::None.allows_recursion());
        assert!(RecursionSemantics::Cautious.allows_recursion());
        assert!(RecursionSemantics::Brave.allows_recursion());
    }

    #[test]
    fn display_matches_serde_names() {
        assert_eq!(RecursionSemantics::None.to_string(), "none");
        assert_eq!(RecursionSemantics::Cautious.to_string(), "cautious");
        assert_eq!(RecursionSemantics::Brave.to_string(), "brave");
    }
}
