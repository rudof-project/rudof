//! Errors of the ShExMap module.

use thiserror::Error;

use crate::shexmap::bindings::Bindings;

/// One dead end met while materializing: which constraint failed and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    /// The output constraint's predicate.
    pub predicate: String,
    /// The variable that was unbound, when that is the reason.
    pub variable: Option<String>,
    /// The Map code involved, when there is one.
    pub code: Option<String>,
    /// What went wrong, when it is not an unbound variable.
    pub error: Option<String>,
    /// The input scope the constraint was evaluated at (list index, iteration index, ...).
    pub scope: Option<Vec<usize>>,
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = self.variable.as_deref().or(self.code.as_deref()).unwrap_or("");
        let why = self.error.as_deref().unwrap_or("unbound");
        write!(f, "{} {what} {why}", self.predicate)
    }
}

#[derive(Error, Debug)]
pub enum ShExMapError {
    /// A `%Map:{ ... %}` code is malformed or cannot be applied to its input.
    #[error("ShExMap: {msg}")]
    Function { msg: String },

    /// The input data does not conform to the input schema, so nothing can be bound.
    #[error("{node} does not conform to the input schema{}", details.as_ref().map(|d| format!(": {d}")).unwrap_or_default())]
    Validation { node: String, details: Option<String> },

    /// The input conforms in more than one way, and the ways bind different values.
    #[error("{node} matches the input schema in {} ways that bind different values", alternatives.len())]
    Ambiguous { node: String, alternatives: Vec<Bindings> },

    /// The binding tree is not a scope tree (nor one of the older layouts).
    #[error("binding tree: {msg}")]
    BindingTree { msg: String },

    /// No alternative materializes the requested shape from the bindings.
    #[error("{msg}{}", describe_failures(failures))]
    Materialization { msg: String, failures: Vec<Failure> },

    /// The schema cannot be used for mapping (undefined reference, no start shape, ...).
    #[error("schema: {msg}")]
    Schema { msg: String },

    /// Reading or writing RDF failed.
    #[error("RDF: {msg}")]
    Rdf { msg: String },
}

fn describe_failures(failures: &[Failure]) -> String {
    if failures.is_empty() {
        return String::new();
    }
    let last: Vec<String> = failures
        .iter()
        .rev()
        .take(3)
        .rev()
        .map(std::string::ToString::to_string)
        .collect();
    format!("; deepest failures: {}", last.join("; "))
}

impl ShExMapError {
    pub fn schema(msg: impl Into<String>) -> Self {
        ShExMapError::Schema { msg: msg.into() }
    }

    pub fn materialization(msg: impl Into<String>) -> Self {
        ShExMapError::Materialization {
            msg: msg.into(),
            failures: Vec::new(),
        }
    }

    pub fn binding_tree(msg: impl Into<String>) -> Self {
        ShExMapError::BindingTree { msg: msg.into() }
    }

    pub fn rdf(msg: impl std::fmt::Display) -> Self {
        ShExMapError::Rdf { msg: msg.to_string() }
    }
}
