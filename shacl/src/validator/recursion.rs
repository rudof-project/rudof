//! Validator-side support for [`RecursionSemantics`]: parsing it from a
//! string, and cutting a cyclic `(node, shape)` reference during
//! validation.
//!
//! The [`RecursionSemantics`] type itself lives in [`crate::ir`] because
//! schema compilation needs it on the `wasm` target too, where this
//! (native-only) validator module isn't compiled. See that module for the
//! semantics of each variant.

use crate::error::ValidationError;
use crate::ir::{IRShape, RecursionSemantics};
use crate::types::MessageMap;
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use std::str::FromStr;

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
