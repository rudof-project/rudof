use crate::{Result, Rudof, errors::DataError, formats::NodeInspectionMode};
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::{NeighsRDF, query::QueryRDF};
use shex_ast::{ShapeMapParser, shapemap::NodeSelector};
use std::{collections::HashMap, fmt::Debug, io};
use termtree::Tree;

pub fn show_node_info<W: io::Write>(
    rudof: &mut Rudof,
    node: &str,
    predicates: Option<&[String]>,
    show_node_mode: Option<&NodeInspectionMode>,
    depth: Option<usize>,
    show_hyperlinks: Option<bool>,
    show_colors: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    let config = NodeDisplayConfig::from_options(predicates, show_node_mode, depth, show_hyperlinks, show_colors);

    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoRdfDataLoaded))?;

    if !data.is_rdf() {
        return Err(Box::new(DataError::NoRdfDataLoaded).into());
    }

    let node_selector = parse_node_selector(node)?;

    let node_infos = collect_node_information(data.unwrap_rdf_mut(), node_selector, &config)?;

    write_node_information_list(&node_infos, data.unwrap_rdf_mut(), writer, &config)?;

    Ok(())
}

/// Collects information for all nodes matching the selector.
///
/// This function:
/// 1. Resolves the node selector to concrete nodes
/// 2. For each node, gathers outgoing and incoming arcs based on configuration
/// 3. Returns a list of `NodeInfo` structures
fn collect_node_information<R>(
    rdf: &R,
    node_selector: NodeSelector,
    config: &NodeDisplayConfig,
) -> Result<Vec<NodeInfo<R>>>
where
    R: NeighsRDF + Debug + QueryRDF,
{
    let nodes = node_selector
        .nodes(rdf)
        .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?;

    nodes.iter().map(|node| build_node_info(rdf, node, config)).collect()
}

/// Builds complete information for a single node.
fn build_node_info<R>(rdf: &R, node: &R::Term, config: &NodeDisplayConfig) -> Result<NodeInfo<R>>
where
    R: NeighsRDF + Debug + QueryRDF,
{
    let subject =
        R::term_as_subject(node).map_err(|e| Box::new(DataError::FailedQualification { error: e.to_string() }))?;

    let subject_qualified = qualify_subject(rdf, &subject, config.show_colors)?;

    let outgoing = if config.mode.show_outgoing() {
        collect_outgoing_arcs_recursive(rdf, &subject, &config.predicates, config.depth)?
    } else {
        HashMap::new()
    };

    let incoming = if config.mode.show_incoming() {
        collect_incoming_arcs_recursive(rdf, &subject, config.depth)?
    } else {
        HashMap::new()
    };

    Ok(NodeInfo {
        subject,
        subject_qualified,
        outgoing,
        incoming,
    })
}

/// Recursively collects outgoing arcs up to the specified depth.
///
/// # Depth Behavior
///
/// * `depth = 1`: Returns only direct neighbors as `OutgoingNeighsNode::Term`
/// * `depth > 1`: Recursively traverses, storing nested relationships in `OutgoingNeighsNode::More`
fn collect_outgoing_arcs_recursive<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    predicates: &[String],
    depth: usize,
) -> Result<HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>> {
    // Base case: depth 1 means no recursion
    if depth == 1 {
        return Ok(collect_outgoing_arcs(rdf, subject, predicates)?
            .into_iter()
            .map(|(predicate, terms)| {
                let nodes = terms
                    .into_iter()
                    .map(|term| OutgoingNeighsNode::Term { term })
                    .collect();
                (predicate, nodes)
            })
            .collect());
    }

    // Recursive case: depth > 1
    let arc_map = if predicates.is_empty() {
        rdf.outgoing_arcs(subject)
            .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?
    } else {
        let predicate_iris = convert_predicate_strings_to_iris(predicates, rdf)?;
        let (map, _not_found) = rdf
            .outgoing_arcs_from_list(subject, &predicate_iris)
            .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?;
        map
    };

    let mut result = HashMap::new();

    for (predicate, objects) in arc_map {
        let mut nodes = Vec::new();

        for object in objects {
            // Attempt to treat the object as a subject for further traversal
            let nested_arcs = match S::term_as_subject(&object) {
                Ok(obj_as_subject) => {
                    // Successfully converted to subject, recurse
                    collect_outgoing_arcs_recursive(rdf, &obj_as_subject, predicates, depth - 1)?
                },
                Err(_) => {
                    // Cannot be a subject (e.g., it's a literal), no further traversal
                    HashMap::new()
                },
            };

            nodes.push(OutgoingNeighsNode::More {
                term: object,
                rest: nested_arcs,
            });
        }

        result.insert(predicate, nodes);
    }

    Ok(result)
}

