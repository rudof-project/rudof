// Shared core logic for node information
use anyhow::*;
use prefixmap::IriRef;
use shex_ast::ObjectValue;
use srdf::NeighsRDF;
use std::collections::HashMap;
use shapemap::{NodeSelector, ShapeSelector};

use crate::ShapeMapParser;

// Core data structure representing node information
#[derive(Debug, Clone)]
pub struct NodeInfo<S: NeighsRDF> {
    pub subject: S::Subject,
    pub subject_qualified: String,
    pub outgoing: HashMap<S::IRI, Vec<S::Term>>,
    pub incoming: HashMap<S::IRI, Vec<S::Subject>>,
}

// Options for what information to retrieve about a node
#[derive(Debug, Clone)]
pub struct NodeInfoOptions {
    pub show_outgoing: bool,
    pub show_incoming: bool,
}

impl NodeInfoOptions {
    pub fn outgoing() -> Self {
        Self {
            show_outgoing: true,
            show_incoming: false,
        }
    }

    pub fn incoming() -> Self {
        Self {
            show_outgoing: false,
            show_incoming: true,
        }
    }

    pub fn both() -> Self {
        Self {
            show_outgoing: true,
            show_incoming: true,
        }
    }

    pub fn from_mode_str(mode: &str) -> Result<Self> {
        match mode {
            "outgoing" => Ok(Self::outgoing()),
            "incoming" => Ok(Self::incoming()),
            "both" => Ok(Self::both()),
            _ => bail!("Invalid mode: {mode}. Must be 'outgoing', 'incoming', or 'both'"),
        }
    }
}

// Get node information from RDF data
// This is the main entry point for retrieving node information.
// It iterates over all nodes in the selector and collects their information.
pub fn get_node_info<S: NeighsRDF>(
    rdf: &S,
    node_selector: NodeSelector,
    predicates: &[String],
    options: NodeInfoOptions,
) -> Result<Vec<NodeInfo<S>>> {
    let mut results = Vec::new();

    for node in node_selector.iter_node(rdf) {
        let subject = node_to_subject(node, rdf)?;
        let subject_qualified = rdf.qualify_subject(&subject);

        let outgoing = if options.show_outgoing {
            get_outgoing_arcs(rdf, &subject, predicates)?
        } else {
            HashMap::new()
        };

        let incoming = if options.show_incoming {
            get_incoming_arcs(rdf, &subject)?
        } else {
            HashMap::new()
        };

        results.push(NodeInfo {
            subject,
            subject_qualified,
            outgoing,
            incoming,
        });
    }

    Ok(results)
}

// Get outgoing arcs for a subject, optionally filtered by predicates
fn get_outgoing_arcs<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    predicates: &[String],
) -> Result<HashMap<S::IRI, Vec<S::Term>>> {
    if predicates.is_empty() {
        let map = rdf
            .outgoing_arcs(subject.clone())
            .map_err(|e| anyhow!("Error obtaining outgoing arcs of {subject}: {e}"))?;
        
        let map_vec = map.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        Ok(map_vec)
    } else {
        let preds = convert_predicates(predicates, rdf)?;
        let (map, _) = rdf.outgoing_arcs_from_list(subject, &preds)
            .map_err(|e| anyhow!("Error obtaining outgoing arcs of {subject}: {e}"))?;
        let map_vec = map.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        Ok(map_vec)
    }
}

// Get incoming arcs for a subject
fn get_incoming_arcs<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
) -> Result<HashMap<S::IRI, Vec<S::Subject>>> {
    let object: S::Term = subject.clone().into();
    let map = rdf
        .incoming_arcs(object.clone())
        .map_err(|e| anyhow!("Can't get incoming arcs of node {subject}: {e}"))?;
    
    let map_vec = map.into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();
    Ok(map_vec)
}

// Convert an ObjectValue (node) to a Subject
// This handles both full IRIs and prefixed names
pub fn node_to_subject<S>(node: &ObjectValue, rdf: &S) -> Result<S::Subject>
where
    S: NeighsRDF,
{
    match node {
        ObjectValue::IriRef(iri_ref) => {
            let term: S::Term = match iri_ref {
                IriRef::Iri(iri_s) => iri_s.clone().into(),
                IriRef::Prefixed { prefix, local } => {
                    let iri_s = rdf.resolve_prefix_local(prefix, local)?;
                    iri_s.into()
                }
            };
            S::term_as_subject(&term)
                .map_err(|_| anyhow!("node_to_subject: Can't convert term {term} to subject"))
        }
        ObjectValue::Literal(lit) => {
            bail!("Node must be an IRI, but found a literal {lit}")
        }
    }
}

// Convert predicate strings to IRI objects 
// Handles both full IRIs and prefixed names
pub fn convert_predicates<S>(predicates: &[String], rdf: &S) -> Result<Vec<S::IRI>>
where
    S: NeighsRDF,
{
    let mut vs = Vec::new();
    for s in predicates {
        let iri_ref = parse_iri_ref(s)?;
        let iri_s = match iri_ref {
            IriRef::Prefixed { prefix, local } => {
                rdf.resolve_prefix_local(prefix.as_str(), local.as_str())?
            }
            IriRef::Iri(iri) => iri,
        };
        vs.push(iri_s.into())
    }
    Ok(vs)
}

// Parse an IRI reference string
// This uses ShapeMapParser to handle both full IRIs and prefixed names
pub fn parse_iri_ref(iri: &str) -> Result<IriRef> {
    ShapeMapParser::parse_iri_ref(iri)
        .map_err(|e| anyhow!("Failed to parse IRI reference '{iri}': {e}"))
}

// Parses a string representation into a NodeSelector.
pub fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let ns = ShapeMapParser::parse_node_selector(node_str)?;
    Ok(ns)
}

// Creates and returns a new, default, or initial ShapeSelector.
pub fn start() -> ShapeSelector {
    ShapeSelector::start()
}

// Parses a string representation into a ShapeSelector.
pub fn parse_shape_selector(label_str: &str) -> Result<ShapeSelector> {
    let selector = ShapeMapParser::parse_shape_selector(label_str)?;
    Ok(selector)
}