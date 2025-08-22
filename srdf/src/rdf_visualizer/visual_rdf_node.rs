use std::fmt::Display;

use crate::iri::Iri;
use crate::rdf_visualizer::REIFIES;
use crate::{
    IriOrBlankNode, NeighsRDF, Object, RDFError, Rdf,
    rdf_visualizer::{
        rdf_visualizer_error::RdfVisualizerError,
        visual_rdf_graph::{NodeId, VisualRDFGraph},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFNode {
    Reifies,
    Iri { label: String, url: String },
    BlankNode { label: String },
    Literal { value: String },
    Predicate { label: String, url: String },
    NonAssertedTriple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
    AssertedTriple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
}

impl VisualRDFNode {
    pub fn from_predicate<R: Rdf>(rdf: &R, predicate: &R::IRI) -> VisualRDFNode {
        if predicate.as_str() == REIFIES {
            VisualRDFNode::Reifies
        } else {
            let iri_label = rdf.qualify_iri(predicate);
            let iri_str = predicate.to_string();
            VisualRDFNode::Predicate {
                label: iri_label,
                url: iri_str,
            }
        }
    }

    pub fn from_subject<R: NeighsRDF>(
        rdf: &R,
        subject: &R::Subject,
        graph: &mut VisualRDFGraph,
    ) -> Result<Self, RDFError> {
        let term = R::subject_as_term(subject);
        term_to_visual_node(rdf, &term, graph)
    }

    pub fn from_term<R: NeighsRDF>(
        rdf: &R,
        term: &R::Term,
        graph: &mut VisualRDFGraph,
    ) -> Result<Self, RDFError> {
        term_to_visual_node(rdf, term, graph)
    }

    pub fn non_asserted_triple(s: VisualRDFNode, p: VisualRDFNode, o: VisualRDFNode) -> Self {
        VisualRDFNode::NonAssertedTriple(Box::new(s), Box::new(p), Box::new(o))
    }

    pub fn asserted_triple(s: VisualRDFNode, p: VisualRDFNode, o: VisualRDFNode) -> Self {
        VisualRDFNode::AssertedTriple(Box::new(s), Box::new(p), Box::new(o))
    }

    pub fn as_plantuml(
        &self,
        node_id: NodeId,
        show_if_predicate: bool,
        _graph: &VisualRDFGraph,
    ) -> Result<String, RdfVisualizerError> {
        println!("Converting node {self} with node id {node_id} to plantuml");
        match self {
            VisualRDFNode::Iri { label, url } => Ok(format!(
                "rectangle \"[[{url} {label}]]\" <<uri>> as {node_id}"
            )),
            VisualRDFNode::BlankNode { label: _ } => {
                Ok(format!("rectangle \" \" <<bnode>> as {node_id}"))
            }
            VisualRDFNode::Literal { value } => {
                Ok(format!("rectangle \"{value}\" <<literal>> as {node_id}"))
            }
            VisualRDFNode::NonAssertedTriple(_subj, _pred, _obj) => {
                let mut str = String::new();
                str.push_str(format!("cloud \" \" <<non_asserted>> as {node_id}\n").as_str());
                Ok(str)
            }
            VisualRDFNode::AssertedTriple(_subj, _pred, _obj) => {
                let mut str = String::new();
                str.push_str(format!("rectangle \" \" <<asserted>> as {node_id}\n").as_str());
                Ok(str)
            }
            VisualRDFNode::Predicate { label, url } => {
                if show_if_predicate {
                    Ok(format!(
                        "rectangle \"[[{url} {label}]]\" <<uri>> as {node_id}"
                    ))
                } else {
                    Ok(String::new())
                }
            }
            VisualRDFNode::Reifies => {
                if show_if_predicate {
                    Ok(format!("rectangle \"reifies\" <<reifies>> as {node_id}"))
                } else {
                    Ok("".to_string())
                }
            }
        }
    }
}

fn term_to_visual_node<R: NeighsRDF>(
    rdf: &R,
    term: &R::Term,
    graph: &mut VisualRDFGraph,
) -> Result<VisualRDFNode, RDFError> {
    let object = R::term_as_object(term)?;
    let object_node = object_to_visual_node(rdf, &object, graph)?;
    Ok(object_node)
}

/*
fn subject_to_visual_node<R: NeighsRDF>(
    rdf: &R,
    subject: &IriOrBlankNode,
    in_triple: bool,
) -> VisualRDFNode {
    match subject {
        IriOrBlankNode::Iri(iri_s) => {
            let iri: R::IRI = iri_s.clone().into();
            VisualRDFNode::Iri {
                label: rdf.qualify_iri(&iri),
                url: iri_s.as_str().to_string(),
                in_triple,
            }
        }
        IriOrBlankNode::BlankNode(bnode) => VisualRDFNode::BlankNode {
            label: format!("{}", bnode),
            in_triple,
        },
    }
}

fn predicate_to_visual_node<R: Rdf>(rdf: &R, predicate: &IriS, in_triple: bool) -> VisualRDFNode {
    let iri: R::IRI = predicate.clone().into();
    let iri_label = rdf.qualify_iri(&iri);
    let iri_str = (*predicate).as_str().to_string();
    VisualRDFNode::Iri {
        label: iri_label,
        url: iri_str,
        in_triple,
    }
}*/

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
        }
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
            let sub: IriOrBlankNode = (**subject).clone();
            let s: R::Subject = R::Subject::from(sub);
            let p: R::IRI = R::IRI::from((**predicate).clone());
            let o: R::Term = R::Term::from((**object).clone());
            let triple = graph.create_triple_term(rdf, s, p, o)?;
            Ok(triple)
        }
    }
}

impl Display for VisualRDFNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualRDFNode::Iri { label, url } => write!(f, "Iri: {label} ({url})"),
            VisualRDFNode::BlankNode { label } => {
                write!(f, "BlankNode: {label}")
            }
            VisualRDFNode::Literal { value } => {
                write!(f, "Literal: {value}")
            }
            VisualRDFNode::NonAssertedTriple(_, _, _) => write!(f, "NonAssertedTriple"),
            VisualRDFNode::AssertedTriple(_, _, _) => write!(f, "AssertedTriple"),
            VisualRDFNode::Predicate { label, url } => {
                write!(f, "Predicate: {label} ({url})")
            }
            VisualRDFNode::Reifies => {
                write!(f, "Reifies")
            }
        }
    }
}
