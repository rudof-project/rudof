// Shared core logic for node information
use crate::shapemap::NodeSelector;
use crate::{RudofError, ShapeMapParser};
use iri_s::IriS;
use prefixmap::IriRef;
use rdf::rdf_core::{NeighsRDF, query::QueryRDF};
use shex_ast::ObjectValue;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::io::Write;
use termtree::Tree;

// Core data structure representing node information
#[derive(Debug, Clone)]
pub struct NodeInfo<S: NeighsRDF> {
    pub subject: S::Subject,
    pub subject_qualified: String,
    pub outgoing: HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>,
    pub incoming: HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>,
}

// REVIEW - Maybe both OutgoingNeighsNode and IncomingNeighsNode can be unified into a single enum
// since they have the same structure. Usages must be checked first.
#[derive(Debug, Clone)]
pub enum OutgoingNeighsNode<S: NeighsRDF> {
    Term {
        term: S::Term,
    },
    More {
        term: S::Term,
        rest: HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>,
    },
}

#[derive(Debug, Clone)]
pub enum IncomingNeighsNode<S: NeighsRDF> {
    Term {
        term: S::Term,
    },
    More {
        term: S::Term,
        rest: HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>,
    },
}

impl<S: NeighsRDF> NodeInfo<S> {
    pub fn write<W: Write>(&self, rdf: &S, options: &NodeInfoOptions, writer: &mut W) -> Result<(), RudofError> {
        format_node_info(self, rdf, writer, options)
            .map_err(|e| RudofError::NodeInfoFormatError { error: e.to_string() })
    }
}

// Options for what information to retrieve about a node
#[derive(Debug, Clone)]
pub struct NodeInfoOptions {
    pub show_outgoing: bool,
    pub show_incoming: bool,
    pub show_colors: bool,
    pub depth: usize,
}

impl NodeInfoOptions {
    pub fn outgoing() -> Self {
        Self {
            show_outgoing: true,
            show_incoming: false,
            show_colors: true,
            depth: 1,
        }
    }

    pub fn incoming() -> Self {
        Self {
            show_outgoing: false,
            show_incoming: true,
            show_colors: true,
            depth: 1,
        }
    }

