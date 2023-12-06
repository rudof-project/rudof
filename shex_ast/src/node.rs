use std::fmt::Display;

use iri_s::IriS;
use rbe::Value;
use srdf::Object;

impl Value for Node {}

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone)]
pub struct Node {
    node: Object,
}

impl Node {
    pub fn iri(iri: IriS) -> Node {
        Node {
            node: Object::iri(iri),
        }
    }

    pub fn as_object(&self) -> &Object {
        &self.node
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl From<Object> for Node {
    fn from(node: Object) -> Self {
        Node { node }
    }
}

impl From<IriS> for Node {
    fn from(iri: IriS) -> Self {
        Node { node: iri.into() }
    }
}