/// Retrieves outgoing arcs for a subject with optional predicate filtering.
///
/// This is a non-recursive version that returns a simple HashMap of predicates to terms.
fn collect_outgoing_arcs<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    predicates: &[String],
) -> Result<HashMap<S::IRI, Vec<S::Term>>> {
    let arc_map = if predicates.is_empty() {
        rdf.outgoing_arcs(subject)
            .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?
    } else {
        let predicate_iris = convert_predicate_strings_to_iris(predicates, rdf)?;
        let (map, _not_found) = rdf
            .outgoing_arcs_from_list(subject, &predicate_iris)
            .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?;
        map
    };

    Ok(arc_map
        .into_iter()
        .map(|(predicate, objects)| (predicate, objects.into_iter().collect()))
        .collect())
}

/// Recursively collects incoming arcs up to the specified depth.
///
/// # Depth Behavior
///
/// * `depth = 1`: Returns only direct parents as `IncomingNeighsNode::Term`
/// * `depth > 1`: Recursively traverses upward, storing nested relationships
fn collect_incoming_arcs_recursive<S: NeighsRDF>(
    rdf: &S,
    subject: &S::Subject,
    depth: usize,
) -> Result<HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>> {
    // Base case: depth 1 means no recursion
    if depth == 1 {
        let incoming_arcs = collect_incoming_arcs(rdf, subject)?;
        return Ok(incoming_arcs
            .into_iter()
            .map(|(predicate, subjects)| {
                let nodes = subjects
                    .iter()
                    .map(|s| IncomingNeighsNode::Term {
                        term: S::subject_as_term(s),
                    })
                    .collect();
                (predicate, nodes)
            })
            .collect());
    }

    // Recursive case: depth > 1
    let object: S::Term = subject.clone().into();
    let arc_map = rdf
        .incoming_arcs(&object)
        .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?;

    let mut result = HashMap::new();

    for (predicate, subjects) in arc_map {
        let mut nodes = Vec::new();

        for subj in subjects {
            // Recurse on each incoming subject
            let nested_arcs = collect_incoming_arcs_recursive(rdf, &subj, depth - 1)?;
            nodes.push(IncomingNeighsNode::More {
                term: S::subject_as_term(&subj),
                rest: nested_arcs,
            });
        }

        result.insert(predicate, nodes);
    }

    Ok(result)
}

/// Retrieves incoming arcs for a subject (non-recursive).
///
/// Finds all subjects that have this node as an object.
fn collect_incoming_arcs<S: NeighsRDF>(rdf: &S, subject: &S::Subject) -> Result<HashMap<S::IRI, Vec<S::Subject>>> {
    let object: S::Term = subject.clone().into();

    let arc_map = rdf
        .incoming_arcs(&object)
        .map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?;

    Ok(arc_map
        .into_iter()
        .map(|(predicate, subjects)| (predicate, subjects.into_iter().collect()))
        .collect())
}

/// Writes formatted node information for multiple nodes.
fn write_node_information_list<S: NeighsRDF, W: io::Write>(
    node_infos: &[NodeInfo<S>],
    rdf: &S,
    writer: &mut W,
    config: &NodeDisplayConfig,
) -> Result<()> {
    for node_info in node_infos {
        write_node_information(node_info, rdf, writer, config)?;
    }
    Ok(())
}