    pub fn both() -> Self {
        Self {
            show_outgoing: true,
            show_incoming: true,
            show_colors: true,
            depth: 1,
        }
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
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
pub fn get_node_info<R>(
    rdf: &R,
    node_selector: NodeSelector,
    predicates: &[String],
    options: &NodeInfoOptions,
) -> Result<Vec<NodeInfo<R>>, RudofError>
where
    R: NeighsRDF + Debug + QueryRDF,
{
    let mut results = Vec::new();
    let nodes = node_selector.nodes(rdf).map_err(|e| RudofError::NodeSelectorError {
        node_selector: node_selector.to_string(),
        error: e.to_string(),
    })?;

    for node in nodes.iter() {
        let subject = R::term_as_subject(node).map_err(|e| RudofError::Term2Subject {
            term: node.to_string(),
            error: e.to_string(),
        })?;

        let subject_qualified = qualify_subject(rdf, &subject, options)?;

        let outgoing = if options.show_outgoing {
            get_outgoing_arcs_depth(rdf, &subject, predicates, options.depth)?
        } else {
            HashMap::new()
        };

        let incoming = if options.show_incoming {
            get_incoming_arcs_depth(rdf, &subject, options.depth)?
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
        let map = rdf.outgoing_arcs(subject).map_err(|e| RudofError::OutgoingArcs {
            subject: rdf.qualify_subject(subject),
            error: e.to_string(),
        })?;

        let map_vec = map.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect();
        Ok(map_vec)
    } else {
        let preds = convert_predicates(predicates, rdf)?;
        let (map, _) = rdf
            .outgoing_arcs_from_list(subject, &preds)
            .map_err(|e| RudofError::OutgoingArcs {
                subject: rdf.qualify_subject(subject),
                error: e.to_string(),
            })?;
        let map_vec = map.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect();
        Ok(map_vec)
    }
}

// Get outgoing arcs for a subject, optionally filtered by predicates
fn get_outgoing_arcs_depth<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    predicates: &[String],
    depth: usize,
) -> Result<HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>, RudofError> {
    if depth == 1 {
        let outgoing_1 = get_outgoing_arcs(rdf, subject, predicates)?;
        let result = outgoing_1
            .iter()
            .map(|(k, v)| {
                let nodes: Vec<_> = v.iter().map(|t| OutgoingNeighsNode::Term { term: t.clone() }).collect();
                (k.clone(), nodes)
            })
            .collect();
        Ok(result)
    } else if predicates.is_empty() {
        let map = rdf.outgoing_arcs(subject).map_err(|e| RudofError::OutgoingArcs {
            subject: rdf.qualify_subject(subject),
            error: e.to_string(),
        })?;

        let mut result = HashMap::new();
        for (k, v) in map.into_iter() {
            let mut nodes = Vec::new();
            for obj in v.into_iter() {
                let rest = match S::term_as_subject(&obj) {
                    Ok(s) => get_outgoing_arcs_depth(rdf, &s, predicates, depth - 1)?,
                    Err(_) => HashMap::new(),
                };
                nodes.push(OutgoingNeighsNode::More { term: obj, rest });
            }
            result.insert(k, nodes);
        }
        Ok(result)
    } else {
        let preds = convert_predicates(predicates, rdf)?;
        let (map, _) = rdf
            .outgoing_arcs_from_list(subject, &preds)
            .map_err(|e| RudofError::OutgoingArcs {
                subject: rdf.qualify_subject(subject),
                error: e.to_string(),
            })?;
        let mut result = HashMap::new();
        for (k, v) in map.into_iter() {
            let mut nodes = Vec::new();
            for obj in v.into_iter() {
                let rest = match S::term_as_subject(&obj) {
                    Ok(s) => get_outgoing_arcs_depth(rdf, &s, predicates, depth - 1)?,
                    Err(_) => HashMap::new(),
                };
                nodes.push(OutgoingNeighsNode::More { term: obj, rest });
            }
            result.insert(k, nodes);
        }
        Ok(result)
    }
    /*if predicates.is_empty() {
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
    }*/
}

// Get incoming arcs for a subject
fn get_incoming_arcs<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
) -> Result<HashMap<S::IRI, Vec<S::Subject>>, RudofError> {
    let object: S::Term = subject.clone().into();
    let map = rdf.incoming_arcs(&object).map_err(|e| RudofError::IncomingArcs {
        object: rdf.qualify_term(&object),
        error: e.to_string(),
    })?;

    let map_vec = map.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect();
    Ok(map_vec)
}

// Get incoming arcs for a subject
fn get_incoming_arcs_depth<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    depth: usize,
) -> Result<HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>, RudofError> {
    if depth == 1 {
        let incoming_1 = get_incoming_arcs(rdf, subject)?;
        let result = incoming_1
            .iter()
            .map(|(k, v)| {
                let nodes: Vec<_> = v
                    .iter()
                    .map(|s| IncomingNeighsNode::Term {
                        term: S::subject_as_term(s),
                    })
                    .collect();
                (k.clone(), nodes)
            })
            .collect();
        Ok(result)
    } else {
        let object: S::Term = subject.clone().into();
        let map = rdf.incoming_arcs(&object).map_err(|e| RudofError::IncomingArcs {
            object: rdf.qualify_term(&object),
            error: e.to_string(),
        })?;

        let mut result = HashMap::new();
        for (k, v) in map.into_iter() {
            let mut nodes = Vec::new();
            for subj in v.into_iter() {
                let rest = get_incoming_arcs_depth(rdf, &subj, depth - 1)?;
                nodes.push(IncomingNeighsNode::More {
                    term: S::subject_as_term(&subj),
                    rest,
                });
            }
            result.insert(k, nodes);
        }
        Ok(result)
    }
}

