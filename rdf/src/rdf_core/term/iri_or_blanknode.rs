use std::fmt::Display;

use iri_s::IriS;
use serde::{Deserialize, Serialize};
use crate::rdf_core::{RDFError, term::Object};

/// Represents an RDF resource that is either an IRI or a blank node.
/// 
/// # Variants
/// - **BlankNode**: An anonymous resource without a global identifier, represented
///   by a local label (e.g., "b0", "genid1")
/// - **Iri**: A globally identified resource using an Internationalized Resource
///   Identifier
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum IriOrBlankNode {
    BlankNode(String),
    Iri(IriS),
}

impl IriOrBlankNode {
    /// Returns the length (in bytes) of this resource's string representation.
    ///
    /// - For blank nodes: the length of the identifier string
    /// - For IRIs: the length of the full IRI string
    pub fn length(&self) -> usize {
        match self {
            IriOrBlankNode::BlankNode(label) => label.len(),
            IriOrBlankNode::Iri(iri) => iri.as_str().len(),
        }
    }

    /// Creates an IRI resource from an `IriS` reference.
    ///
    /// # Parameters
    /// - `iri`: Reference to the IRI to wrap
    pub fn iri(iri: &IriS) -> IriOrBlankNode {
        IriOrBlankNode::Iri(iri.clone())
    }

    /// Formats this resource using qualified names (prefixes) where possible.
    ///
    /// This method produces a compact representation by replacing full IRIs
    /// with prefixed names when a matching prefix exists in the provided map.
    ///
    /// - Blank nodes: formatted as "_:<id>"
    /// - IRIs: qualified using the prefix map (e.g., "ex:resource" instead of
    ///   "http://example.org/resource")
    ///
    /// # Parameters
    /// - `prefixmap`: A prefix map containing IRI-to-prefix mappings
    pub fn show_qualified(&self, prefixmap: &prefixmap::PrefixMap) -> String {
        match self {
            IriOrBlankNode::BlankNode(bnode) => format!("_:{bnode}"),
            IriOrBlankNode::Iri(iri) => prefixmap.qualify(iri),
        }
    }
}

// ============================================================================
// Trait Implementations - Display
// ============================================================================

impl Display for IriOrBlankNode {
    /// Formats the resource for display (human-readable output).
    ///
    /// - Blank nodes: displayed as the identifier string without "_:" prefix
    /// - IRIs: displayed as the full IRI string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriOrBlankNode::BlankNode(b) => write!(f, "{b}"),
            IriOrBlankNode::Iri(iri_s) => write!(f, "{iri_s}"),
        }
    }
}

// =======================================
// Trait Implementations 
// =======================================

/// Converts an `IriOrBlankNode` into an `oxrdf::NamedOrBlankNode`.
///
/// This enables interoperability with the `oxrdf` library, allowing seamless
/// conversion from the custom representation to oxrdf's subject type.
impl From<IriOrBlankNode> for oxrdf::NamedOrBlankNode {
    fn from(value: IriOrBlankNode) -> Self {
        match value {
            IriOrBlankNode::Iri(iri) => oxrdf::NamedNode::new_unchecked(iri.as_str()).into(),
            IriOrBlankNode::BlankNode(bnode) => oxrdf::BlankNode::new_unchecked(bnode).into(),
        }
    }
}

/// Converts an `oxrdf::NamedOrBlankNode` into an `IriOrBlankNode`.
///
/// This enables interoperability with the `oxrdf` library, allowing conversion
/// from oxrdf's subject type into the custom representation.
impl TryFrom<Object> for IriOrBlankNode {
    type Error = RDFError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Iri(iri) => Ok(IriOrBlankNode::Iri(iri)),
            Object::BlankNode(b) => Ok(IriOrBlankNode::BlankNode(b)),
            Object::Literal(l) => Err(RDFError::ExpectedIriOrBlankNodeFoundLiteral {
                literal: l.to_string(),
            }),
            Object::Triple {
                subject,
                predicate,
                object,
            } => Err(RDFError::ExpectedIriOrBlankNodeFoundTriple {
                subject: subject.to_string(),
                predicate: predicate.to_string(),
                object: object.to_string(),
            }),
        }
    }
}

/// Attempts to convert an `Object` into an `IriOrBlankNode`.
///
/// This conversion succeeds only when the object is an IRI or blank node.
/// It fails with an error if the object is a literal or triple, as these
/// cannot be represented as resources.
/// 
/// # Errors
///
/// - `RDFError::ExpectedIriOrBlankNodeFoundLiteral`: When attempting to convert
///   a literal object. Literals cannot be subjects in standard RDF.
/// - `RDFError::ExpectedIriOrBlankNodeFoundTriple`: When attempting to convert
///   an RDF-star triple object in a context where quoted triples are not supported.
impl From<oxrdf::NamedOrBlankNode> for IriOrBlankNode {
    fn from(value: oxrdf::NamedOrBlankNode) -> Self {
        match value {
            oxrdf::NamedOrBlankNode::NamedNode(iri) => IriOrBlankNode::Iri(iri.into()),
            oxrdf::NamedOrBlankNode::BlankNode(bnode) => {
                IriOrBlankNode::BlankNode(bnode.into_string())
            }
        }
    }
}
