use crate::rdf_core::vocabs::RdfVocab;
use crate::rdf_core::{
    NeighsRDF, RDFError, Rdf,
    term::{Iri, IriOrBlankNode, Object},
    visualizer::{NodeId, VisualRDFGraph},
};
use rudof_viz::{BoxId, DiagramBox, Shape};
use std::fmt::Display;

/// Represents a visual node in an RDF graph for visualization purposes.
/// Each variant corresponds to different types of RDF terms or special constructs
/// used in rendering RDF graphs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFNode {
    /// Special node representing the "reifies" predicate.
    Reifies,
    /// An IRI node with a qualified label and full URL.
    Iri { label: String, url: String },
    /// A blank node with a label.
    BlankNode { label: String },
    /// A literal value node.
    Literal { value: String },
    /// A predicate IRI node with a qualified label and full URL.
    Predicate { label: String, url: String },
    /// A non-asserted triple (subject, predicate, object) as a node.
    NonAssertedTriple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
    /// An asserted triple (subject, predicate, object) as a node.
    AssertedTriple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
}

/// Implementation of methods for `VisualRDFNode`.
impl VisualRDFNode {
    /// Creates a `VisualRDFNode` from a predicate IRI.
    /// If the predicate is the special "reifies" IRI, returns `Reifies`.
    /// Otherwise, returns a `Predicate` with qualified label and URL.
    ///
    /// # Arguments
    /// * `rdf` - The RDF data source
    /// * `predicate` - The predicate IRI
    ///
    /// # Returns
    /// * `VisualRDFNode` - The created visual node
    pub fn from_predicate<R: Rdf>(rdf: &R, predicate: &R::IRI) -> VisualRDFNode {
        if predicate.as_str() == RdfVocab::RDF_REIFIES {
            VisualRDFNode::Reifies
        } else {
            let iri_label = rdf.qualify_iri(predicate);
            let iri_str = predicate.as_str().to_string();
            VisualRDFNode::Predicate {
                label: iri_label,
                url: iri_str,
            }
        }
    }

    /// Creates a `VisualRDFNode` from a subject term.
    /// Converts the subject to a term and delegates to `term_to_visual_node`.
    ///
    /// # Arguments
    /// * `rdf` - The RDF data source
    /// * `subject` - The subject term
    /// * `graph` - The visual graph
    ///
    /// # Returns
    /// * `Result<Self, RDFError>` - The visual node or an error
    pub fn from_subject<R: NeighsRDF>(
        rdf: &R,
        subject: &R::Subject,
        graph: &mut VisualRDFGraph,
    ) -> Result<Self, RDFError> {
        let term = R::subject_as_term(subject);
        term_to_visual_node(rdf, &term, graph)
    }

    /// Creates a `VisualRDFNode` from a general term.
    /// Delegates to `term_to_visual_node`.
    ///
    /// # Arguments
    /// * `rdf` - The RDF data source
    /// * `term` - The term to convert
    /// * `graph` - The visual graph
    ///
    /// # Returns
    /// * `Result<Self, RDFError>` - The visual node or an error
    pub fn from_term<R: NeighsRDF>(rdf: &R, term: &R::Term, graph: &mut VisualRDFGraph) -> Result<Self, RDFError> {
        term_to_visual_node(rdf, term, graph)
    }

    /// Creates a `NonAssertedTriple` node from subject, predicate, and object nodes.
    ///
    /// # Arguments
    /// * `s` - The subject node
    /// * `p` - The predicate node
    /// * `o` - The object node
    ///
    /// # Returns
    /// * `Self` - The non-asserted triple node
    pub fn non_asserted_triple(s: VisualRDFNode, p: VisualRDFNode, o: VisualRDFNode) -> Self {
        VisualRDFNode::NonAssertedTriple(Box::new(s), Box::new(p), Box::new(o))
    }

    /// Creates an `AssertedTriple` node from subject, predicate, and object nodes.
    ///
    /// # Arguments
    /// * `s` - The subject node
    /// * `p` - The predicate node
    /// * `o` - The object node
    ///
    /// # Returns
    /// * `Self` - The asserted triple node
    pub fn asserted_triple(s: VisualRDFNode, p: VisualRDFNode, o: VisualRDFNode) -> Self {
        VisualRDFNode::AssertedTriple(Box::new(s), Box::new(p), Box::new(o))
    }

