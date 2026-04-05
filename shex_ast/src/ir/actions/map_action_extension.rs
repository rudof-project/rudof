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
        s: Option<&str>,
        _p: Option<&str>,
        _o: Option<&str>,
    ) -> Result<(), SemanticActionError> {
        println!("Node: {}", s.unwrap_or("None"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ext() -> MapActionExtension {
        MapActionExtension {}
    }

    #[test]
    fn print_literal() {
        ext()
            .run_action(Some(r#"print("hello world")"#), None, None, None)
            .unwrap();
    }

    #[test]
    fn print_escaped_literal() {
        ext()
            .run_action(Some(r#"print("say \"hi\"")"#), None, None, None)
            .unwrap();
    }

    #[test]
    fn print_subject() {
        ext()
            .run_action(Some("print(s)"), Some("http://example.org/s"), None, None)
            .unwrap();
    }

    #[test]
    fn fail_literal() {
        let err = ext()
            .run_action(Some(r#"fail("bad value")"#), None, None, None)
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::FailAction { message } if message == "bad value"));
    }

    #[test]
    fn fail_object() {
        let err = ext()
            .run_action(Some("fail(o)"), None, None, Some("http://example.org/bad"))
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::FailAction { message } if message == "http://example.org/bad"));
    }

    /*#[test]
    fn unresolved_variable() {
        let err = ext().run_action(Some("print(p)"), None, None, None).unwrap_err();
        assert!(matches!(err, SemanticActionError::UnresolvedVariable { variable } if variable == "p"));
    }*/

    #[test]
    fn invalid_parameter() {
        let err = ext().run_action(Some("unknown(s)"), None, None, None).unwrap_err();
        assert!(matches!(err, SemanticActionError::InvalidTestParameter { .. }));
    }

    #[test]
    fn empty_parameter() {
        ext().run_action(None, None, None, None).unwrap();
    }
}
