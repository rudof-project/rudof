//! Fetching the data a shapes graph needs, shape by shape, before validation.
//!
//! This follows the data-access strategy of Corman, Florenzano, Reutter and
//! Savković, *Validating SHACL constraints over a SPARQL endpoint* (ISWC 2019):
//! instead of one request per focus node and constraint, the endpoint is asked
//! a few queries per *shape*, starting from the shapes that have targets and
//! then moving, breadth first, to the shapes they reference. The answers are
//! kept in memory, where the validation itself runs.
//!
//! It differs from the paper's algorithm in two ways:
//!
//! - The queries are restricted (`VALUES`) to the nodes that the validation
//!   can reach: the targets, then the values of their paths, and so on. The
//!   paper's queries range over the whole graph, which is impractical on a
//!   large endpoint such as Wikidata.
//! - The in-memory step is rudof's own validation engine, run over the
//!   fetched data (see [`CachedRdfData`]), instead of the paper's rule
//!   saturation. It implements the same recursive semantics, supports every
//!   constraint component, and gives a full validation report. Any data the
//!   prefetch did not anticipate is fetched on demand, so the result is the
//!   same as without prefetching.

use crate::error::ValidationError;
use crate::ir::{IRComponent, IRSchema, ShapeLabelIdx};
use crate::validator::engine::Engine;
use crate::validator::engine::focus_nodes_ops::FocusNodesOps;
use crate::validator::store::CachedRdfData;
use oxrdf::{NamedNode as OxNamedNode, Term as OxTerm};
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath};
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::debug;

/// Default for the maximum number of rounds: how many shape references deep
/// the prefetch follows from the target shapes. Deeper data is fetched on
/// demand during validation.
pub const DEFAULT_PREFETCH_MAX_ROUNDS: usize = 16;

/// Fills `store`'s cache with the data validating `schema` needs. See the
/// [module documentation](self).
pub(crate) fn prefetch(
    store: &CachedRdfData,
    schema: &IRSchema,
    runner: &dyn Engine<CachedRdfData>,
    max_rounds: usize,
) -> Result<(), ValidationError> {
    let mut frontier: HashMap<ShapeLabelIdx, Vec<OxTerm>> = HashMap::new();
    for (id, shape) in schema.iter_with_targets() {
        let Some(idx) = schema.get_idx(id) else {
            continue;
        };
        let focus_nodes: Vec<OxTerm> = shape.focus_nodes(store, runner).iter().cloned().collect();
        frontier.entry(*idx).or_default().extend(focus_nodes);
    }

    let mut visited: HashSet<(ShapeLabelIdx, OxTerm)> = HashSet::new();
    let mut round = 0;
    while !frontier.is_empty() && round < max_rounds {
        round += 1;
        debug!(
            round,
            shapes = frontier.len(),
            "prefetch: fetching the data of a round of shapes"
        );
        let mut work: VecDeque<(ShapeLabelIdx, Vec<OxTerm>)> = frontier.drain().collect();
        let mut next: HashMap<ShapeLabelIdx, Vec<OxTerm>> = HashMap::new();

        while let Some((idx, nodes)) = work.pop_front() {
            let nodes: Vec<OxTerm> = nodes.into_iter().filter(|n| visited.insert((idx, n.clone()))).collect();
            if nodes.is_empty() {
                continue;
            }
            let shape = schema.get_shape_from_idx_e(&idx)?;
            let is_node_shape = shape.path().is_none();
            let values = match shape.path() {
                Some(path) => path_values(store, &nodes, path)?,
                None => nodes.clone(),
            };

            // The property shapes of a node shape all start from the same
            // nodes: fetch the first step of their paths with one query.
            if is_node_shape {
                first_steps(store, schema, &nodes, shape.property_shapes())?;
            }

            // A shape referenced by a node shape applies to the same nodes,
            // so it is fetched in this round. One referenced by a property
            // shape applies to the values of its path: the next round.
            let mut refer = |referenced: &ShapeLabelIdx| {
                if is_node_shape {
                    work.push_back((*referenced, values.clone()));
                } else {
                    next.entry(*referenced).or_default().extend(values.iter().cloned());
                }
            };
            for ps in shape.property_shapes() {
                refer(ps);
            }
            for component in shape.components() {
                match component {
                    IRComponent::Node(c) => refer(c.shape()),
                    IRComponent::Not(c) => refer(c.shape()),
                    IRComponent::And(c) => c.shapes().iter().for_each(&mut refer),
                    IRComponent::Or(c) => c.shapes().iter().for_each(&mut refer),
                    IRComponent::Xone(c) => c.shapes().iter().for_each(&mut refer),
                    IRComponent::QualifiedValueShape(c) => {
                        refer(c.shape());
                        if c.qualified_value_shapes_disjoint().unwrap_or(false) {
                            c.siblings().iter().for_each(&mut refer);
                        }
                    },
                    IRComponent::Class(_) => class_data(store, &values)?,
                    IRComponent::Equals(c) => outgoing(store, &nodes, c.iri())?,
                    IRComponent::Disjoint(c) => outgoing(store, &nodes, c.iri())?,
                    IRComponent::LessThan(c) => outgoing(store, &nodes, c.iri())?,
                    IRComponent::LessThanOrEquals(c) => outgoing(store, &nodes, c.iri())?,
                    IRComponent::SubsetOf(c) => {
                        path_values(store, &nodes, c.path())?;
                    },
                    _ => {},
                }
            }
            if shape.closed() && is_node_shape {
                store
                    .prefetch_all_outgoing(&nodes)
                    .map_err(ValidationError::new_graph_error::<CachedRdfData>)?;
            }
        }
        frontier = next;
    }
    debug!(rounds = round, queries = store.prefetch_queries(), "prefetch: done");
    Ok(())
}

