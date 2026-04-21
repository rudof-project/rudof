use rbe::Context;
use serde::Serialize;
use std::fmt::Display;

use crate::{Node, Pred};

/// Context passed to semantic actions when they are executed.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize)]
pub struct SemanticActionContext {
    subject: Option<Node>,
    predicate: Option<Pred>,
    object: Option<Node>,
}

impl SemanticActionContext {
    pub fn s(&self) -> Option<Node> {
        self.subject.clone()
    }

    pub fn p(&self) -> Option<Pred> {
        self.predicate.clone()
    }

    pub fn o(&self) -> Option<Node> {
        self.object.clone()
    }

    pub fn triple(subject: &Node, predicate: &Pred, object: &Node) -> Self {
        SemanticActionContext {
            subject: Some(subject.clone()),
            predicate: Some(predicate.clone()),
            object: Some(object.clone()),
        }
    }

    pub fn subject(subject: &Node) -> Self {
        SemanticActionContext {
            subject: Some(subject.clone()),
            predicate: None,
            object: None,
        }
    }

    pub fn object(object: &Node) -> Self {
        SemanticActionContext {
            subject: None,
            predicate: None,
            object: Some(object.clone()),
        }
    }

    pub fn predicate(predicate: &Pred) -> Self {
        SemanticActionContext {
            subject: None,
            predicate: Some(predicate.clone()),
            object: None,
        }
    }

    pub fn with_subject(mut self, subject: Node) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_predicate(mut self, predicate: Pred) -> Self {
        self.predicate = Some(predicate);
        self
    }

    pub fn with_object(mut self, object: Node) -> Self {
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