/// Writes formatted information for a single node.
///
/// Outputs:
/// * Outgoing arcs as a tree (if enabled)
/// * Incoming arcs as a tree (if enabled)
fn write_node_information<S: NeighsRDF, W: io::Write>(
    node_info: &NodeInfo<S>,
    rdf: &S,
    writer: &mut W,
    config: &NodeDisplayConfig,
) -> Result<()> {
    // Display outgoing arcs if requested and present
    if config.mode.show_outgoing() && !node_info.outgoing.is_empty() {
        writeln!(writer, "Outgoing arcs")
            .map_err(|e| Box::new(DataError::FailedIoOperation { error: e.to_string() }))?;

        let mut tree = Tree::new(node_info.subject_qualified.clone()).with_glyphs(create_outgoing_glyphs());

        build_outgoing_tree(&mut tree, &node_info.outgoing, rdf, config)?;

        writeln!(writer, "{}", tree).map_err(|e| Box::new(DataError::FailedIoOperation { error: e.to_string() }))?;
    }

    if config.mode.show_incoming() && !node_info.incoming.is_empty() {
        writeln!(writer, "Incoming arcs")
            .map_err(|e| Box::new(DataError::FailedIoOperation { error: e.to_string() }))?;

        let subject_qualified = qualify_subject(rdf, &node_info.subject, config.show_colors)?;
        let root_label = format!("{}\n▲", subject_qualified);

        let mut tree = Tree::new(root_label).with_glyphs(create_incoming_glyphs());

        build_incoming_tree(&mut tree, &node_info.incoming, rdf, config)?;

        writeln!(writer, "{}", tree).map_err(|e| Box::new(DataError::FailedIoOperation { error: e.to_string() }))?;
    }

    Ok(())
}

/// Recursively builds a tree representation of outgoing arcs.
fn build_outgoing_tree<S: NeighsRDF>(
    tree: &mut Tree<String>,
    outgoing_arcs: &HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>,
    rdf: &S,
    config: &NodeDisplayConfig,
) -> Result<()> {
    let mut predicates: Vec<_> = outgoing_arcs.keys().collect();
    predicates.sort();

    for predicate in predicates {
        let pred_str = qualify_iri(rdf, predicate, config.show_colors);

        if let Some(objects) = outgoing_arcs.get(predicate) {
            for object_node in objects {
                match object_node {
                    OutgoingNeighsNode::Term { term } => {
                        let obj_str = qualify_term(rdf, term, config.show_colors)?;
                        let label = format!("─ {} ─► {}", pred_str, obj_str);
                        tree.leaves.push(Tree::new(label).with_glyphs(create_outgoing_glyphs()));
                    },
                    OutgoingNeighsNode::More { term, rest } => {
                        let obj_str = qualify_term(rdf, term, config.show_colors)?;
                        let label = format!("─ {} ─► {}", pred_str, obj_str);

                        let mut subtree = Tree::new(label).with_glyphs(create_outgoing_glyphs());

                        build_outgoing_tree(&mut subtree, rest, rdf, config)?;
                        tree.leaves.push(subtree);
                    },
                }
            }
        }
    }

    Ok(())
}

/// Recursively builds a tree representation of incoming arcs.
fn build_incoming_tree<S: NeighsRDF>(
    tree: &mut Tree<String>,
    incoming_arcs: &HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>,
    rdf: &S,
    config: &NodeDisplayConfig,
) -> Result<()> {
    let mut predicates: Vec<_> = incoming_arcs.keys().collect();
    predicates.sort();

    for predicate in predicates {
        let pred_str = qualify_iri(rdf, predicate, config.show_colors);

        if let Some(subjects) = incoming_arcs.get(predicate) {
            for subject_node in subjects {
                match subject_node {
                    IncomingNeighsNode::Term { term } => {
                        let subj_str = qualify_term(rdf, term, config.show_colors)?;
                        let label = format!("─ {} ── {}", pred_str, subj_str);
                        tree.leaves.push(Tree::new(label).with_glyphs(create_incoming_glyphs()));
                    },
                    IncomingNeighsNode::More { term, rest } => {
                        let subj_str = qualify_term(rdf, term, config.show_colors)?;
                        let label = format!("─ {} ── {}", pred_str, subj_str);

                        let mut subtree = Tree::new(label).with_glyphs(create_incoming_glyphs());

                        build_incoming_tree(&mut subtree, rest, rdf, config)?;
                        tree.leaves.push(subtree);
                    },
                }
            }
        }
    }

    Ok(())
}

