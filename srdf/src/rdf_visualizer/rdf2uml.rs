use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;

use crate::rdf_visualizer::rdf_visualizer_config::RDFVisualizationConfig;
use crate::rdf_visualizer::rdf_visualizer_error::RdfVisualizerError;
use crate::{Iri, NeighsRDF, Object, RDFError};
use crate::{Rdf, Triple};

/// Converts RDF graphs to PlantUML
pub struct VisualRDFGraph {
    node_counter: usize,
    nodes_map: HashMap<VisualRDFNode, NodeId>,
    edge_counter: usize,
    edges_map: HashMap<VisualRDFEdge, EdgeId>,

    edges: HashSet<(NodeId, EdgeId, NodeId)>,
}

impl VisualRDFGraph {
    pub fn new() -> Self {
        VisualRDFGraph {
            node_counter: 0,
            nodes_map: HashMap::new(),
            edge_counter: 0,
            edges_map: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn from_rdf<R: NeighsRDF>(rdf: &R) -> Result<Self, RDFError> {
        let mut graph = VisualRDFGraph::new();
        let triples = rdf.triples().map_err(|e| RDFError::ObtainingTriples {
            error: e.to_string(),
        })?;
        for triple in triples {
            let (subject, predicate, object) = triple.into_components();
            let subject_id = graph.get_or_create_node(subject_to_visual_node(rdf, &subject)?);
            let edge_id = graph.get_or_create_edge(convert_to_visual_edge(rdf, &predicate));
            let object_id = graph.get_or_create_node(term_to_visual_node(rdf, &object)?);
            graph.edges.insert((subject_id, edge_id, object_id));
        }
        // Convert RDF data into VisualRDFGraph
        Ok(graph)
    }

    pub fn get_or_create_node(&mut self, node: VisualRDFNode) -> NodeId {
        *self.nodes_map.entry(node).or_insert_with(|| {
            let id = self.node_counter;
            self.node_counter += 1;
            NodeId { id }
        })
    }

    pub fn get_or_create_edge(&mut self, edge: VisualRDFEdge) -> EdgeId {
        *self.edges_map.entry(edge).or_insert_with(|| {
            let id = self.edge_counter;
            self.edge_counter += 1;
            EdgeId { id }
        })
    }

    pub fn as_plantuml<W: Write>(
        &self,
        writer: &mut W,
        config: &RDFVisualizationConfig,
    ) -> Result<(), RdfVisualizerError> {
        writeln!(writer, "@startuml\n")?;
        writeln!(writer, "{}", style())?;

        // Add nodes
        for (node, node_id) in &self.nodes_map {
            writeln!(writer, "{}\n", node.as_plantuml(*node_id))?;
        }
        // Add edges
        for (source, edge, target) in &self.edges {
            let edfe = writeln!(writer, "{} --> {} : {}\n", source, target, edge)?;
        }
        writeln!(writer, "@enduml\n")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct NodeId {
    id: usize,
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct EdgeId {
    id: usize,
}

impl Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFEdge {
    Iri { label: String, url: String },
    Reifies,
}

impl VisualRDFEdge {
    pub fn as_plantuml(&self, edge_id: EdgeId) -> String {
        " ".to_string()
    }
}

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
}

fn subject_to_visual_node<R: Rdf>(
    rdf: &R,
    subject: &R::Subject,
) -> Result<VisualRDFNode, RDFError> {
    let term = R::subject_as_term(subject);
    term_to_visual_node(rdf, &term)
}

fn term_to_visual_node<R: Rdf>(rdf: &R, term: &R::Term) -> Result<VisualRDFNode, RDFError> {
    let object = R::term_as_object(term)?;
    Ok(object_to_visual_node(rdf, &object))
}

fn object_to_visual_node<R: Rdf>(rdf: &R, object: &Object) -> VisualRDFNode {
    match object {
        Object::Iri(iri_s) => {
            let iri: R::IRI = iri_s.clone().into();
            VisualRDFNode::Iri {
                label: format!("{:?}", iri),
                url: rdf.qualify_iri(&iri),
            }
        }
        Object::BlankNode(bnode) => VisualRDFNode::BlankNode {
            label: format!("{:?}", bnode),
        },
        Object::Literal(literal) => VisualRDFNode::Literal {
            value: format!("{:?}", literal),
        },
        Object::Triple { .. } => todo!(),
    }
}

fn convert_to_visual_edge<R: Rdf>(rdf: &R, iri: &R::IRI) -> VisualRDFEdge {
    let iri_label = R::qualify_iri(&rdf, iri);
    let iri_str = (*iri).as_str().to_string();
    VisualRDFEdge::Iri {
        label: iri_label,
        url: iri_str,
    }
}

fn style() -> String {
    r#"<style>
.reifier {
 BackGroundColor Yellow
 LineThickness 1
 LineColor black
}
.literal {
 BackGroundColor Cyan
 LineThickness 1
 LineColor black
}
.uri {
 BackGroundColor White
 LineThickness 1
 LineColor Blue
 RoundCorner 25
}
.bnode {
 BackGroundColor Gray
 LineThickness 1
 LineColor Blue
 RoundCorner 25
}

.asserted {
 BackGroundColor White
 LineThickness 2
 LineColor Black
}
.non_asserted {
 BackGroundColor White
 LineThickness 1
 LineColor Blue
}
</style>

hide stereotype
"#
    .to_string()
}