/// Fetches, with one query per batch of `nodes` and direction, the first
/// predicate step of the paths of `property_shapes`.
fn first_steps(
    store: &CachedRdfData,
    schema: &IRSchema,
    nodes: &[OxTerm],
    property_shapes: &[ShapeLabelIdx],
) -> Result<(), ValidationError> {
    let mut forwards = Vec::new();
    let mut backwards = Vec::new();
    for ps in property_shapes {
        if let Some(path) = schema.get_shape_from_idx_e(ps)?.path() {
            first_step(path, &mut forwards, &mut backwards);
        }
    }
    store
        .prefetch_outgoing(nodes, &forwards)
        .map_err(ValidationError::new_graph_error::<CachedRdfData>)?;
    store
        .prefetch_incoming(nodes, &backwards)
        .map_err(ValidationError::new_graph_error::<CachedRdfData>)
}

/// Collects the predicates `path` can start with, forwards or backwards.
/// Paths that can start with zero steps add nothing: their first step is
/// fetched by `path_values`.
fn first_step(path: &SHACLPath, forwards: &mut Vec<OxNamedNode>, backwards: &mut Vec<OxNamedNode>) {
    match path {
        SHACLPath::Predicate { pred } => forwards.push(pred.clone().into()),
        SHACLPath::Inverse { path } => {
            if let Some(pred) = path.pred() {
                backwards.push(pred.clone().into());
            }
        },
        SHACLPath::Alternative { paths } => paths.iter().for_each(|p| first_step(p, forwards, backwards)),
        SHACLPath::Sequence { paths } => {
            if let Some(first) = paths.first() {
                first_step(first, forwards, backwards);
            }
        },
        SHACLPath::OneOrMore { path } => first_step(path, forwards, backwards),
        SHACLPath::ZeroOrMore { .. } | SHACLPath::ZeroOrOne { .. } => {},
    }
}

/// Fetches the `pred` values of `nodes`.
fn outgoing(store: &CachedRdfData, nodes: &[OxTerm], pred: &rudof_iri::IriS) -> Result<(), ValidationError> {
    store
        .prefetch_outgoing(nodes, &[OxNamedNode::new_unchecked(pred.as_str())])
        .map_err(ValidationError::new_graph_error::<CachedRdfData>)
}

