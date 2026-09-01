use crate::rdf_core::{
    NeighsRDF, RDFError,
    term::Triple,
    visualizer::{RDFVisualizationConfig, VisualRDFEdge, VisualRDFNode, errors::RdfVisualizerError, utils::UsageCount},
};

use rudof_viz::{
    BoxId, Connector, ConnectorKind, Diagram, DiagramRenderer, DiagramScope, backends::plantuml::PlantUmlBackend,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;

/// A visual representation of an RDF graph that can be converted to a technology-agnostic
/// [`Diagram`] (and, from there, rendered by any `rudof_viz` backend; PlantUML is used directly
/// by [`VisualRDFGraph::as_plantuml`]/[`VisualRDFGraph::as_image`] for convenience).
///
/// This struct maintains mappings between RDF terms and visual nodes, tracks usage
/// counts for different roles (subject, predicate, object), and manages the edges
/// between nodes for visualization purposes.
pub struct VisualRDFGraph {
    /// Counter for generating unique node IDs
    node_counter: usize,
    /// Mapping from visual nodes to their unique IDs
    nodes_map: HashMap<VisualRDFNode, NodeId>,
    /// Usage counts for each node in different contexts
    usage_count: HashMap<VisualRDFNode, UsageCount>,
    /// Set of edges between nodes in the graph
    edges: HashSet<(NodeId, VisualRDFEdge, NodeId)>,
    /// Configuration for visualization styling and behavior
    config: RDFVisualizationConfig,
}

impl VisualRDFGraph {
    /// Creates a new empty visual RDF graph with the given configuration.
    ///
    /// # Arguments
    /// * `config` - Configuration settings for visualization
    ///
    /// # Returns
    /// * A new `VisualRDFGraph` instance
    pub fn new(config: RDFVisualizationConfig) -> Self {
        VisualRDFGraph {
            node_counter: 0,
            nodes_map: HashMap::new(),
            usage_count: HashMap::new(),
            edges: HashSet::new(),
            config,
        }
    }

    /// Creates a visual RDF graph from an RDF data source.
    ///
    /// This method iterates through all triples in the RDF source and creates
    /// corresponding visual nodes and edges.
    ///
    /// # Arguments
    /// * `rdf` - The RDF data source implementing `NeighsRDF`
    /// * `config` - Configuration for visualization
    ///
    /// # Returns
    /// * `Result<Self, RDFError>` - The constructed graph or an error
    pub fn from_rdf<R: NeighsRDF>(rdf: &R, config: RDFVisualizationConfig) -> Result<Self, RDFError> {
        let mut graph = VisualRDFGraph::new(config);
        let triples = rdf
            .triples()
            .map_err(|e| RDFError::ObtainingTriples { error: e.to_string() })?;

        // Reserve capacity based on size hint to reduce reallocations
        if let Some(upper_bound) = triples.size_hint().1 {
            graph.nodes_map.reserve(upper_bound.saturating_mul(3)); // Estimate 3 nodes per triple
            graph.usage_count.reserve(upper_bound.saturating_mul(3));
            graph.edges.reserve(upper_bound);
        }

        for triple in triples {
            let (subject, predicate, object) = triple.into_components();
            graph.create_triple(rdf, subject, predicate, object)?;
        }
        Ok(graph)
    }

    /// Creates a visual representation of an RDF triple in the graph.
    ///
    /// This method converts RDF subject, predicate, and object into visual nodes,
    /// creates edges between them, and updates usage counts.
    ///
    /// # Arguments
    /// * `rdf` - The RDF data source
    /// * `subject` - The subject of the triple
    /// * `predicate` - The predicate of the triple
    /// * `object` - The object of the triple
    ///
    /// # Returns
    /// * `Result<VisualRDFNode, RDFError>` - The created triple node or an error
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

        let edge_node = VisualRDFNode::from_predicate(rdf, &predicate);
        self.increment_usage_count_as_predicate(&edge_node);
        let edge = VisualRDFEdge::from_iri(rdf, &predicate);

        let object_node = VisualRDFNode::from_term(rdf, &object, self)?;
        self.increment_usage_count_as_object(&object_node);
        let object_id = self.get_or_create_node(object_node.clone());
        self.edges.insert((subject_id, edge, object_id));

        Ok(VisualRDFNode::non_asserted_triple(subject_node, edge_node, object_node))
    }

    /// Creates a visual representation of an RDF triple as a term (for RDF-star).
    ///
    /// Similar to `create_triple` but handles triple terms differently,
    /// without creating edges in the visual graph.
    ///
    /// # Arguments
    /// * `rdf` - The RDF data source
    /// * `subject` - The subject of the triple
    /// * `predicate` - The predicate of the triple
    /// * `object` - The object of the triple
    ///
    /// # Returns
    /// * `Result<VisualRDFNode, RDFError>` - The created triple term node or an error
    pub fn create_triple_term<R: NeighsRDF>(
        &mut self,
        rdf: &R,
        subject: R::Subject,
        predicate: R::IRI,
        object: R::Term,
    ) -> Result<VisualRDFNode, RDFError> {
        let subject_node = VisualRDFNode::from_subject(rdf, &subject, self)?;
        self.increment_usage_count_as_subject_in_triple(&subject_node);
        self.get_or_create_node(subject_node.clone());

        let edge_node = VisualRDFNode::from_predicate(rdf, &predicate);
        self.increment_usage_count_as_predicate_in_triple(&edge_node);
        self.get_or_create_node(edge_node.clone());

        let object_node = VisualRDFNode::from_term(rdf, &object, self)?;
        self.increment_usage_count_as_object_in_triple(&object_node);
        self.get_or_create_node(object_node.clone());

        let subject_str = subject.to_string();
        let predicate_str = predicate.to_string();
        let object_str = object.to_string();
        let asserted = rdf
            .contains(&subject, &predicate, &object)
            .map_err(|e| RDFError::FailedCheckingAssertion {
                subject: subject_str.to_string(),
                predicate: predicate_str.to_string(),
                object: object_str.to_string(),
                error: e.to_string(),
            })?;
        let triple = if asserted {
            VisualRDFNode::asserted_triple(subject_node, edge_node, object_node)
        } else {
            VisualRDFNode::non_asserted_triple(subject_node, edge_node, object_node)
        };
        Ok(triple)
    }

    /// Increments the usage count for a node when used as a subject.
    ///
    /// # Arguments
    /// * `node` - The node to increment the count for
    #[inline]
    pub fn increment_usage_count_as_subject(&mut self, node: &VisualRDFNode) {
        let count = self.usage_count.entry(node.clone()).or_default();
        count.increment_as_subject();
    }

    /// Increments the usage count for a node when used as a subject in a triple term.
    ///
    /// # Arguments
    /// * `node` - The node to increment the count for
    #[inline]
    pub fn increment_usage_count_as_subject_in_triple(&mut self, node: &VisualRDFNode) {
        let count = self.usage_count.entry(node.clone()).or_default();
        count.increment_as_subject_in_triple();
    }

    /// Increments the usage count for a node when used as a predicate.
    ///
    /// # Arguments
    /// * `node` - The node to increment the count for
    #[inline]
    pub fn increment_usage_count_as_predicate(&mut self, node: &VisualRDFNode) {
        let count = self.usage_count.entry(node.clone()).or_default();
        count.increment_as_predicate();
    }

    /// Increments the usage count for a node when used as a predicate in a triple term.
    ///
    /// # Arguments
    /// * `node` - The node to increment the count for
    #[inline]
    pub fn increment_usage_count_as_predicate_in_triple(&mut self, node: &VisualRDFNode) {
        let count = self.usage_count.entry(node.clone()).or_default();
        count.increment_as_predicate_in_triple();
    }

    /// Increments the usage count for a node when used as an object.
    ///
    /// # Arguments
    /// * `node` - The node to increment the count for
    #[inline]
    pub fn increment_usage_count_as_object(&mut self, node: &VisualRDFNode) {
        let count = self.usage_count.entry(node.clone()).or_default();
        count.increment_as_object();
    }

    /// Increments the usage count for a node when used as an object in a triple term.
    ///
    /// # Arguments
    /// * `node` - The node to increment the count for
    #[inline]
    pub fn increment_usage_count_as_object_in_triple(&mut self, node: &VisualRDFNode) {
        let count = self.usage_count.entry(node.clone()).or_default();
        count.increment_as_object_in_triple();
    }

    /// Gets the ID of a node, creating it if it doesn't exist.
    ///
    /// # Arguments
    /// * `node` - The node to get or create an ID for
    ///
    /// # Returns
    /// * `NodeId` - The unique ID for the node
    pub fn get_or_create_node(&mut self, node: VisualRDFNode) -> NodeId {
        *self.nodes_map.entry(node).or_insert_with(|| {
            let id = self.node_counter;
            self.node_counter += 1;
            NodeId { id }
        })
    }

    /// Gets the ID of an existing node.
    ///
    /// # Arguments
    /// * `node` - The node to get the ID for
    ///
    /// # Returns
    /// * `Result<NodeId, RdfVisualizerError>` - The node ID or an error if not found
    pub fn get_node_id(&self, node: &VisualRDFNode) -> Result<NodeId, RdfVisualizerError> {
        match self.nodes_map.get(node) {
            Some(id) => Ok(*id),
            None => Err(RdfVisualizerError::NodeNotFound { node: node.clone() }),
        }
    }

    /// Builds the technology-agnostic [`Diagram`] for this graph: one box per visualized node
    /// (see [`Self::show_node`]), one connector per RDF edge, and one connector per
    /// subject/predicate/object role for every triple term.
    pub fn to_diagram(&self) -> Result<Diagram, RdfVisualizerError> {
        let mut diagram = Diagram::new().with_style_sheet(self.config.style_sheet());

        for (node, node_id) in &self.nodes_map {
            if let Some(b) = node.to_diagram_box(*node_id, self.show_node(node)) {
                diagram.add_box(b);
            }
        }

        for (source, edge, target) in &self.edges {
            diagram.add_connector(
                Connector::new(to_box_id(*source), to_box_id(*target), ConnectorKind::Association)
                    .with_label(edge.to_label()),
            );
        }

        for (node, node_id) in &self.nodes_map {
            match node {
                VisualRDFNode::NonAssertedTriple(subj, pred, obj) | VisualRDFNode::AssertedTriple(subj, pred, obj) => {
                    self.add_triple_term_connectors(&mut diagram, node_id, subj, pred, obj)?;
                },
                _ => {},
            }
        }

        Ok(diagram)
    }

    /// Adds the three fan-out connectors (subject/predicate/object) from a triple-term box to
    /// its constituent nodes.
    fn add_triple_term_connectors(
        &self,
        diagram: &mut Diagram,
        triple_id: &NodeId,
        subj: &VisualRDFNode,
        pred: &VisualRDFNode,
        obj: &VisualRDFNode,
    ) -> Result<(), RdfVisualizerError> {
        let subj_id = self.get_node_id(subj)?;
        let pred_id = self.get_node_id(pred)?;
        let obj_id = self.get_node_id(obj)?;
        diagram.add_connector(
            Connector::new(to_box_id(*triple_id), to_box_id(subj_id), ConnectorKind::Association)
                .with_label(self.config.subject_text().clone())
                .with_style(self.config.subject_arrow_style().clone()),
        );
        diagram.add_connector(
            Connector::new(to_box_id(*triple_id), to_box_id(pred_id), ConnectorKind::Association)
                .with_label(self.config.predicate_text().clone())
                .with_style(self.config.predicate_arrow_style().clone()),
        );
        diagram.add_connector(
            Connector::new(to_box_id(*triple_id), to_box_id(obj_id), ConnectorKind::Association)
                .with_label(self.config.object_text().clone())
                .with_style(self.config.object_arrow_style().clone()),
        );
        Ok(())
    }

    /// Renders this graph as PlantUML text.
    ///
    /// `mode` is currently ignored (RDF diagrams are always rendered in full), matching prior
    /// behavior.
    pub fn as_plantuml<W: Write>(&self, writer: &mut W, _mode: &DiagramScope) -> Result<(), RdfVisualizerError> {
        let diagram = self.to_diagram()?;
        PlantUmlBackend::default().render(&diagram, writer)?;
        Ok(())
    }

    /// Renders this graph to an image via `java -jar plantuml.jar`.
    ///
    /// `mode` is currently ignored (RDF diagrams are always rendered in full), matching prior
    /// behavior.
    #[cfg(not(target_family = "wasm"))]
    pub fn as_image<W: Write, P: AsRef<std::path::Path>>(
        &self,
        writer: &mut W,
        image_format: rudof_viz::ImageFormat,
        _mode: &DiagramScope,
        plantuml_path: P,
    ) -> Result<(), RdfVisualizerError> {
        use rudof_viz::ExternalToolRenderer;

        let diagram = self.to_diagram()?;
        PlantUmlBackend::new(plantuml_path.as_ref()).render_image(&diagram, image_format, writer)?;
        Ok(())
    }

    /// Determines whether a node should be shown in the visualization.
    ///
    /// Some nodes (like predicates) are only shown if they appear in triple terms.
    ///
    /// # Arguments
    /// * `node` - The node to check
    ///
    /// # Returns
    /// * `bool` - True if the node should be visualized
    pub fn show_node(&self, node: &VisualRDFNode) -> bool {
        match node {
            VisualRDFNode::Predicate { .. } | VisualRDFNode::Reifies => match self.usage_count.get(node) {
                Some(usage_count) => usage_count.in_triple(),
                None => false,
            },
            // All nodes are visualized by default
            _ => true,
        }
    }
}

