use crate::ir::actions::semantic_action_error::SemanticActionError;
use crate::ir::actions::semantic_action_extension::SemanticActionExtension;
use iri_s::iri;

/// Represents the ShExMap action extension documented [here](http://shex.io/extensions/Map/)
///
#[derive(Debug, Clone)]
pub struct MapActionExtension {}

impl MapActionExtension {
    pub fn new() -> Self {
        MapActionExtension {}
    }
}

impl Default for MapActionExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticActionExtension for MapActionExtension {
    fn action_iri(&self) -> iri_s::IriS {
        iri!("http://shex.io/extensions/Map/")
    }

    fn run_action(
        &self,
        _parameter: Option<&str>,
        context: &crate::ir::semantic_action_context::SemanticActionContext,
    ) -> Result<(), SemanticActionError> {
        println!("Node: {}", context.s().unwrap_or("None".to_string()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::semantic_action_context::SemanticActionContext;

    use super::*;

    fn ext() -> MapActionExtension {
        MapActionExtension {}
    }

    #[test]
    fn print_literal() {
        ext()
            .run_action(Some(r#"print("hello world")"#), &SemanticActionContext::default())
            .unwrap();
    }

    #[test]
    fn print_escaped_literal() {
        ext()
            .run_action(Some(r#"print("say \"hi\"")"#), &SemanticActionContext::default())
            .unwrap();
    }

    #[test]
    fn print_subject() {
        ext()
            .run_action(
                Some("print(s)"),
                &SemanticActionContext::subject("http://example.org/s"),
            )
            .unwrap();
    }

    #[test]
    fn empty_parameter() {
        ext().run_action(None, &SemanticActionContext::default()).unwrap();
    }
}
