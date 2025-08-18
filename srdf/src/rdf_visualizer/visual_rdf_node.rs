use std::fmt::Display;

use iri_s::IriS;

use crate::{
    rdf_visualizer::{
        rdf_visualizer_error::RdfVisualizerError,
        visual_rdf_graph::{NodeId, VisualRDFGraph},
    },
    IriOrBlankNode, NeighsRDF, Object, RDFError, Rdf,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFNode {
    Iri { label: String, url: String },
    BlankNode { label: String },
    Literal { value: String },
    Predicate { label: String, url: String },
    NonAssertedTriple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
    AssertedTriple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
}

impl VisualRDFNode {
    pub fn from_predicate<R: Rdf>(rdf: &R, predicate: &R::IRI) -> VisualRDFNode {
        let iri_label = rdf.qualify_iri(&predicate);
        let iri_str = predicate.to_string();
        VisualRDFNode::Predicate {
            label: iri_label,
            url: iri_str,
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

    pub fn as_plantuml(
        &self,
        node_id: NodeId,
        graph: &VisualRDFGraph,
    ) -> Result<String, RdfVisualizerError> {
        println!("Converting node {self} with node id {node_id} to plantuml");
        match self {
            VisualRDFNode::Iri { label, url } => {
                Ok(format!("rectangle \"{label}\" <<uri>> as {node_id}"))
            }
            VisualRDFNode::BlankNode { label } => {
                Ok(format!("rectangle \" \" <<bnode>> as {node_id}"))
            }
            VisualRDFNode::Literal { value } => {
                Ok(format!("rectangle \"{value}\" <<literal>> as {node_id}"))
            }
            VisualRDFNode::NonAssertedTriple(subj, pred, obj) => {
                let mut str = String::new();
                let subj_id = graph.get_node_id(subj)?;
                let pred_id = graph.get_node_id(pred)?;
                let obj_id = graph.get_node_id(obj)?;
                str.push_str(format!("cloud \" \" <<non_asserted>> as {node_id}\n").as_str());
                //str.push_str(format!("{node_id}-->{subj_id}: subject \n").as_str());
                //str.push_str(format!("{node_id}-->{pred_id}: predicate \n").as_str());
                //str.push_str(format!("{node_id}-->{obj_id}: object \n").as_str());
                Ok(str)
            }
            VisualRDFNode::AssertedTriple(visual_rdfnode, visual_rdfnode1, visual_rdfnode2) => {
                todo!()
            }
            // We don't show predicates as indivdual nodes by now...
            VisualRDFNode::Predicate { label, url } => Ok("".to_string()),
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

fn subject_to_visual_node<R: NeighsRDF>(rdf: &R, subject: &IriOrBlankNode) -> VisualRDFNode {
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
            label: format!("{}", bnode),
        }),
        Object::Literal(literal) => Ok(VisualRDFNode::Literal {
            value: format!("{}", literal),
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
            let triple = graph.create_triple(rdf, s, p, o)?;
            Ok(triple)
        }
    }
}

impl Display for VisualRDFNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualRDFNode::Iri { label, url } => write!(f, "Iri: {} ({})", label, url),
            VisualRDFNode::BlankNode { label } => write!(f, "BlankNode: {}", label),
            VisualRDFNode::Literal { value } => write!(f, "Literal: {}", value),
            VisualRDFNode::NonAssertedTriple(_, _, _) => write!(f, "NonAssertedTriple"),
            VisualRDFNode::AssertedTriple(_, _, _) => write!(f, "AssertedTriple"),
            VisualRDFNode::Predicate { label, url } => write!(f, "Predicate: {} ({})", label, url),
        }
    }
}
