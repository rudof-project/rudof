use rbe::Context;
use serde::Serialize;
use std::fmt::Display;

/// Context passed to semantic actions when they are executed.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize)]
pub struct SemanticActionContext {
    subject: Option<String>,
    predicate: Option<String>,
    object: Option<String>,
}

impl SemanticActionContext {
    pub fn s(&self) -> Option<String> {
        self.subject.clone()
    }

    pub fn p(&self) -> Option<String> {
        self.predicate.clone()
    }

    pub fn o(&self) -> Option<String> {
        self.object.clone()
    }

    pub fn triple(subject: String, predicate: String, object: String) -> Self {
        SemanticActionContext {
            subject: Some(subject),
            predicate: Some(predicate),
            object: Some(object),
        }
    }

    pub fn subject(subject: &str) -> Self {
        SemanticActionContext {
            subject: Some(subject.to_string()),
            predicate: None,
            object: None,
        }
    }

    pub fn object(object: &str) -> Self {
        SemanticActionContext {
            subject: None,
            predicate: None,
            object: Some(object.to_string()),
        }
    }

    pub fn predicate(predicate: &str) -> Self {
        SemanticActionContext {
            subject: None,
            predicate: Some(predicate.to_string()),
            object: None,
        }
    }

    pub fn with_subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_predicate(mut self, predicate: String) -> Self {
        self.predicate = Some(predicate);
        self
    }

    pub fn with_object(mut self, object: String) -> Self {
        self.object = Some(object);
        self
    }
}

impl Context for SemanticActionContext {}

impl Display for SemanticActionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Context {{ subject: {:?}, predicate: {:?}, object: {:?} }}",
            self.subject, self.predicate, self.object
        )
    }
}
