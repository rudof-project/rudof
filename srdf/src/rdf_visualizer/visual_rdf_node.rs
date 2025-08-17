use iri_s::IriS;

use crate::{rdf_visualizer::rdf_visualizer_graph::NodeId, IriOrBlankNode, Object, RDFError, Rdf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFNode {
    Iri { label: String, url: String },
    BlankNode { label: String },
    Literal { value: String },
    Triple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
}

impl VisualRDFNode {
    pub fn as_plantuml(&self, node_id: NodeId) -> String {
        match self {
            VisualRDFNode::Iri { label, url } => {
                format!("rectangle \"{label}\" <<uri>> as {node_id}")
            }
            VisualRDFNode::BlankNode { label } => {
                format!("rectangle \" \" <<bnode>> as {node_id}")
            }
            VisualRDFNode::Literal { value } => {
                format!("rectangle \"{value}\" <<literal>> as {node_id}")
            }
            VisualRDFNode::Triple(visual_rdfnode, visual_rdfnode1, visual_rdfnode2) => todo!(),
        }
    }

    pub fn from_subject<R: Rdf>(rdf: &R, subject: &R::Subject) -> Result<Self, RDFError> {
        let term = R::subject_as_term(subject);
        term_to_visual_node(rdf, &term)
    }

    pub fn from_term<R: Rdf>(rdf: &R, term: &R::Term) -> Result<Self, RDFError> {
        term_to_visual_node(rdf, term)
    }
}

fn term_to_visual_node<R: Rdf>(rdf: &R, term: &R::Term) -> Result<VisualRDFNode, RDFError> {
    let object = R::term_as_object(term)?;
    Ok(object_to_visual_node(rdf, &object))
}

fn subject_to_visual_node<R: Rdf>(rdf: &R, subject: &IriOrBlankNode) -> VisualRDFNode {
    match subject {
        IriOrBlankNode::Iri(iri_s) => {
            let iri: R::IRI = iri_s.clone().into();
            VisualRDFNode::Iri {
                label: rdf.qualify_iri(&iri),
                url: iri_s.as_str().to_string(),
            }
        }
        IriOrBlankNode::BlankNode(bnode) => VisualRDFNode::BlankNode {
            label: format!("{}", bnode),
        },
    }
}

fn predicate_to_visual_node<R: Rdf>(rdf: &R, predicate: &IriS) -> VisualRDFNode {
    let iri: R::IRI = predicate.clone().into();
    let iri_label = rdf.qualify_iri(&iri);
    let iri_str = (*predicate).as_str().to_string();
    VisualRDFNode::Iri {
        label: iri_label,
        url: iri_str,
    }
}

fn object_to_visual_node<R: Rdf>(rdf: &R, object: &Object) -> VisualRDFNode {
    match object {
        Object::Iri(iri_s) => {
            let iri: R::IRI = iri_s.clone().into();
            VisualRDFNode::Iri {
                label: rdf.qualify_iri(&iri),
                url: iri_s.as_str().to_string(),
            }
        }
        Object::BlankNode(bnode) => VisualRDFNode::BlankNode {
            label: format!("{}", bnode),
        },
        Object::Literal(literal) => VisualRDFNode::Literal {
            value: format!("{}", literal),
        },
        Object::Triple {
            subject,
            predicate,
            object,
        } => {
            let s = subject_to_visual_node(rdf, subject);
            let p = predicate_to_visual_node(rdf, predicate);
            let o = object_to_visual_node(rdf, object);
            VisualRDFNode::Triple(Box::new(s), Box::new(p), Box::new(o))
        }
    }
}
