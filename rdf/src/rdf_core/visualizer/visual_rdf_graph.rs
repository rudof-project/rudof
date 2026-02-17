use crate::rdf_core::{
    NeighsRDF, RDFError,
    term::Triple,
    visualizer::{
        RDFVisualizationConfig, VisualRDFEdge, VisualRDFNode,
        errors::RdfVisualizerError,
        uml_converter::{UmlConverter, UmlGenerationMode, errors::UmlConverterError},
        utils::UsageCount,
    },
};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;

/// A visual representation of an RDF graph that can be converted to PlantUML diagrams.
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

    /// Converts the visual graph to PlantUML format and writes it to the given writer.
    ///
    /// # Arguments
    /// * `writer` - The writer to output the PlantUML code to
    /// * `_mode` - The generation mode (currently unused)
    ///
    /// # Returns
    /// * `Result<(), RdfVisualizerError>` - Ok if successful, Err with details on failure
    pub fn as_plantuml<W: Write>(&self, writer: &mut W, _mode: &UmlGenerationMode) -> Result<(), RdfVisualizerError> {
        let style = self.config.get_style();
        writeln!(writer, "@startuml\n")?;
        writeln!(writer, "{}", style.as_uml())?;

        // Add nodes
        for (node, node_id) in &self.nodes_map {
            let show_node = self.show_node(node);
            let node_uml = node.as_plantuml(*node_id, show_node, self)?;
            writeln!(writer, "{node_uml}\n")?;
        }
        // Add edges
        for (source, edge, target) in &self.edges {
            writeln!(writer, "{source} --> {target} : {}\n", edge.as_plantuml_link())?;
        }

        // Add edges from triples
        for (node, node_id) in &self.nodes_map {
            match node {
                VisualRDFNode::NonAssertedTriple(subj, pred, obj) => {
                    triple_term_as_plantuml(writer, self, node_id, subj, pred, obj)?;
                },
                VisualRDFNode::AssertedTriple(subj, pred, obj) => {
                    triple_term_as_plantuml(writer, self, node_id, subj, pred, obj)?;
                },
                _ => {},
            }
        }

        writeln!(writer, "@enduml\n")?;
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

/// Unique identifier for nodes in the visual graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct NodeId {
    /// The unique numeric identifier
    id: usize,
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

impl UmlConverter for VisualRDFGraph {
    /// Converts the visual graph to PlantUML format.
    ///
    /// This implementation delegates to the struct's own `as_plantuml` method.
    fn as_plantuml<W: Write>(&self, writer: &mut W, mode: &UmlGenerationMode) -> Result<(), UmlConverterError> {
        self.as_plantuml(writer, mode)
            .map_err(|e| UmlConverterError::UmlError { error: e.to_string() })
    }
}

/// Generates PlantUML representation for triple terms (RDF-star triples).
///
/// This function creates the visual connections between a triple node and its
/// constituent subject, predicate, and object nodes.
///
/// # Arguments
/// * `writer` - The writer to output PlantUML code to
/// * `graph` - The visual graph containing the nodes
/// * `triple_id` - ID of the triple node
/// * `subj` - The subject node
/// * `pred` - The predicate node
/// * `obj` - The object node
///
/// # Returns
/// * `Result<(), RdfVisualizerError>` - Ok if successful, Err with details on failure
fn triple_term_as_plantuml<W: Write>(
    writer: &mut W,
    graph: &VisualRDFGraph,
    triple_id: &NodeId,
    subj: &VisualRDFNode,
    pred: &VisualRDFNode,
    obj: &VisualRDFNode,
) -> Result<(), RdfVisualizerError> {
    let subj_id = graph.get_node_id(subj)?;
    let pred_id = graph.get_node_id(pred)?;
    let obj_id = graph.get_node_id(obj)?;
    writeln!(
        writer,
        "{triple_id}-->{subj_id} {} : {} \n",
        graph.config.get_subject_arrow_style().as_plantuml(),
        graph.config.get_subject_text()
    )?;
    writeln!(
        writer,
        "{triple_id}-->{pred_id} {} : {}\n",
        graph.config.get_predicate_arrow_style().as_plantuml(),
        graph.config.get_predicate_text()
    )?;
    writeln!(
        writer,
        "{triple_id}-->{obj_id} {} : {}\n",
        graph.config.get_object_arrow_style().as_plantuml(),
        graph.config.get_object_text()
    )?;
    Ok(())
}
