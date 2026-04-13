use crate::ir::{actions::semantic_action_error::SemanticActionError, semantic_action_context::SemanticActionContext};
use iri_s::IriS;

/// A Semantic Action Extension represents an Extension of the ShEx extension
/// Some examples of semantic action extensions are [here](http://shex.io/extensions/)
///
pub trait SemanticActionExtension {
    fn action_iri(&self) -> IriS;

    /// Run the semantic action body, which has been passed as `parameter` using the context provided
    /// in `context`.
    fn run_action(&self, parameter: Option<&str>, context: &SemanticActionContext) -> Result<(), SemanticActionError>;

    fn as_any(&self) -> &dyn std::any::Any;
}