// Convert an ObjectValue (node) to a Subject and checks that it exists in the RDF graph.
pub fn node_to_subject_checked<S>(node: &ObjectValue, rdf: &S) -> Result<S::Subject, RudofError>
where
    S: NeighsRDF,
{
    // Convert node to subject
    let subject = node_to_subject(node, rdf)?;

    // Check if the subject actually exists in the RDF graph
    let mut triples = rdf
        .triples_with_subject(&subject)
        .map_err(|e| RudofError::RdfError { error: e.to_string() })?;

    if triples.next().is_none() {
        // No triples found for this subject → node does not exist
        return Err(RudofError::NodeNotFound {
            node: rdf.qualify_term(&S::subject_as_term(&subject)),
        });
    }

    Ok(subject.clone())
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
                    let iri_s = rdf
                        .resolve_prefix_local(prefix, local)
                        .map_err(|e| RudofError::NodeResolveError {
                            node: iri_ref.to_string(),
                            error: e.to_string(),
                        })?;
                    iri_s.into()
                },
            };
            S::term_as_subject(&term).map_err(|_| RudofError::NodeNotSubject {
                node: rdf.qualify_term(&term),
            })
        },
        ObjectValue::Literal(lit) => Err(RudofError::LiteralNotSubject { node: lit.to_string() }),
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
        let iri_s =
            match iri_ref {
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

// Format a single node's information to a writer
pub fn format_node_info<S: NeighsRDF, W: Write>(
    node_info: &NodeInfo<S>,
    rdf: &S,
    writer: &mut W,
    options: &NodeInfoOptions,
) -> Result<(), RudofError> {
    if options.show_outgoing && !node_info.outgoing.is_empty() {
        writeln!(writer, "Outgoing arcs")?;
        let mut outgoing_tree = Tree::new(node_info.subject_qualified.to_string()).with_glyphs(outgoing_glyphs());
        mk_outgoing_tree(&mut outgoing_tree, &node_info.outgoing, rdf, options)?;
        writeln!(writer, "{}", outgoing_tree)?;
    }

    if options.show_incoming && !node_info.incoming.is_empty() {
        writeln!(writer, "Incoming arcs")?;
        let subject: S::Subject = node_info.subject.clone();
        let subject_str = qualify_subject(rdf, &subject, options)?;
        let node_str = format!("{}\n▲", subject_str);
        let mut incoming_tree = Tree::new(node_str).with_glyphs(incoming_glyphs());

        mk_incoming_tree(&mut incoming_tree, &node_info.incoming, rdf, options)?;

        writeln!(writer, "{}", incoming_tree)?;
    }
    Ok(())
}

fn mk_outgoing_tree<S: NeighsRDF>(
    outgoing_tree: &mut Tree<String>,
    outgoing_neighs: &HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>,
    rdf: &S,
    options: &NodeInfoOptions,
) -> Result<(), RudofError> {
    let mut preds: Vec<_> = outgoing_neighs.keys().collect();
    preds.sort();
    for pred in outgoing_neighs.keys() {
        let pred_str = qualify_iri(rdf, pred, options);
        if let Some(objs) = outgoing_neighs.get(pred) {
            for o in objs {
                match o {
                    OutgoingNeighsNode::Term { term } => {
                        let obj_str = qualify_object(rdf, term, options)?;
                        outgoing_tree
                            .leaves
                            .push(Tree::new(format!("─ {} ─► {}", pred_str, obj_str)).with_glyphs(outgoing_glyphs()));
                    },
                    OutgoingNeighsNode::More { term, rest } => {
                        let subj_str = qualify_object(rdf, term, options)?;
                        let origin_str = format!("─ {} ─► {}", pred_str, subj_str);
                        let mut sub_tree = Tree::new(origin_str).with_glyphs(outgoing_glyphs());
                        mk_outgoing_tree(&mut sub_tree, rest, rdf, options)?;
                        outgoing_tree.leaves.push(sub_tree);
                    },
                }
            }
        }
    }
    Ok(())
}

fn mk_incoming_tree<S: NeighsRDF>(
    incoming_tree: &mut Tree<String>,
    incoming_neighs: &HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>,
    rdf: &S,
    options: &NodeInfoOptions,
) -> Result<(), RudofError> {
    let mut preds: Vec<_> = incoming_neighs.keys().collect();
    preds.sort();
    for pred in preds {
        let pred_str = qualify_iri(rdf, pred, options);
        if let Some(subjs) = incoming_neighs.get(pred) {
            for s in subjs {
                match s {
                    IncomingNeighsNode::Term { term } => {
                        let subj_str = qualify_object(rdf, term, options)?;
                        incoming_tree
                            .leaves
                            .push(Tree::new(format!("─ {} ── {}", pred_str, subj_str)).with_glyphs(incoming_glyphs()));
                    },
                    IncomingNeighsNode::More { term, rest } => {
                        let subj_str = qualify_object(rdf, term, options)?;
                        let origin_str = format!("─ {} ── {}", pred_str, subj_str);
                        let mut sub_tree = Tree::new(origin_str).with_glyphs(incoming_glyphs());
                        mk_incoming_tree(&mut sub_tree, rest, rdf, options)?;
                        incoming_tree.leaves.push(sub_tree);
                    },
                }
            }
        }
    }
    Ok(())
}

fn qualify_iri<S: NeighsRDF>(rdf: &S, iri: &S::IRI, options: &NodeInfoOptions) -> String {
    if options.show_colors {
        rdf.qualify_iri(iri)
    } else {
        let prefixmap = rdf.prefixmap().unwrap_or_default().clone();
        let iri_s: IriS = iri.clone().into();
        prefixmap.without_colors().qualify(&iri_s)
    }
}

fn qualify_subject<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    options: &NodeInfoOptions,
) -> Result<String, RudofError> {
    if options.show_colors {
        Ok(rdf.qualify_subject(subject))
    } else {
        let prefixmap = rdf.prefixmap().unwrap_or_default().clone();
        let subject_term: S::Term = S::subject_as_term(subject);
        let node = S::term_as_object(&subject_term).map_err(|e| RudofError::QualifySubject {
            subject: subject.to_string(),
            error: e.to_string(),
        })?;
        Ok(node.show_qualified(&prefixmap.without_colors()))
    }
}

