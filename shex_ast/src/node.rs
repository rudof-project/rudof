use iri_s::IriS;
use prefixmap::IriRef;
use prefixmap::PrefixMapError;
use rbe::Value;
use serde::Serialize;
use srdf::Object;
use srdf::RDFError;
use srdf::SLiteral;
use srdf::numeric_literal::NumericLiteral;
use std::fmt::Display;
use tracing::trace;

use crate::ObjectValue;
use crate::SchemaJsonError;

impl Value for Node {}

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, PartialOrd, Ord)]
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

    /// Returns the inner Object but checks if it is well-formed
    /// This is important in the case of literals like `"hi"^^xsd:integer` which can declare that they are of type integers
    /// but have a lexical form that is not an integer
    /// In that case, this function will return a WrongDatatypeLiteral
    pub fn as_checked_object(&self) -> Result<Object, SchemaJsonError> {
        trace!("as_checked_object: {:?}", self.node);
        match &self.node {
            Object::Literal(sliteral) => {
                let checked_literal =
                    sliteral
                        .as_checked_literal()
                        .map_err(|e| SchemaJsonError::LiteralError {
                            error: e.to_string(),
                        })?;
                Ok(Object::Literal(checked_literal))
            }
            _ => Ok(self.node.clone()),
        }
    }

    pub fn literal(lit: SLiteral) -> Node {
        Node {
            node: Object::literal(lit),
        }
    }

    pub fn datatype(&self) -> Option<IriRef> {
        self.node.datatype()
    }

    pub fn parse(str: &str, base: Option<&str>) -> Result<Node, RDFError> {
        let obj = Object::parse(str, base)?;
        Ok(Node { node: obj })
    }

    pub fn show_qualified(
        &self,
        prefixmap: &prefixmap::PrefixMap,
    ) -> Result<String, PrefixMapError> {
        self.node.show_qualified(prefixmap)
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

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl TryFrom<&Node> for ObjectValue {
    type Error = crate::SchemaJsonError;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        match &node.node {
            srdf::Object::Iri(iri) => Ok(ObjectValue::IriRef(IriRef::iri(iri.clone()))),
            srdf::Object::Literal(lit) => Ok(ObjectValue::Literal(lit.clone())),
            srdf::Object::BlankNode(bnode_id) => {
                Err(crate::SchemaJsonError::InvalidNodeInObjectValue {
                    node: node.to_string(),
                    error: format!("Blank node _:{bnode_id}"),
                })
            }
            srdf::Object::Triple { .. } => Err(SchemaJsonError::InvalidNodeInObjectValue {
                node: node.to_string(),
                error: "RDF triples are not supported in ObjectValue".to_string(),
            }),
        }
    }
}
