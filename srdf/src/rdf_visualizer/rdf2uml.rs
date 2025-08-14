use std::collections::{HashMap, HashSet};

use crate::{NeighsRDF, RDF};
use crate::{Rdf, Triple};

/// Converts RDF graphs to PlantUML
pub struct VisualRDFGraph {
    node_counter: usize,
    nodes_map: HashMap<VisualRDFNode, NodeId>,
    edges_map: HashMap<VisualRDFEdge, EdgeId>,
    edges: HashSet<(NodeId, EdgeId, NodeId)>,
}

impl<R: Rdf> VisualRDFGraph
where
    VisualRDFNode: From<R::Subject>,
{
    pub fn new() -> Self {
        VisualRDFGraph {
            node_counter: 0,
            nodes_map: HashMap::new(),
            edges_map: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn from_rdf<R: NeighsRDF>(rdf: R) -> Result<Self, R::Err> {
        let mut graph = VisualRDFGraph::new();
        for triple in rdf.triples()? {
            let (subject, predicate, object) = triple.into_components();
            let subject_id = graph.get_or_create_node(subject);
            let edge_id = graph.get_or_create_node(predicate);
            let object_id = graph.get_or_create_node(object);

            graph.edges.insert((subject_id, edge_id, object_id));
        }
        // Convert RDF data into VisualRDFGraph
        graph
    }

    pub fn get_or_create_node(&mut self, node: impl Into<VisualRDFNode>) -> NodeId {
        let node_id = node.into();
        *self.nodes_map.entry(node_id).or_insert_with(|| {
            let id = self.node_counter;
            self.node_counter += 1;
            NodeId { id }
        })
    }

    pub fn get_or_create_edge(&mut self, edge: impl Into<VisualRDFEdge>) -> EdgeId {
        let edge_id = edge.into();
        *self.edges_map.entry(edge_id).or_insert_with(|| {
            let id = self.edges_map.len();
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
