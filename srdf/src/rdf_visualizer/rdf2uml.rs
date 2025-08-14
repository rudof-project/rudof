use std::collections::{HashMap, HashSet};

use crate::{NeighsRDF, RDF};
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

    pub fn from_rdf<R: NeighsRDF>(rdf: R) -> Result<Self, R::Err> {
        let mut graph = VisualRDFGraph::new();
        for triple in rdf.triples()? {
            let (subject, predicate, object) = triple.into_components();
            let subject_id = graph.get_or_create_node(subject_to_visual_node(subject));
            let edge_id = graph.get_or_create_edge(convert_to_visual_edge(predicate));
            let object_id = graph.get_or_create_node(term_to_visual_node(object));

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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct NodeId {
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct EdgeId {
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFEdge {
    Iri { label: String, url: String },
    Reifies,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFNode {
    Iri { label: String, url: String },
    BlankNode { label: String },
    Literal { value: String },
    Triple(Box<VisualRDFNode>, Box<VisualRDFNode>, Box<VisualRDFNode>),
}

fn subject_to_visual_node<R: Rdf>(subject: R::Subject) -> Result<VisualRDFNode, R::Err> {
    match R::subject_as_object(&subject) {
        Ok(object) => obj_to_visual_node(object),
        Err(_) => VisualRDFNode::BlankNode {
            label: format!("{:?}", subject),
        },
    }
}

fn term_to_visual_node<R: Rdf>(term: R::Term) -> VisualRDFNode {
    // This is a placeholder implementation. Adjust based on your RDF model
    match term {
        _ => VisualRDFNode::BlankNode {
            label: format!("{:?}", term),
        },
    }
}

fn convert_to_visual_edge<R: Rdf>(term: R::Term) -> VisualRDFEdge {
    // This is a placeholder implementation. Adjust based on your RDF model
    match term {
        _ => VisualRDFEdge::Iri {
            label: format!("{:?}", term),
            url: String::new(),
        },
    }
}
