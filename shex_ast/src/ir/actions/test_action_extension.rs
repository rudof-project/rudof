use iri_s::iri;
use lazy_regex::regex;

use crate::ir::actions::semantic_action_error::SemanticActionError;
use crate::ir::actions::semantic_action_extension::SemanticActionExtension;
use crate::ir::semantic_action_context::SemanticActionContext;

/// Represents the Test action extension documented [here](http://shex.io/extensions/Test/)
///
/// Supported directives on a TripleConstraint:
/// - `print(msg)` — emit a message and continue validation
/// - `fail(msg)`  — emit a message and fail validation
///
/// `msg` is either a quoted string literal (delimiters stripped, `\\` and `\"`
/// unescaped) or one of the particles `s`, `p`, `o`, which are resolved to the
/// subject, predicate, or object of the matching triple respectively.
#[derive(Debug, Clone)]
pub struct TestActionExtension {}

impl TestActionExtension {
    pub fn new() -> Self {
        TestActionExtension {}
    }
}

impl Default for TestActionExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticActionExtension for TestActionExtension {
    fn action_iri(&self) -> iri_s::IriS {
        iri!("http://shex.io/extensions/Test/")
    }

    fn run_action(&self, parameter: Option<&str>, context: &SemanticActionContext) -> Result<(), SemanticActionError> {
        let code = if let Some(parameter) = parameter {
            parameter
        } else {
            return Ok(()); // No parameter means no action, so we succeed silently.
        };

        // Pattern from the Test extension spec:
        //   ^ *(fail|print) *\( *(?:("(?:[^\\"]|\\\\|\\")*")|([spo])) *\) *$
        let re = regex!(r#"^ *(fail|print) *\( *(?:("(?:[^\\"]|\\\\|\\")*")|([spo])) *\) *$"#);

        let caps = re
            .captures(code)
            .ok_or_else(|| SemanticActionError::InvalidTestParameter {
                parameter: code.to_string(),
            })?;

        let directive = &caps[1]; // "fail" or "print"

        // Resolve the argument: either a quoted literal or a particle s/p/o.
        let message: String = if let Some(quoted) = caps.get(2) {
            // Strip surrounding quotes and unescape \\ and \"
            let inner = &quoted.as_str()[1..quoted.as_str().len() - 1];
            inner.replace(r#"\\"#, r"\").replace(r#"\""#, "\"")
        } else {
            // Particle: s, p, or o
            let particle = &caps[3];
            let binding = match particle {
                "s" => context.s(),
                "p" => context.p(),
                "o" => context.o(),
                _ => unreachable!("regex only matches s, p, or o"),
            };
            /* TODO:
               The following code raises an error if the variable is not in the binding
               By now, we return an empty string and raise a warning
            binding
                .ok_or_else(|| SemanticActionError::UnresolvedVariable {
                    variable: particle.to_string(),
                })?
                .to_string()*/
            match binding {
                Some(str) => str.to_string(),
                None => {
                    eprintln!("Warning: Unresolved variable {particle} in Test semact: no binding provided");
                    String::new()
                },
            }
        };

        match directive {
            "print" => {
                println!("{message}");
                Ok(())
            },
            "fail" => Err(SemanticActionError::FailAction { message }),
            _ => unreachable!("regex only matches fail or print"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ext() -> TestActionExtension {
        TestActionExtension {}
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
    fn fail_literal() {
        let err = ext()
            .run_action(Some(r#"fail("bad value")"#), &SemanticActionContext::default())
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::FailAction { message } if message == "bad value"));
    }

    #[test]
    fn fail_object() {
        let err = ext()
            .run_action(
                Some("fail(o)"),
                &SemanticActionContext::object("http://example.org/bad"),
            )
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
        let err = ext()
            .run_action(Some("unknown(s)"), &SemanticActionContext::default())
            .unwrap_err();
        assert!(matches!(err, SemanticActionError::InvalidTestParameter { .. }));
    }

    #[test]
    fn empty_parameter() {
        ext().run_action(None, &SemanticActionContext::default()).unwrap();
    }
}
