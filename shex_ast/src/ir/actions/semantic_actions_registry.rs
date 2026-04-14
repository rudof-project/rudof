use crate::ir::{
    actions::{semantic_action_error::SemanticActionError, semantic_action_extension::SemanticActionExtension},
    map_action_extension::MapActionExtension,
    map_state::MapState,
    semantic_action_context::SemanticActionContext,
};
use iri_s::IriS;
use std::{
    fmt,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct SemanticActionsRegistry {
    extensions: Vec<Arc<dyn SemanticActionExtension + Send + Sync>>,
}

impl SemanticActionsRegistry {
    pub fn new() -> Self {
        Self { extensions: Vec::new() }
    }

    /// Create a registry with the given extensions pre-registered.
    pub fn with(mut self, extensions: Vec<Box<dyn SemanticActionExtension + Send + Sync>>) -> Self {
        for ext in extensions {
            self.register(ext);
        }
        self
    }

    pub fn register(&mut self, extension: Box<dyn SemanticActionExtension + Send + Sync>) {
        self.extensions.push(Arc::from(extension));
    }

    pub fn set_map_state(&mut self, map_state: &mut MapState) {
        for ext in &self.extensions {
            if let Some(map_ext) = ext.as_any().downcast_ref::<MapActionExtension>() {
                map_ext.set_state(map_state.clone());
            }
        }
    }

    /// Return a shared reference to the live `MapState` held by the registered
    /// `MapActionExtension`, if one is present.
    ///
    /// Because `MapActionExtension` stores its state inside an `Arc<Mutex<MapState>>`, any
    /// mutations performed by validation closures are immediately visible through the returned
    /// Arc — even when the registry or the `SchemaIR` has been cloned, since Arc-clone is
    /// reference-counted rather than deep-copied.
    pub fn get_map_state_arc(&self) -> Option<Arc<Mutex<MapState>>> {
        for ext in &self.extensions {
            if let Some(map_ext) = ext.as_any().downcast_ref::<MapActionExtension>() {
                return Some(map_ext.get_state());
            }
        }
        None
    }

    /// Find the extension registered for `action_iri`.
    ///
    /// Returns a cloned `Arc` to the extension so the caller can embed it in a
    /// long-lived closure (e.g. a `MatchCond`).
    pub fn find_code(
        &self,
        action_iri: &IriS,
    ) -> Result<Arc<dyn SemanticActionExtension + Send + Sync>, SemanticActionError> {
        self.extensions
            .iter()
            .find(|e| &e.action_iri() == action_iri)
            .cloned()
            .ok_or_else(|| SemanticActionError::UnknownExtension {
                iri: action_iri.to_string(),
            })
    }

    /// Run the action identified by `action_iri` with the given parameters.
    pub fn run_action(
        &self,
        action_iri: &IriS,
        parameter: Option<&str>,
        context: &SemanticActionContext,
    ) -> Result<(), SemanticActionError> {
        let ext = self
            .extensions
            .iter()
            .find(|e| &e.action_iri() == action_iri)
            .ok_or_else(|| SemanticActionError::UnknownExtension {
                iri: action_iri.to_string(),
            })?;
        ext.run_action(parameter, context)
    }
}

impl fmt::Debug for SemanticActionsRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticActionsRegistry")
            .field("extensions_count", &self.extensions.len())
            .finish()
    }
}

impl Default for SemanticActionsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use iri_s::{IriS, iri};

    use crate::ir::{
        actions::{
            semantic_action_error::SemanticActionError, semantic_actions_registry::SemanticActionsRegistry,
            test_action_extension::TestActionExtension,
        },
        semantic_action_context::SemanticActionContext,
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
            .run_action(&test_iri(), Some(r#"print("ok")"#), &SemanticActionContext::default())
            .unwrap();
    }

    #[test]
    fn fail_dispatches_to_test_extension() {
        let err = registry_with_test()
            .run_action(&test_iri(), Some(r#"fail("bad")"#), &SemanticActionContext::default())
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::FailAction { message } if message == "bad"));
    }

    #[test]
    fn unknown_iri_returns_error() {
        let unknown = iri!("http://example.org/unknown/");
        let err = registry_with_test()
            .run_action(&unknown, Some(r#"print("x")"#), &SemanticActionContext::default())
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::UnknownExtension { .. }));
    }

    #[test]
    fn empty_registry_returns_error() {
        let err = SemanticActionsRegistry::new()
            .run_action(&test_iri(), Some(r#"print("x")"#), &SemanticActionContext::default())
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::UnknownExtension { .. }));
    }

    #[test]
    fn find_code_returns_extension() {
        let ext = registry_with_test().find_code(&test_iri()).unwrap();
        assert_eq!(ext.action_iri(), test_iri());
    }

    #[test]
    fn find_code_unknown_returns_error() {
        let unknown = iri!("http://example.org/unknown/");
        let result = registry_with_test().find_code(&unknown);
        assert!(matches!(result, Err(SemanticActionError::UnknownExtension { .. })));
    }
}