/// Fetches the data an `sh:class` check of `nodes` reads: their `rdf:type`s,
/// and the `rdfs:subClassOf` ancestors of those types.
fn class_data(store: &CachedRdfData, nodes: &[OxTerm]) -> Result<(), ValidationError> {
    let rdf_type: OxNamedNode = RdfVocab::rdf_type().into();
    let subclass_of: OxNamedNode = RdfsVocab::rdfs_subclass_of_str().into();
    let types = step(store, nodes, &rdf_type, false)?;
    let mut seen: HashSet<OxTerm> = types.iter().cloned().collect();
    let mut level = types;
    while !level.is_empty() {
        level = step(store, &level, &subclass_of, false)?
            .into_iter()
            .filter(|t| seen.insert(t.clone()))
            .collect();
    }
    Ok(())
}

/// Fetches the data needed to follow `path` from `nodes`, one batch of queries
/// per step, and returns the nodes it reaches.
fn path_values(store: &CachedRdfData, nodes: &[OxTerm], path: &SHACLPath) -> Result<Vec<OxTerm>, ValidationError> {
    Ok(match path {
        SHACLPath::Predicate { pred } => step(store, nodes, &pred.clone().into(), false)?,
        SHACLPath::Inverse { path } => match path.pred() {
            Some(pred) => step(store, nodes, &pred.clone().into(), true)?,
            // Left to the lookups.
            None => Vec::new(),
        },
        SHACLPath::Alternative { paths } => {
            let mut values = Vec::new();
            for path in paths {
                values.extend(path_values(store, nodes, path)?);
            }
            dedup(values)
        },
        SHACLPath::Sequence { paths } => {
            let mut current = nodes.to_vec();
            for path in paths {
                current = path_values(store, &current, path)?;
            }
            current
        },
        SHACLPath::ZeroOrOne { path } => {
            let mut values = nodes.to_vec();
            values.extend(path_values(store, nodes, path)?);
            dedup(values)
        },
        SHACLPath::ZeroOrMore { path } => closure(store, nodes.to_vec(), nodes.to_vec(), path)?,
        SHACLPath::OneOrMore { path } => {
            let first = path_values(store, nodes, path)?;
            closure(store, first.clone(), first, path)?
        },
    })
}

/// Follows `path` repeatedly from `level` until no new node is reached,
/// adding the nodes reached to `reached`.
fn closure(
    store: &CachedRdfData,
    reached: Vec<OxTerm>,
    mut level: Vec<OxTerm>,
    path: &SHACLPath,
) -> Result<Vec<OxTerm>, ValidationError> {
    let mut seen: HashSet<OxTerm> = reached.iter().cloned().collect();
    let mut reached = reached;
    while !level.is_empty() {
        level = path_values(store, &level, path)?
            .into_iter()
            .filter(|n| seen.insert(n.clone()))
            .collect();
        reached.extend(level.iter().cloned());
    }
    Ok(reached)
}

/// Fetches one predicate step from `nodes` (forwards, or backwards when
/// `inverse`) and returns the nodes it reaches, read back from the cache.
fn step(
    store: &CachedRdfData,
    nodes: &[OxTerm],
    pred: &OxNamedNode,
    inverse: bool,
) -> Result<Vec<OxTerm>, ValidationError> {
    let mut values = Vec::new();
    if inverse {
        store
            .prefetch_incoming(nodes, std::slice::from_ref(pred))
            .map_err(ValidationError::new_graph_error::<CachedRdfData>)?;
        for node in nodes {
            values.extend(store.subjects_for(pred, node)?);
        }
    } else {
        // Literals have no outgoing arcs.
        let subjects: Vec<OxTerm> = nodes
            .iter()
            .filter(|n| <CachedRdfData as Rdf>::term_as_subject(n).is_ok())
            .cloned()
            .collect();
        store
            .prefetch_outgoing(&subjects, std::slice::from_ref(pred))
            .map_err(ValidationError::new_graph_error::<CachedRdfData>)?;
        for node in &subjects {
            values.extend(store.objects_for(node, pred)?);
        }
    }
    Ok(dedup(values))
}

fn dedup(values: Vec<OxTerm>) -> Vec<OxTerm> {
    let mut seen = HashSet::new();
    values.into_iter().filter(|v| seen.insert(v.clone())).collect()
}