fn to_box_id(node_id: NodeId) -> BoxId {
    BoxId::new(node_id.as_usize())
}

/// Unique identifier for nodes in the visual graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct NodeId {
    id: usize,
}

impl NodeId {
    pub fn as_usize(&self) -> usize {
        self.id
    }
}

impl Display for NodeId {
    /// Formats the node ID as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Unique identifier for edges in the visual graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct EdgeId {
    /// The unique numeric identifier
    id: usize,
}

impl Display for EdgeId {
    /// Formats the edge ID as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Display for VisualRDFGraph {
    /// Formats the visual graph for debugging and logging purposes.
    ///
    /// Shows the number of nodes and edges, plus details about each node
    /// and edge in the graph.
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
            write!(f, "\nNode {id}: {node}")?;
            write!(f, "\n     count: {count}")?;
        }
        for (source, edge, target) in &self.edges {
            write!(f, "\nEdge {edge}: {source} --> {target}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rdf_core::RDFFormat;
    use crate::rdf_impl::{OxigraphInMemory, ReaderMode};

    const GRAPH: &str = r#"
        prefix : <http://example.org/>
        prefix foaf: <http://xmlns.com/foaf/0.1/>
        :alice foaf:knows :bob .
        :alice foaf:name "Alice" .
    "#;

    fn sample_graph() -> VisualRDFGraph {
        let rdf = OxigraphInMemory::from_str(GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        VisualRDFGraph::from_rdf(&rdf, RDFVisualizationConfig::default()).unwrap()
    }

    #[test]
    fn to_diagram_has_a_box_per_visualized_node_and_a_connector_per_edge() {
        let graph = sample_graph();
        let diagram = graph.to_diagram().unwrap();

        // :alice, :bob, foaf:knows-object-position-only? no, predicates are hidden unless in a triple term.
        // Visible boxes: :alice (uri), :bob (uri), "Alice" (literal) = 3.
        assert_eq!(diagram.boxes().count(), 3);
        assert_eq!(diagram.connectors().count(), 2);
        assert!(!diagram.style_sheet().is_empty());
    }

    #[test]
    fn as_plantuml_renders_valid_plantuml_with_uri_and_literal_stereotypes() {
        let graph = sample_graph();
        let mut out = Vec::new();
        graph.as_plantuml(&mut out, &DiagramScope::all()).unwrap();
        let text = String::from_utf8(out).unwrap();

        assert!(text.starts_with("@startuml"));
        assert!(text.trim_end().ends_with("@enduml"));
        assert!(text.contains("<<uri>>"));
        assert!(text.contains("<<literal>>"));
        assert!(text.contains("hide stereotype"));
    }
}
