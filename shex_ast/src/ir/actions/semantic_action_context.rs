use rbe::Context;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::ir::actions::semantic_actions_registry::SemanticActionsRegistry;
use crate::{Node, Pred};

/// Context passed to semantic actions when they are executed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SemanticActionContext {
    subject: Option<Node>,
    predicate: Option<Pred>,
    object: Option<Node>,

    #[serde(skip)]
    registry: Option<Arc<SemanticActionsRegistry>>,
}

impl SemanticActionContext {
    pub fn new_start_act_context() -> Self {
        SemanticActionContext {
            subject: None,
            predicate: None,
            object: None,
            registry: None,
        }
    }
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
            registry: None,
        }
    }

    pub fn subject(subject: &Node) -> Self {
        SemanticActionContext {
            subject: Some(subject.clone()),
            predicate: None,
            object: None,
            registry: None,
        }
    }

    pub fn object(object: &Node) -> Self {
        SemanticActionContext {
            subject: None,
            predicate: None,
            object: Some(object.clone()),
            registry: None,
        }
    }

    pub fn predicate(predicate: &Pred) -> Self {
        SemanticActionContext {
            subject: None,
            predicate: Some(predicate.clone()),
            object: None,
            registry: None,
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

    pub fn with_registry(mut self, registry: Arc<SemanticActionsRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }

    pub fn registry(&self) -> Option<&SemanticActionsRegistry> {
        self.registry.as_deref()
    }
}

impl PartialEq for SemanticActionContext {
    fn eq(&self, other: &Self) -> bool {
        self.subject == other.subject
            && self.predicate == other.predicate
            && self.object == other.object
    }
}

impl Eq for SemanticActionContext {}

impl Hash for SemanticActionContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.subject.hash(state);
        self.predicate.hash(state);
        self.object.hash(state);
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
