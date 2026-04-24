use crate::ir::actions::semantic_action_error::SemanticActionError;
use crate::ir::actions::semantic_action_extension::SemanticActionExtension;
use crate::ir::map_state::MapState;
use crate::ir::semantic_action_context::SemanticActionContext;
use rudof_iri::{IriS, iri};
use std::sync::{Arc, Mutex};
use tracing::trace;

/// Represents the ShExMap action extension documented [here](http://shex.io/extensions/Map/)
///
#[derive(Debug, Clone)]
pub struct MapActionExtension {
    state: Arc<Mutex<MapState>>,
}

impl MapActionExtension {
    /// Create a new MapActionExtension with the given initial state.
    pub fn new(state: MapState) -> Self {
        MapActionExtension {
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<MapState>> {
        Arc::clone(&self.state)
    }

    pub fn set_state(&self, new_state: MapState) {
        let mut st = self.state.lock().unwrap();
        *st = new_state;
    }
}

impl SemanticActionExtension for MapActionExtension {
    fn action_iri(&self) -> rudof_iri::IriS {
        iri!("http://shex.io/extensions/Map/")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn run_action(&self, parameter: Option<&str>, context: &SemanticActionContext) -> Result<(), SemanticActionError> {
        if let Some(param) = parameter {
            trace!("Parameter: {}", param);
            let iri = IriS::parse_turtle(param.trim()).map_err(|e| SemanticActionError::InvalidParameter {
                details: format!("Invalid IRI parameter: {}", e),
            })?;
            if let Some(object) = context.o() {
                trace!("Object from context: {}", object);
                let mut st = self.state.lock().unwrap();
                st.insert(iri, object);
                trace!("Updated state: {}", *st);
            } else {
                return Err(SemanticActionError::NoObjectInContext {
                    details: "No object provided in context".to_string(),
                });
            }
        } else {
            return Err(SemanticActionError::InvalidParameter {
                details: "No parameter provided".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    use super::*;

    fn ext() -> MapActionExtension {
        MapActionExtension::new(MapState::default())
    }

    #[test]
    fn map_valid_iri_with_object() {
        let ctx = SemanticActionContext::object(&Node::iri(iri!("http://example.org/value")));
        ext().run_action(Some("<http://example.org/x>"), &ctx).unwrap();
    }

    #[test]
    fn map_no_parameter_returns_error() {
        let err = ext().run_action(None, &SemanticActionContext::default()).unwrap_err();
        assert!(matches!(err, SemanticActionError::InvalidParameter { .. }));
    }

    #[test]
    fn map_no_object_in_context_returns_error() {
        let err = ext()
            .run_action(Some("<http://example.org/x>"), &SemanticActionContext::default())
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::NoObjectInContext { .. }));
    }

    #[test]
    fn map_state_is_updated() {
        let ext = MapActionExtension::new(MapState::default());
        let ctx = SemanticActionContext::object(&Node::iri(iri!("http://example.org/value")));
        ext.run_action(Some("<http://example.org/x>"), &ctx).unwrap();
        let state = ext.get_state();
        let guard = state.lock().unwrap();
        let iri = IriS::new("http://example.org/x").unwrap();
        assert!(guard.get(&iri).is_some());
    }
}
