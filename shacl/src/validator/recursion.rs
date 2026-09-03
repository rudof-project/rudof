//! Cycle-breaking for recursive shapes.
//!
//! A shape can reference itself, directly or through other shapes (e.g. a
//! `Person` shape whose `knows` property must itself be a `Person`). Without
//! special handling, validating such a shape against data that actually
//! contains a cycle would recurse forever. Instead, while validating
//! `(node, shape)`, if a nested validation call re-enters that very same
//! `(node, shape)` pair, we cut the cycle and assume a default verdict for
//! that inner reference, then keep evaluating the rest of the shape's
//! constraints against that assumption. The default is a choice of
//! semantics ([`RecursionSemantics`]): assume the cut reference does *not*
//! conform (cautious) or *does* conform (brave).
//!
//! Only positive recursion (shapes combined via `sh:and`, `sh:or`,
//! `sh:node`, `sh:property`, `sh:minCount`, `sh:closed`, ...) is soundly
//! handled by this: those constraints are monotonic, so cutting a cycle can
//! only make the final verdict more permissive or more restrictive in a
//! well-behaved way. Constructs that rely on negation (`sh:not`, `sh:xone`,
//! `sh:qualifiedMaxCount`, `sh:qualifiedValueShapesDisjoint`) are cut the
//! same way here for simplicity, but a cycle passing through one of them is
//! not guaranteed sound — stratified negation is left for a future
//! extension.

use crate::error::ValidationError;
use crate::ir::IRShape;
use crate::types::MessageMap;
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Which fixpoint semantics to use when a recursive shape reference is
/// encountered during validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecursionSemantics {
    /// Least fixpoint (LFP): "believe a node conforms only if that can be
    /// justified without ever assuming the very fact being proven." A
    /// cyclic reference is cut by assuming it does **not** conform.
    #[default]
    Cautious,
    /// Greatest fixpoint (GFP): "accept any assignment that is
    /// self-consistent, even if the only reason it holds together is the
    /// cycle itself." A cyclic reference is cut by assuming it **does**
    /// conform.
    Brave,
}

impl Display for RecursionSemantics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecursionSemantics::Cautious => write!(f, "cautious"),
            RecursionSemantics::Brave => write!(f, "brave"),
        }
    }
}

impl FromStr for RecursionSemantics {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
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
pub(crate) fn cut_outcome(semantics: RecursionSemantics, shape: &IRShape, node: &Object) -> ValidationOutcome {
    let component = recursion_constraint_component();
    match semantics {
        RecursionSemantics::Brave => {
            let evidence = Evidence::new(node.clone(), component).with_source(Some(shape.id().clone()));
            ValidationOutcome::from_evidence(evidence)
        },
        RecursionSemantics::Cautious => {
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
    }

    #[test]
    fn from_str_accepts_names_and_lfp_gfp_aliases() {
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
        for s in [RecursionSemantics::Cautious, RecursionSemantics::Brave] {
            assert_eq!(s.to_string().parse::<RecursionSemantics>().unwrap(), s);
        }
    }
}
