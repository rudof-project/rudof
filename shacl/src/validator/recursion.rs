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

use crate::error::ValidationError;
use crate::ir::IRShape;
use crate::types::MessageMap;
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

impl FromStr for RecursionSemantics {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "cautious" | "lfp" => Ok(Self::Cautious),
            "brave" | "gfp" => Ok(Self::Brave),
            other => Err(Self::Err::UnsupportedMode(other.to_string())),
        }
    }
}

/// Marker "constraint component" used to tag the [`Evidence`]/[`ValidationResult`]
/// synthesized when a recursive shape reference is cut. Not a real SHACL
/// vocabulary term — the SHACL Recommendation doesn't define recursive-shape
/// semantics — so this is a rudof-local marker rather than something minted
/// under the `sh:` namespace.
fn recursion_constraint_component() -> Object {
    Object::iri(IriS::new_unchecked(
        "https://rudof-project.github.io/rudof/ns/shacl-recursion#RecursiveReference",
    ))
}

/// Cuts a cyclic reference to `(node, shape)`, returning the outcome assumed
/// under `semantics` instead of recursing further.
///
/// A schema compiled with [`RecursionSemantics::None`] never has a cycle to
/// cut in the first place (compilation rejects it first — see
/// `IRSchema::compile_with_recursion`), so this should never actually be
/// called with `None`. It's handled the same as `Cautious` regardless, so
/// that reusing an already-compiled cyclic schema under a config that was
/// changed to `None` afterwards degrades safely instead of panicking.
pub(crate) fn cut_outcome(semantics: RecursionSemantics, shape: &IRShape, node: &Object) -> ValidationOutcome {
    let component = recursion_constraint_component();
    match semantics {
        RecursionSemantics::Brave => {
            let evidence = Evidence::new(node.clone(), component).with_source(Some(shape.id().clone()));
            ValidationOutcome::from_evidence(evidence)
        },
        RecursionSemantics::Cautious | RecursionSemantics::None => {
            let msg = format!(
                "Recursive reference to shape {} for node {node} assumed non-conformant (cautious/LFP semantics)",
                shape.id()
            );
            let violation = ValidationResult::new(node.clone(), component, shape.severity().clone())
                .with_message(MessageMap::from(msg))
                .with_source(Some(shape.id().clone()));
            ValidationOutcome::from_violation(violation)
        },
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
    fn from_str_accepts_names_and_lfp_gfp_aliases() {
        assert_eq!("none".parse::<RecursionSemantics>().unwrap(), RecursionSemantics::None);
        assert_eq!(
            "cautious".parse::<RecursionSemantics>().unwrap(),
            RecursionSemantics::Cautious
        );
        assert_eq!(
            "LFP".parse::<RecursionSemantics>().unwrap(),
            RecursionSemantics::Cautious
        );
        assert_eq!(
            "brave".parse::<RecursionSemantics>().unwrap(),
            RecursionSemantics::Brave
        );
        assert_eq!("GFP".parse::<RecursionSemantics>().unwrap(), RecursionSemantics::Brave);
        assert!("other".parse::<RecursionSemantics>().is_err());
    }

    #[test]
    fn display_round_trips_through_from_str() {
        for s in [
            RecursionSemantics::None,
            RecursionSemantics::Cautious,
            RecursionSemantics::Brave,
        ] {
            assert_eq!(s.to_string().parse::<RecursionSemantics>().unwrap(), s);
        }
    }
}
