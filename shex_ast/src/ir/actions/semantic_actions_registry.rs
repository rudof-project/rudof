use iri_s::IriS;

use crate::ir::actions::{
    semantic_action_error::SemanticActionError, semantic_action_extension::SemanticActionExtension,
};

pub struct SemanticActionsRegistry {
    extensions: Vec<Box<dyn SemanticActionExtension>>,
}

impl SemanticActionsRegistry {
    pub fn new() -> Self {
        Self { extensions: Vec::new() }
    }

    pub fn register(&mut self, extension: Box<dyn SemanticActionExtension>) {
        self.extensions.push(extension);
    }

    pub fn run_action(
        &self,
        action_iri: &IriS,
        parameter: Option<&str>,
        s: Option<&str>,
        p: Option<&str>,
        o: Option<&str>,
    ) -> Result<(), SemanticActionError> {
        let ext = self
            .extensions
            .iter()
            .find(|e| &e.action_iri() == action_iri)
            .ok_or_else(|| SemanticActionError::UnknownExtension {
                iri: action_iri.to_string(),
            })?;
        ext.run_action(parameter, s, p, o)
    }
}

#[cfg(test)]
mod tests {
    use iri_s::{IriS, iri};

    use crate::ir::actions::{
        semantic_action_error::SemanticActionError, semantic_actions_registry::SemanticActionsRegistry,
        test_action_extension::TestActionExtension,
    };

    fn test_iri() -> IriS {
        iri!("http://shex.io/extensions/Test/")
    }

    fn registry_with_test() -> SemanticActionsRegistry {
        let mut r = SemanticActionsRegistry::new();
        r.register(Box::new(TestActionExtension::new()));
        r
    }

    #[test]
    fn print_dispatches_to_test_extension() {
        registry_with_test()
            .run_action(&test_iri(), Some(r#"print("ok")"#), None, None, None)
            .unwrap();
    }

    #[test]
    fn fail_dispatches_to_test_extension() {
        let err = registry_with_test()
            .run_action(&test_iri(), Some(r#"fail("bad")"#), None, None, None)
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::FailAction { message } if message == "bad"));
    }

    #[test]
    fn unknown_iri_returns_error() {
        let unknown = iri!("http://example.org/unknown/");
        let err = registry_with_test()
            .run_action(&unknown, Some(r#"print("x")"#), None, None, None)
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::UnknownExtension { .. }));
    }

    #[test]
    fn empty_registry_returns_error() {
        let err = SemanticActionsRegistry::new()
            .run_action(&test_iri(), Some(r#"print("x")"#), None, None, None)
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::UnknownExtension { .. }));
    }
}