/// Creates glyph configuration for incoming arc trees.
fn create_incoming_glyphs() -> termtree::GlyphPalette {
    termtree::GlyphPalette {
        middle_item: "├",
        last_item: "└",
        item_indent: "──",
        middle_skip: "│",
        last_skip: "",
        skip_indent: "   ",
    }
}

/// Creates glyph configuration for outgoing arc trees.
fn create_outgoing_glyphs() -> termtree::GlyphPalette {
    termtree::GlyphPalette {
        middle_item: "├",
        last_item: "└",
        item_indent: "──",
        middle_skip: "│",
        last_skip: "",
        skip_indent: "   ",
    }
}

/// Normalizes a node string to a format accepted by `ShapeMapParser`.
///
/// Bare absolute IRIs like `http://example.org/Alice` are wrapped in angle brackets
/// to produce `<http://example.org/Alice>`. Prefixed names and already-bracketed IRIs
/// are returned unchanged.
fn normalize_node_str(node_str: &str) -> String {
    let trimmed = node_str.trim();
    let is_bare_iri = !trimmed.starts_with('<') && !trimmed.starts_with('_') && trimmed.contains("://");
    if is_bare_iri {
        format!("<{}>", trimmed)
    } else {
        trimmed.to_string()
    }
}

/// Parses a node selector string into a `NodeSelector` instance.
///
/// Supports various formats:
/// * Full IRIs: `<http://example.org/node>` or `http://example.org/node`
/// * Prefixed names: `ex:node`
/// * Blank nodes: `_:b1`
fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let normalized = normalize_node_str(node_str);
    let node_str = normalized.as_str();
    ShapeMapParser::parse_node_selector(node_str).map_err(|e| {
        Box::new(DataError::FailedNodeSelectorParse {
            node: node_str.to_string(),
            error: e.to_string(),
        })
        .into()
    })
}

/// Parses an IRI reference string into a structured `IriRef`.
fn parse_iri_ref(iri: &str) -> Result<IriRef> {
    ShapeMapParser::parse_iri_ref(iri).map_err(|e| {
        Box::new(DataError::FailedIriRefParse {
            iri: iri.to_string(),
            error: e.to_string(),
        })
        .into()
    })
}

/// Converts predicate strings to IRI objects.
///
/// Handles both prefixed names (e.g., `"rdf:type"`) and full IRIs.
fn convert_predicate_strings_to_iris<S>(predicates: &[String], rdf: &S) -> Result<Vec<S::IRI>>
where
    S: NeighsRDF,
{
    predicates
        .iter()
        .map(|pred_str| {
            let iri_ref = parse_iri_ref(pred_str)?;

            let iri_s = match iri_ref {
                IriRef::Prefixed { prefix, local } => {
                    // Resolve prefix:local to full IRI
                    rdf.resolve_prefix_local(prefix.as_str(), local.as_str()).map_err(|e| {
                        Box::new(DataError::FailedPrefixResolution {
                            prefix: prefix.to_string(),
                            error: e.to_string(),
                        })
                    })?
                },
                IriRef::Iri(iri) => iri,
            };

            Ok(iri_s.into())
        })
        .collect()
}

