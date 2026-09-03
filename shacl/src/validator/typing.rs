use crate::ir::ShapeLabelIdx;
use crate::validator::report::{Evidence, ValidationResult};
use rudof_rdf::rdf_core::term::Object;

/// SHACL's instantiation of `rudof_typing`'s generic validation outcome:
/// either the violations why a `(node, shape label)` pair doesn't conform,
/// or the evidence why it does. Mirrors ShEx's own instantiation in
/// `shex_validation::typing`.
pub type Verdict = rudof_typing::ValidationResult<ValidationResult, Evidence>;

/// SHACL's instantiation of the generic memoization-cache trait. See
/// [`rudof_typing::Typing`] for the generic contract.
pub use rudof_typing::Typing;

/// SHACL's instantiation of the generic memoization cache: a plain
/// `HashMap`-backed [`Typing`]. Wrapped behind a lock in
/// [`crate::validator::cache::SharedTyping`] so it can be shared across the
/// engines that validate a topological level in parallel.
pub type ObservableTyping = rudof_typing::ObservableTyping<Object, ShapeLabelIdx, ValidationResult, Evidence>;
