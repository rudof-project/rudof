use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;

use crate::rdf_visualizer::rdf_visualizer_config::RDFVisualizationConfig;
use crate::rdf_visualizer::rdf_visualizer_error::RdfVisualizerError;
use crate::rdf_visualizer::visual_rdf_node::VisualRDFNode;
use crate::{Iri, NeighsRDF, Object, RDFError};
use crate::{Rdf, Triple};

/// Converts RDF graphs to PlantUML
pub struct RDFVisualizerGraph {
    node_counter: usize,
    nodes_map: HashMap<VisualRDFNode, NodeId>,
    edges: HashSet<(NodeId, VisualRDFEdge, NodeId)>,
}

impl RDFVisualizerGraph {
    pub fn new() -> Self {
        RDFVisualizerGraph {
            node_counter: 0,
            nodes_map: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn from_rdf<R: NeighsRDF>(rdf: &R) -> Result<Self, RDFError> {
        let mut graph = RDFVisualizerGraph::new();
        let triples = rdf.triples().map_err(|e| RDFError::ObtainingTriples {
            error: e.to_string(),
        })?;
        for triple in triples {
            let (subject, predicate, object) = triple.into_components();
            let subject_id = graph.get_or_create_node(VisualRDFNode::from_subject(rdf, &subject)?);
            let edge = convert_to_visual_edge(rdf, &predicate);
            let object_id = graph.get_or_create_node(VisualRDFNode::from_term(rdf, &object)?);
            graph.edges.insert((subject_id, edge, object_id));
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
            writeln!(writer, "{} --> {} : {}\n", source, target, edge.label())?;
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
    pub fn as_plantuml(&self, _edge_id: EdgeId) -> String {
        " ".to_string()
    }

    pub fn label(&self) -> String {
        match self {
            VisualRDFEdge::Iri { label, .. } => label.clone(),
            VisualRDFEdge::Reifies => " ".to_string(),
        }
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