fn qualify_object<S: NeighsRDF>(rdf: &S, object: &S::Term, options: &NodeInfoOptions) -> Result<String, RudofError> {
    if options.show_colors {
        Ok(rdf.qualify_term(object))
    } else {
        let prefixmap = rdf.prefixmap().unwrap_or_default().clone();
        let node = S::term_as_object(object).map_err(|e| RudofError::QualifyObject {
            object: object.to_string(),
            error: e.to_string(),
        })?;
        Ok(node.show_qualified(&prefixmap.without_colors()))
    }
}

// Format multiple node information results
pub fn format_node_info_list<S: NeighsRDF, W: Write>(
    node_infos: &[NodeInfo<S>],
    rdf: &S,
    writer: &mut W,
    options: &NodeInfoOptions,
) -> Result<(), RudofError> {
    for node_info in node_infos {
        format_node_info(node_info, rdf, writer, options)?;
    }
    Ok(())
}

fn outgoing_glyphs() -> termtree::GlyphPalette {
    termtree::GlyphPalette {
        middle_item: "├",
        last_item: "└",
        item_indent: "──",
        middle_skip: "│",
        last_skip: "",
        skip_indent: "   ",
    }
}

fn incoming_glyphs() -> termtree::GlyphPalette {
    termtree::GlyphPalette {
        middle_item: "├",
        last_item: "└",
        item_indent: "──",
        middle_skip: "│",
        last_skip: "",
        skip_indent: "   ",
    }
}

impl<S> Display for OutgoingNeighsNode<S>
where
    S: NeighsRDF,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutgoingNeighsNode::Term { term } => write!(f, "{}", term),
            OutgoingNeighsNode::More { term, rest: _ } => {
                write!(f, "{}", term)
            },
        }
    }
}

impl<S> Display for IncomingNeighsNode<S>
where
    S: NeighsRDF,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomingNeighsNode::Term { term } => write!(f, "{}", term),
            IncomingNeighsNode::More { term, rest: _ } => {
                write!(f, "{}", term)
            },
        }
    }
}
