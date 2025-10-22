// Shared core logic for node information
use crate::shapemap::NodeSelector;
use crate::{RudofError, ShapeMapParser, format_node_info};
use prefixmap::IriRef;
use shex_ast::ObjectValue;
use srdf::NeighsRDF;
use std::collections::HashMap;
use std::io::Write;

// Core data structure representing node information
#[derive(Debug, Clone)]
pub struct NodeInfo<S: NeighsRDF> {
    pub subject: S::Subject,
    pub subject_qualified: String,
    pub outgoing: HashMap<S::IRI, Vec<S::Term>>,
    pub incoming: HashMap<S::IRI, Vec<S::Subject>>,
}

impl<S: NeighsRDF> NodeInfo<S> {
    pub fn write<W: Write>(
        &self,
        rdf: &S,
        options: &NodeInfoOptions,
        writer: &mut W,
    ) -> Result<(), RudofError> {
        format_node_info(self, rdf, writer, options).map_err(|e| RudofError::NodeInfoFormatError {
            error: e.to_string(),
        })
    }
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

    pub fn from_mode_str(mode: &str) -> Result<Self, RudofError> {
        match mode {
            "outgoing" => Ok(Self::outgoing()),
            "incoming" => Ok(Self::incoming()),
            "both" => Ok(Self::both()),
            _ => Err(RudofError::InvalidMode {
                mode: mode.to_string(),
                expected: "outgoing, incoming, both".to_string(),
            }),
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
    options: &NodeInfoOptions,
) -> Result<Vec<NodeInfo<S>>, RudofError> {
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
) -> Result<HashMap<S::IRI, Vec<S::Term>>, RudofError> {
    if predicates.is_empty() {
        let map = rdf
            .outgoing_arcs(subject.clone())
            .map_err(|e| RudofError::OutgoingArcs {
                subject: rdf.qualify_subject(subject),
                error: e.to_string(),
            })?;

        let map_vec = map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        Ok(map_vec)
    } else {
        let preds = convert_predicates(predicates, rdf)?;
        let (map, _) =
            rdf.outgoing_arcs_from_list(subject, &preds)
                .map_err(|e| RudofError::OutgoingArcs {
                    subject: rdf.qualify_subject(subject),
                    error: e.to_string(),
                })?;
        let map_vec = map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        Ok(map_vec)
    }
}

// Get incoming arcs for a subject
fn get_incoming_arcs<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
) -> Result<HashMap<S::IRI, Vec<S::Subject>>, RudofError> {
    let object: S::Term = subject.clone().into();
    let map = rdf
        .incoming_arcs(object.clone())
        .map_err(|e| RudofError::IncomingArcs {
            object: rdf.qualify_term(&object),
            error: e.to_string(),
        })?;

    let map_vec = map
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();
    Ok(map_vec)
}

// Convert an ObjectValue (node) to a Subject
// This handles both full IRIs and prefixed names
pub fn node_to_subject<S>(node: &ObjectValue, rdf: &S) -> Result<S::Subject, RudofError>
where
    S: NeighsRDF,
{
    match node {
        ObjectValue::IriRef(iri_ref) => {
            let term: S::Term = match iri_ref {
                IriRef::Iri(iri_s) => iri_s.clone().into(),
                IriRef::Prefixed { prefix, local } => {
                    let iri_s = rdf.resolve_prefix_local(prefix, local).map_err(|e| {
                        RudofError::NodeResolveError {
                            node: iri_ref.to_string(),
                            error: e.to_string(),
                        }
                    })?;
                    iri_s.into()
                }
            };
            S::term_as_subject(&term).map_err(|_| RudofError::NodeNotSubject {
                node: rdf.qualify_term(&term),
            })
        }
        ObjectValue::Literal(lit) => Err(RudofError::LiteralNotSubject {
            node: lit.to_string(),
        }),
    }
}

// Convert predicate strings to IRI objects
// Handles both full IRIs and prefixed names
pub fn convert_predicates<S>(predicates: &[String], rdf: &S) -> Result<Vec<S::IRI>, RudofError>
where
    S: NeighsRDF,
{
    let mut vs = Vec::new();
    for s in predicates {
        let iri_ref = parse_iri_ref(s)?;
        let iri_s = match iri_ref {
            IriRef::Prefixed { prefix, local } => rdf
                .resolve_prefix_local(prefix.as_str(), local.as_str())
                .map_err(|e| RudofError::PredicateResolveError {
                    predicate: s.clone(),
                    error: e.to_string(),
                })?,
            IriRef::Iri(iri) => iri,
        };
        vs.push(iri_s.into())
    }
    Ok(vs)
}

// Parse an IRI reference string
// This uses ShapeMapParser to handle both full IRIs and prefixed names
pub fn parse_iri_ref(iri: &str) -> Result<IriRef, RudofError> {
    ShapeMapParser::parse_iri_ref(iri).map_err(|e| RudofError::IriRefParseError {
        iri: iri.to_string(),
        error: e.to_string(),
    })
}
