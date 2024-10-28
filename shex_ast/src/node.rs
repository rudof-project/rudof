use iri_s::IriS;
use rbe::Value;
use serde_derive::Serialize;
use srdf::numeric_literal::NumericLiteral;
use srdf::Object;
use std::fmt::Display;

impl Value for Node {}

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Serialize)]
pub struct Node {
    node: Object,
}

impl Node {
    /// Creates a node from an [`ÌriS`]
    pub fn iri(iri: IriS) -> Node {
        Node {
            node: Object::iri(iri),
        }
    }

    /// Returns the length of the RDF Node
    pub fn length(&self) -> usize {
        self.node.length()
    }

    /// Returns the numeric value of a node if it is a numeric literal
    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        self.node.numeric_value()
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
