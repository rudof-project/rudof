use crate::Node;
use crate::ir::actions::semantic_action_error::SemanticActionError;
use crate::ir::actions::semantic_action_extension::SemanticActionExtension;
use crate::ir::map_state::MapState;
use crate::ir::semantic_action_context::SemanticActionContext;
use crate::shexmap::functions;
use rudof_iri::{IriS, iri};
use std::sync::{Arc, Mutex};
use tracing::trace;

/// Represents the ShExMap action extension documented [here](http://shex.io/extensions/Map/)
///
/// A `%Map:{ ... %}` code on a triple constraint is a variable name (`<iri>` or
/// `prefix:local`) or a function call (`regex(...)`, `hashmap(...)`, `id(...)`; see
/// [`crate::shexmap::functions`]).  During validation:
///
/// * a variable written as an IRI records the matched value in the [`MapState`] (the flat
///   map the `materialize` command reads); a prefixed name cannot be expanded here, since the
///   schema's prefixes are not passed to semantic actions, and is accepted as is -- the
///   [`crate::shexmap`] module binds both after validation, with the nesting kept;
/// * `regex(...)` and `hashmap(...)` must apply to the matched value, else the triple
///   constraint fails, as in shex.js;
/// * `id(...)` names an output node and is accepted.
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
        let Some(code) = parameter else {
            return Err(SemanticActionError::InvalidParameter {
                details: "No parameter provided".to_string(),
            });
        };
        trace!("Parameter: {}", code);
        let code = code.trim();
        // the matched value: the node a triple constraint's condition is evaluated on
        let value = context.n().or_else(|| context.o());
        if functions::is_function_call(code) {
            let prefixes = functions::any_prefixes(code);
            if functions::is_key_code(code) {
                functions::key_arguments(code, &prefixes)
                    .map_err(|e| SemanticActionError::MapFunctionFailed { details: e.to_string() })?;
                return Ok(());
            }
            let Some(value) = value else {
                return Err(SemanticActionError::NoObjectInContext {
                    details: "No object provided in context".to_string(),
                });
            };
            let bound = functions::lift(code, value.as_object(), &prefixes)
                .map_err(|e| SemanticActionError::MapFunctionFailed { details: e.to_string() })?;
            let mut st = self.state.lock().unwrap();
            for (var, val) in bound {
                if !var.starts_with(functions::PLACEHOLDER_NAMESPACE) {
                    st.insert(IriS::new_unchecked(&var), Node::new(val));
                }
            }
            return Ok(());
        }
        if code.starts_with('<') {
            let iri = IriS::parse_turtle(code).map_err(|e| SemanticActionError::InvalidParameter {
                details: format!("Invalid IRI parameter: {}", e),
            })?;
            let Some(value) = value else {
                return Err(SemanticActionError::NoObjectInContext {
                    details: "No object provided in context".to_string(),
                });
            };
            trace!("Object from context: {}", value);
            let mut st = self.state.lock().unwrap();
            st.insert(iri, value);
            trace!("Updated state: {}", *st);
            return Ok(());
        }
        // a prefixed variable name: checked for form only
        functions::expand_variable(code, &functions::any_prefixes(code))
            .map(|_| ())
            .map_err(|e| SemanticActionError::InvalidParameter { details: e.to_string() })
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

#[cfg(test)]
mod shexmap_tests {
    use super::*;
    use rudof_rdf::rdf_core::term::{Object, literal::ConcreteLiteral};

    fn lit(s: &str) -> Node {
        Node::new(Object::literal(ConcreteLiteral::str(s)))
    }

    #[test]
    fn prefixed_variables_are_accepted() {
        let ext = MapActionExtension::new(MapState::default());
        let ctx = SemanticActionContext::default().with_node(lit("Sue"));
        ext.run_action(Some(" bp:given "), &ctx).unwrap();
        assert!(ext.get_state().lock().unwrap().is_empty());
    }

    #[test]
    fn regex_must_match() {
        let ext = MapActionExtension::new(MapState::default());
        let code = "regex(/(?<bp:family>[a-zA-Z]+), (?<bp:given>[a-zA-Z]+)/)";
        ext.run_action(
            Some(code),
            &SemanticActionContext::default().with_node(lit("Walker, Alice")),
        )
        .unwrap();
        let err = ext
            .run_action(Some(code), &SemanticActionContext::default().with_node(lit("nope")))
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::MapFunctionFailed { .. }));
    }

    #[test]
    fn regex_with_iri_names_records_bindings() {
        let ext = MapActionExtension::new(MapState::default());
        let code = "regex(/(?<<http://v/family>>[a-zA-Z]+), (?<<http://v/given>>[a-zA-Z]+)/)";
        ext.run_action(
            Some(code),
            &SemanticActionContext::default().with_node(lit("Walker, Alice")),
        )
        .unwrap();
        let state = ext.get_state();
        let guard = state.lock().unwrap();
        assert_eq!(guard.get(&IriS::new_unchecked("http://v/given")), Some(&lit("Alice")));
    }

    #[test]
    fn id_is_accepted() {
        let ext = MapActionExtension::new(MapState::default());
        ext.run_action(
            Some("id(bp:a, @node)"),
            &SemanticActionContext::default().with_node(lit("x")),
        )
        .unwrap();
        assert!(ext.run_action(Some("id()"), &SemanticActionContext::default()).is_err());
    }
}
