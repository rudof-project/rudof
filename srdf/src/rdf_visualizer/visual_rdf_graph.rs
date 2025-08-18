use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;

use tracing::debug;

use crate::rdf_visualizer::rdf_visualizer_config::RDFVisualizationConfig;
use crate::rdf_visualizer::rdf_visualizer_error::RdfVisualizerError;
use crate::rdf_visualizer::usage_count::UsageCount;
use crate::rdf_visualizer::visual_rdf_edge::VisualRDFEdge;
use crate::rdf_visualizer::visual_rdf_node::VisualRDFNode;
use crate::Triple;
use crate::{NeighsRDF, RDFError};

/// Converts RDF graphs to PlantUML
pub struct VisualRDFGraph {
    node_counter: usize,
    nodes_map: HashMap<VisualRDFNode, NodeId>,
    usage_count: HashMap<VisualRDFNode, UsageCount>,
    edges: HashSet<(NodeId, VisualRDFEdge, NodeId)>,
}

impl VisualRDFGraph {
    pub fn new() -> Self {
        VisualRDFGraph {
            node_counter: 0,
            nodes_map: HashMap::new(),
            usage_count: HashMap::new(),
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
            graph.create_triple(rdf, subject, predicate, object)?;
        }
        Ok(graph)
    }

    pub fn create_triple<R: NeighsRDF>(
        &mut self,
        rdf: &R,
        subject: R::Subject,
        predicate: R::IRI,
        object: R::Term,
    ) -> Result<VisualRDFNode, RDFError> {
        let subject_node = VisualRDFNode::from_subject(rdf, &subject, self)?;
        self.increment_usage_count_as_subject(&subject_node);
        let subject_id = self.get_or_create_node(subject_node.clone());

        // TODO: Review if we really need edge_id
        let edge_node = VisualRDFNode::from_predicate(rdf, &predicate);
        self.increment_usage_count_as_predicate(&edge_node);
        let _edge_id = self.get_or_create_node(edge_node.clone());
        let edge = VisualRDFEdge::from_iri(rdf, &predicate);

        let object_node = VisualRDFNode::from_term(rdf, &object, self)?;
        self.increment_usage_count_as_object(&object_node);
        let object_id = self.get_or_create_node(object_node.clone());
        self.edges.insert((subject_id, edge, object_id));
        // TODO: Check if the triple is asserted or not
        Ok(VisualRDFNode::non_asserted_triple(
            subject_node,
            edge_node,
            object_node,
        ))
    }

    pub fn increment_usage_count_as_subject(&mut self, node: &VisualRDFNode) {
        let count = self
            .usage_count
            .entry(node.clone())
            .or_insert(UsageCount::new());
        count.increment_as_subject();
    }

    pub fn increment_usage_count_as_predicate(&mut self, node: &VisualRDFNode) {
        let count = self
            .usage_count
            .entry(node.clone())
            .or_insert(UsageCount::new());
        count.increment_as_predicate();
    }

    pub fn increment_usage_count_as_object(&mut self, node: &VisualRDFNode) {
        let count = self
            .usage_count
            .entry(node.clone())
            .or_insert(UsageCount::new());
        count.increment_as_object();
    }

    pub fn increment_usage_count_in_triple(&mut self, node: &VisualRDFNode) {
        let count = self
            .usage_count
            .entry(node.clone())
            .or_insert(UsageCount::new());
        count.increment_in_triple();
    }

    /*pub fn create_triple2<R: NeighsRDF>(
        &mut self,
        rdf: &R,
        subject: IriOrBlankNode,
        predicate: IriS,
        object: Object,
    ) -> Result<(), RDFError> {
        let subject_node = VisualRDFNode::from_subject(rdf, &subject, self)?;
        let subject_id = self.get_or_create_node(subject_node);
        let edge = VisualRDFEdge::from_iri(rdf, &predicate);
        let object_id = VisualRDFNode::from_term(rdf, &object, self)?;
        let object_id = self.get_or_create_node(object_id);
        self.edges.insert((subject_id, edge, object_id));
        Ok(())
    }*/

    pub fn get_or_create_node(&mut self, node: VisualRDFNode) -> NodeId {
        *self.nodes_map.entry(node).or_insert_with(|| {
            let id = self.node_counter;
            self.node_counter += 1;
            NodeId { id }
        })
    }

    pub fn get_node_id(&self, node: &VisualRDFNode) -> Result<NodeId, RdfVisualizerError> {
        match self.nodes_map.get(node) {
            Some(id) => Ok(*id),
            None => Err(RdfVisualizerError::NodeNotFound { node: node.clone() }),
        }
    }

    pub fn as_plantuml<W: Write>(
        &self,
        writer: &mut W,
        config: &RDFVisualizationConfig,
    ) -> Result<(), RdfVisualizerError> {
        println!("Visual graph: {}", self);
        println!("Starting conversion...");
        writeln!(writer, "@startuml\n")?;
        writeln!(writer, "{}", style())?;

        // Add nodes
        for (node, node_id) in &self.nodes_map {
            let node_uml = node.as_plantuml(*node_id, &self)?;
            debug!("Node {}: {}", node_id, node_uml);
            writeln!(writer, "{}\n", node_uml)?;
        }
        // Add edges
        for (source, edge, target) in &self.edges {
            debug!("Edge {} --> {}: {}", source, target, edge);
            writeln!(writer, "{} --> {} : {}\n", source, target, edge.label())?;
        }

        // Add edges from triples
        for (node, node_id) in &self.nodes_map {
            match node {
                VisualRDFNode::NonAssertedTriple(subj, pred, obj) => {
                    let subj_id = self.get_node_id(subj)?;
                    let pred_id = self.get_node_id(pred)?;
                    let obj_id = self.get_node_id(obj)?;
                    writeln!(writer, "{node_id}-->{subj_id}: subject \n")?;
                    writeln!(writer, "{node_id}-->{pred_id}: predicate \n")?;
                    writeln!(writer, "{node_id}-->{obj_id}: object \n")?;
                }
                _ => {}
            }
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

impl Display for VisualRDFGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VisualRDFGraph with {} nodes and {} edges",
            self.nodes_map.len(),
            self.edges.len()
        )?;
        let zero = UsageCount::new();
        for (node, id) in &self.nodes_map {
            let count = self.usage_count.get(node).unwrap_or(&zero);
            write!(f, "\nNode {}: {}", id, node)?;
            write!(f, "     count: {}", count)?;
        }
        for (source, edge, target) in &self.edges {
            write!(f, "\nEdge {}: {} --> {}", edge, source, target)?;
        }
        Ok(())
    }
}
