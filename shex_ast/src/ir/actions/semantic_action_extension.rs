use crate::ir::actions::semantic_action_error::SemanticActionError;
use iri_s::IriS;

/// A Semantic Action Extension represents an Extension of the ShEx extension
/// Some examples of semantic action extensions are [here](http://shex.io/extensions/)
///
pub trait SemanticActionExtension {
    fn action_iri(&self) -> IriS;

    /// Execute the semact body `parameter`.
    ///
    /// `s`, `p`, `o` are the optional subject, predicate, and object bindings
    /// available for TripleExpression semacts. For ShapeExpression semacts only
    /// `s` (focus node) is defined. For Start semacts all three are `None`.
    fn run_action(
        &self,
        parameter: Option<&str>,
        s: Option<&str>,
        p: Option<&str>,
        o: Option<&str>,
    ) -> Result<(), SemanticActionError>;
}