/// Converts a subject to its qualified string representation.
fn qualify_subject<S: NeighsRDF>(rdf: &S, subject: &S::Subject, show_colors: bool) -> Result<String> {
    if show_colors {
        Ok(rdf.qualify_subject(subject))
    } else {
        let prefixmap = rdf.prefixmap().unwrap_or_default().clone().without_colors();

        let subject_term = S::subject_as_term(subject);
        let node = S::term_as_object(&subject_term)
            .map_err(|e| Box::new(DataError::FailedQualification { error: e.to_string() }))?;

        Ok(node.show_qualified(&prefixmap))
    }
}

/// Converts an IRI to its qualified string representation.
fn qualify_iri<S: NeighsRDF>(rdf: &S, iri: &S::IRI, show_colors: bool) -> String {
    if show_colors {
        rdf.qualify_iri(iri)
    } else {
        let prefixmap = rdf.prefixmap().unwrap_or_default().clone().without_colors();

        let iri_s: IriS = iri.clone().into();
        prefixmap.qualify(&iri_s)
    }
}

/// Converts an RDF term to its qualified string representation.
fn qualify_term<S: NeighsRDF>(rdf: &S, term: &S::Term, show_colors: bool) -> Result<String> {
    if show_colors {
        Ok(rdf.qualify_term(term))
    } else {
        let prefixmap = rdf.prefixmap().unwrap_or_default().clone().without_colors();

        let node =
            S::term_as_object(term).map_err(|e| Box::new(DataError::FailedQualification { error: e.to_string() }))?;

        Ok(node.show_qualified(&prefixmap))
    }
}

/// Configuration for node display operations.
///
/// Encapsulates all display preferences to avoid passing many individual parameters.
#[derive(Debug, Clone)]
struct NodeDisplayConfig {
    predicates: Vec<String>,
    mode: NodeInspectionMode,
    depth: usize,
    _show_hyperlinks: bool,
    show_colors: bool,
}

impl NodeDisplayConfig {
    /// Creates a configuration from optional parameters, applying defaults.
    fn from_options(
        predicates: Option<&[String]>,
        mode: Option<&NodeInspectionMode>,
        depth: Option<usize>,
        show_hyperlinks: Option<bool>,
        show_colors: Option<bool>,
    ) -> Self {
        Self {
            predicates: predicates.map(|p| p.to_vec()).unwrap_or_default(),
            mode: mode.copied().unwrap_or_default(),
            depth: depth.unwrap_or(1),
            _show_hyperlinks: show_hyperlinks.unwrap_or(false),
            show_colors: show_colors.unwrap_or(true),
        }
    }
}

/// Complete information about a node in the RDF graph.
///
/// Contains both the raw subject and its qualified (prefixed) string representation,
/// along with all incoming and outgoing relationships.
#[derive(Debug, Clone)]
pub struct NodeInfo<S: NeighsRDF> {
    /// The RDF subject being described
    pub subject: S::Subject,
    /// Human-readable qualified name (e.g., "ex:Person1")
    pub subject_qualified: String,
    /// Outgoing arcs grouped by predicate
    pub outgoing: HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>,
    /// Incoming arcs grouped by predicate
    pub incoming: HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>,
}

/// Represents a neighboring node in the outgoing direction.
///
/// This enum supports recursive depth traversal by storing either a simple term
/// or a term with additional nested relationships.
#[derive(Debug, Clone)]
pub enum OutgoingNeighsNode<S: NeighsRDF> {
    /// Leaf node with no further traversal
    Term { term: S::Term },
    /// Node with additional outgoing relationships
    More {
        term: S::Term,
        /// Nested relationships keyed by predicate
        rest: HashMap<S::IRI, Vec<OutgoingNeighsNode<S>>>,
    },
}

/// Represents a neighboring node in the incoming direction.
///
/// Similar to `OutgoingNeighsNode` but for incoming arcs (subjects pointing to this node).
#[derive(Debug, Clone)]
pub enum IncomingNeighsNode<S: NeighsRDF> {
    /// Leaf node with no further traversal
    Term { term: S::Term },
    /// Node with additional incoming relationships
    More {
        term: S::Term,
        /// Nested relationships keyed by predicate
        rest: HashMap<S::IRI, Vec<IncomingNeighsNode<S>>>,
    },
}