    /// Converts this node to a [`DiagramBox`] for the technology-agnostic diagram model.
    ///
    /// `show_if_predicate` controls whether predicate/reifies nodes are included: they are only
    /// drawn when they participate in an RDF-star triple term. Returns `None` when the node
    /// should not be drawn at all.
    ///
    /// # Arguments
    /// * `node_id` - The unique ID for the node
    /// * `show_if_predicate` - Whether to include predicate/reifies nodes
    pub fn to_diagram_box(&self, node_id: NodeId, show_if_predicate: bool) -> Option<DiagramBox> {
        let id = BoxId::new(node_id.as_usize());
        match self {
            VisualRDFNode::Iri { label, url } => Some(
                DiagramBox::new(id, Shape::Rectangle, label.clone())
                    .with_href(url.clone())
                    .with_stereotype("uri"),
            ),
            VisualRDFNode::BlankNode { label: _ } => {
                Some(DiagramBox::new(id, Shape::Rectangle, " ").with_stereotype("bnode"))
            },
            VisualRDFNode::Literal { value } => {
                Some(DiagramBox::new(id, Shape::Rectangle, value.clone()).with_stereotype("literal"))
            },
            VisualRDFNode::NonAssertedTriple(_subj, _pred, _obj) => {
                Some(DiagramBox::new(id, Shape::Cloud, " ").with_stereotype("non_asserted"))
            },
            VisualRDFNode::AssertedTriple(_subj, _pred, _obj) => {
                Some(DiagramBox::new(id, Shape::Rectangle, " ").with_stereotype("asserted"))
            },
            VisualRDFNode::Predicate { label, url } => {
                if show_if_predicate {
                    Some(
                        DiagramBox::new(id, Shape::Rectangle, label.clone())
                            .with_href(url.clone())
                            .with_stereotype("uri"),
                    )
                } else {
                    None
                }
            },
            VisualRDFNode::Reifies => {
                if show_if_predicate {
                    Some(DiagramBox::new(id, Shape::Rectangle, "reifies").with_stereotype("reifies"))
                } else {
                    None
                }
            },
        }
    }
}

/// Converts a term to a visual node.
/// First converts the term to an object, then delegates to `object_to_visual_node`.
///
/// # Arguments
/// * `rdf` - The RDF data source
/// * `term` - The term to convert
/// * `graph` - The visual graph
///
/// # Returns
/// * `Result<VisualRDFNode, RDFError>` - The visual node or an error
fn term_to_visual_node<R: NeighsRDF>(
    rdf: &R,
    term: &R::Term,
    graph: &mut VisualRDFGraph,
) -> Result<VisualRDFNode, RDFError> {
    let object = R::term_as_object(term)?;
    let object_node = object_to_visual_node(rdf, &object, graph)?;
    Ok(object_node)
}

/// Converts an RDF object to a visual node.
/// Matches on the object type to create the appropriate `VisualRDFNode` variant.
///
/// # Arguments
/// * `rdf` - The RDF data source
/// * `object` - The RDF object
/// * `graph` - The visual graph
///
/// # Returns
/// * `Result<VisualRDFNode, RDFError>` - The visual node or an error
fn object_to_visual_node<R: NeighsRDF>(
    rdf: &R,
    object: &Object,
    graph: &mut VisualRDFGraph,
) -> Result<VisualRDFNode, RDFError> {
    match object {
        Object::Iri(iri_s) => {
            let iri: R::IRI = iri_s.clone().into();
            Ok(VisualRDFNode::Iri {
                label: rdf.qualify_iri(&iri),
                url: iri_s.as_str().to_string(),
            })
        },
        Object::BlankNode(bnode) => Ok(VisualRDFNode::BlankNode {
            label: bnode.to_string(),
        }),
        Object::Literal(literal) => Ok(VisualRDFNode::Literal {
            value: literal.to_string(),
        }),
        Object::Triple {
            subject,
            predicate,
            object,
        } => {
            // Convert the triple components to appropriate types
            let sub: IriOrBlankNode = (**subject).clone();
            let s: R::Subject = R::Subject::from(sub);
            let p: R::IRI = R::IRI::from(predicate.clone());
            let o: R::Term = R::Term::from((**object).clone());
            // Create a triple term in the graph
            let triple = graph.create_triple_term(rdf, s, p, o)?;
            Ok(triple)
        },
    }
}

/// Implementation of `Display` for `VisualRDFNode`.
/// Provides a human-readable string representation of the node.
impl Display for VisualRDFNode {
    /// Formats the visual node as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualRDFNode::Iri { label, url } => write!(f, "Iri: {label} ({url})"),
            VisualRDFNode::BlankNode { label } => {
                write!(f, "BlankNode: {label}")
            },
            VisualRDFNode::Literal { value } => {
                write!(f, "Literal: {value}")
            },
            VisualRDFNode::NonAssertedTriple(_, _, _) => write!(f, "NonAssertedTriple"),
            VisualRDFNode::AssertedTriple(_, _, _) => write!(f, "AssertedTriple"),
            VisualRDFNode::Predicate { label, url } => {
                write!(f, "Predicate: {label} ({url})")
            },
            VisualRDFNode::Reifies => {
                write!(f, "Reifies")
            },
        }
    }
}
