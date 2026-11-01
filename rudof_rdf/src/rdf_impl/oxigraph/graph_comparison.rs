//! Comparison of RDF graphs: isomorphism and subgraph checks.
//!
//! Two RDF graphs are [isomorphic](https://www.w3.org/TR/rdf12-concepts/#graph-isomorphism)
//! when there is a bijection between their blank nodes that maps one onto the
//! other. A graph `G1` is a subgraph of `G2` (up to blank node renaming) when
//! there is an injective mapping from the blank nodes of `G1` to the blank
//! nodes of `G2` that maps every triple of `G1` to a triple of `G2`.
//!
//! RDF 1.2 triple terms are compared structurally: blank nodes nested inside
//! triple terms (at any depth) take part in the mapping like any other blank node.
//!
//! The containment check can be relaxed with a [`SubgraphMode`]:
//! - [`SubgraphMode::Injective`] (the default) is the definition above.
//! - [`SubgraphMode::Homomorphic`] lets several blank nodes of `G1` map to the
//!   same blank node of `G2`.
//! - [`SubgraphMode::Entailment`] lets blank nodes of `G1` map to any term of
//!   `G2` (IRI, literal, blank node or triple term): `G2`
//!   [simply entails](https://www.w3.org/TR/rdf12-semantics/#simpleentailment) `G1`.

use oxrdf::dataset::CanonicalizationAlgorithm;
use oxrdf::{BlankNodeRef, Graph, NamedOrBlankNodeRef, TermRef, TripleRef};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The relation between two RDF graphs, `first` and `second`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphRelation {
    /// Both graphs are isomorphic: equal up to blank node renaming.
    Isomorphic,
    /// Each graph is contained in the other but they are not isomorphic. Only
    /// possible with [`SubgraphMode::Homomorphic`] and [`SubgraphMode::Entailment`]
    /// (e.g. a graph and its lean version).
    Equivalent,
    /// `first` is contained in `second` (with the default mode: isomorphic to a
    /// proper subgraph of it), but not the other way round.
    Subgraph,
    /// `second` is contained in `first`, but not the other way round.
    Supergraph,
    /// None of the above.
    Different,
}

impl GraphRelation {
    /// The relation seen from the other side: `compare(b, a) == compare(a, b).inverse()`.
    pub fn inverse(self) -> Self {
        match self {
            GraphRelation::Subgraph => GraphRelation::Supergraph,
            GraphRelation::Supergraph => GraphRelation::Subgraph,
            other => other,
        }
    }
}

impl Display for GraphRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GraphRelation::Isomorphic => "isomorphic",
            GraphRelation::Equivalent => "equivalent",
            GraphRelation::Subgraph => "subgraph",
            GraphRelation::Supergraph => "supergraph",
            GraphRelation::Different => "different",
        };
        write!(f, "{s}")
    }
}

/// How the blank nodes of the contained graph may be mapped when checking
/// whether one graph is contained in another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubgraphMode {
    /// Blank nodes map one-to-one to blank nodes: the first graph is
    /// isomorphic to a subgraph of the second.
    #[default]
    Injective,
    /// Blank nodes map to blank nodes, several possibly to the same one.
    Homomorphic,
    /// Blank nodes map to any term, several possibly to the same one: the
    /// second graph simply entails the first.
    Entailment,
}

impl Display for SubgraphMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SubgraphMode::Injective => "injective",
            SubgraphMode::Homomorphic => "homomorphic",
            SubgraphMode::Entailment => "entailment",
        };
        write!(f, "{s}")
    }
}

impl FromStr for SubgraphMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "injective" => Ok(SubgraphMode::Injective),
            "homomorphic" => Ok(SubgraphMode::Homomorphic),
            "entailment" => Ok(SubgraphMode::Entailment),
            _ => Err(format!(
                "Unknown subgraph mode '{s}'. Valid modes are: 'injective', 'homomorphic', 'entailment'"
            )),
        }
    }
}

/// Returns `true` if `first` is contained in `second` according to `mode`.
///
/// It is a backtracking search for a mapping of the blank nodes of `first`
/// that maps every triple of `first` to a triple of `second`; `mode` says
/// what the blank nodes may be mapped to. With [`SubgraphMode::Entailment`],
/// this is `second` simply entails `first`.
///
/// <div class="warning">These problems are NP-complete: the worst case is
/// exponential in the number of blank nodes of `first`. Ground triples are
/// checked directly and candidates are pruned, so typical data is fast.</div>
pub fn is_subgraph_with(first: &Graph, second: &Graph, mode: SubgraphMode) -> bool {
    embedding(first, second, mode).is_some()
}

/// A mapping of the blank nodes of `first` that maps every triple of `first`
/// to a triple of `second`, according to `mode`, if there is one.
pub(super) fn embedding<'a, 'b>(
    first: &'a Graph,
    second: &'b Graph,
    mode: SubgraphMode,
) -> Option<HashMap<BlankNodeRef<'a>, TermRef<'b>>> {
    // Only an injective mapping keeps distinct triples distinct
    if mode == SubgraphMode::Injective && first.len() > second.len() {
        return None;
    }
    let mut pending = Vec::new();
    for t in first.iter() {
        if triple_is_ground(t) {
            if !second.contains(t) {
                return None;
            }
        } else {
            pending.push(t);
        }
    }
    if pending.is_empty() {
        return Some(HashMap::new());
    }

    // Candidate images for each pending triple: same shape, same ground terms
    let mut candidates = Vec::with_capacity(pending.len());
    for t in &pending {
        let cs: Vec<TripleRef<'_>> = candidate_triples(*t, second)
            .into_iter()
            .filter(|c| unify_triple(*t, *c, &mut Mapping::new(mode)))
            .collect();
        if cs.is_empty() {
            return None;
        }
        candidates.push(cs);
    }

    let order = search_order(&pending, &candidates);
    let triples: Vec<_> = order.iter().map(|&i| pending[i]).collect();
    let candidates: Vec<_> = order.iter().map(|&i| std::mem::take(&mut candidates[i])).collect();
    let mut mapping = Mapping::new(mode);
    // On success, the search leaves the bindings that map every triple
    search(&triples, &candidates, 0, &mut mapping).then_some(mapping.forward)
}

/// Returns the [`GraphRelation`] between `first` and `second`, containment
/// being checked according to `mode`. Isomorphism does not depend on `mode`.
pub fn compare_graphs_with(first: &Graph, second: &Graph, mode: SubgraphMode) -> GraphRelation {
    if mode == SubgraphMode::Injective {
        return compare_graphs(first, second);
    }
    if is_isomorphic(first, second) {
        return GraphRelation::Isomorphic;
    }
    match (
        is_subgraph_with(first, second, mode),
        is_subgraph_with(second, first, mode),
    ) {
        (true, true) => GraphRelation::Equivalent,
        (true, false) => GraphRelation::Subgraph,
        (false, true) => GraphRelation::Supergraph,
        (false, false) => GraphRelation::Different,
    }
}

/// Returns `true` if both graphs are isomorphic.
///
/// Delegates to Oxigraph: both graphs are canonicalized with
/// [`Graph::canonicalize`] (an RDFC-1.0 based algorithm that also renames
/// blank nodes inside triple terms) and the results compared as sets.
/// Ground graphs skip the canonicalization.
pub fn is_isomorphic(first: &Graph, second: &Graph) -> bool {
    if first.len() != second.len() {
        return false;
    }
    if is_ground(first) && is_ground(second) {
        return first == second;
    }
    let mut first = first.clone();
    let mut second = second.clone();
    first.canonicalize(CanonicalizationAlgorithm::Unstable);
    second.canonicalize(CanonicalizationAlgorithm::Unstable);
    first == second
}

/// Returns `true` if `first` is isomorphic to a subgraph (not necessarily proper) of `second`.
///
/// Oxigraph has no such check: this is [`is_subgraph_with`] with
/// [`SubgraphMode::Injective`].
pub fn is_subgraph(first: &Graph, second: &Graph) -> bool {
    is_subgraph_with(first, second, SubgraphMode::Injective)
}

/// Returns the [`GraphRelation`] between `first` and `second`, with the
/// default [`SubgraphMode::Injective`].
pub fn compare_graphs(first: &Graph, second: &Graph) -> GraphRelation {
    use std::cmp::Ordering::*;
    match first.len().cmp(&second.len()) {
        Equal if is_isomorphic(first, second) => GraphRelation::Isomorphic,
        Less if is_subgraph(first, second) => GraphRelation::Subgraph,
        Greater if is_subgraph(second, first) => GraphRelation::Supergraph,
        _ => GraphRelation::Different,
    }
}

/// The triples of `second` that `t` could be mapped to: same predicate, and
/// same subject or object when it has no blank node, through the most selective index.
pub(super) fn candidate_triples<'b>(t: TripleRef<'_>, second: &'b Graph) -> Vec<TripleRef<'b>> {
    let same_predicate = |c: &TripleRef<'_>| c.predicate == t.predicate;
    if let NamedOrBlankNodeRef::NamedNode(_) = t.subject {
        second.triples_for_subject(t.subject).filter(same_predicate).collect()
    } else if term_is_ground(t.object) {
        second.triples_for_object(t.object).filter(same_predicate).collect()
    } else {
        second.triples_for_predicate(t.predicate).collect()
    }
}

fn is_ground(graph: &Graph) -> bool {
    graph.iter().all(triple_is_ground)
}

pub(super) fn triple_is_ground(t: TripleRef<'_>) -> bool {
    !matches!(t.subject, NamedOrBlankNodeRef::BlankNode(_)) && term_is_ground(t.object)
}

fn term_is_ground(t: TermRef<'_>) -> bool {
    match t {
        TermRef::BlankNode(_) => false,
        TermRef::Triple(t) => triple_is_ground(t.as_ref()),
        _ => true,
    }
}

/// A partial mapping from the blank nodes of the first graph to terms of the
/// second, constrained by a [`SubgraphMode`], with an undo trail for backtracking.
pub(super) struct Mapping<'a, 'b> {
    mode: SubgraphMode,
    pub(super) forward: HashMap<BlankNodeRef<'a>, TermRef<'b>>,
    /// Images already taken, to keep the mapping injective (injective mode only)
    taken: HashSet<TermRef<'b>>,
    pub(super) trail: Vec<BlankNodeRef<'a>>,
}

impl<'a, 'b> Mapping<'a, 'b> {
    pub(super) fn new(mode: SubgraphMode) -> Self {
        Self {
            mode,
            forward: HashMap::new(),
            taken: HashSet::new(),
            trail: Vec::new(),
        }
    }

    pub(super) fn bind(&mut self, from: BlankNodeRef<'a>, to: TermRef<'b>) -> bool {
        if self.mode != SubgraphMode::Entailment && !matches!(to, TermRef::BlankNode(_)) {
            return false;
        }
        if let Some(current) = self.forward.get(&from) {
            return *current == to;
        }
        if self.mode == SubgraphMode::Injective && !self.taken.insert(to) {
            return false;
        }
        self.forward.insert(from, to);
        self.trail.push(from);
        true
    }

    pub(super) fn undo_to(&mut self, len: usize) {
        while self.trail.len() > len {
            let from = self.trail.pop().unwrap();
            let to = self.forward.remove(&from).unwrap();
            self.taken.remove(&to);
        }
    }
}

/// Tries to extend `mapping` so that it maps `t1` to `t2`. On failure the
/// mapping may hold partial bindings: callers undo them.
pub(super) fn unify_triple<'a, 'b>(t1: TripleRef<'a>, t2: TripleRef<'b>, mapping: &mut Mapping<'a, 'b>) -> bool {
    t1.predicate == t2.predicate
        && match t1.subject {
            NamedOrBlankNodeRef::BlankNode(b1) => mapping.bind(b1, t2.subject.into()),
            s1 => s1 == t2.subject,
        }
        && unify_term(t1.object, t2.object, mapping)
}

fn unify_term<'a, 'b>(o1: TermRef<'a>, o2: TermRef<'b>, mapping: &mut Mapping<'a, 'b>) -> bool {
    match (o1, o2) {
        (TermRef::BlankNode(b1), o2) => mapping.bind(b1, o2),
        (TermRef::Triple(t1), TermRef::Triple(t2)) => unify_triple(t1.as_ref(), t2.as_ref(), mapping),
        (o1, o2) => o1 == o2,
    }
}

fn search<'a, 'b>(
    triples: &[TripleRef<'a>],
    candidates: &[Vec<TripleRef<'b>>],
    i: usize,
    mapping: &mut Mapping<'a, 'b>,
) -> bool {
    if i == triples.len() {
        return true;
    }
    let mark = mapping.trail.len();
    for c in &candidates[i] {
        if unify_triple(triples[i], *c, mapping) && search(triples, candidates, i + 1, mapping) {
            return true;
        }
        mapping.undo_to(mark);
    }
    false
}

pub(super) fn blank_nodes<'a>(t: TripleRef<'a>, acc: &mut Vec<BlankNodeRef<'a>>) {
    if let NamedOrBlankNodeRef::BlankNode(b) = t.subject {
        acc.push(b);
    }
    match t.object {
        TermRef::BlankNode(b) => acc.push(b),
        TermRef::Triple(t) => blank_nodes(t.as_ref(), acc),
        _ => {},
    }
}

/// Orders the triples so that each one shares as many blank nodes as
/// possible with the previous ones (so bindings are checked early), breaking
/// ties by the number of candidates.
fn search_order(triples: &[TripleRef<'_>], candidates: &[Vec<TripleRef<'_>>]) -> Vec<usize> {
    let bnodes: Vec<Vec<BlankNodeRef<'_>>> = triples
        .iter()
        .map(|t| {
            let mut acc = Vec::new();
            blank_nodes(*t, &mut acc);
            acc
        })
        .collect();
    let mut bound = HashSet::new();
    let mut remaining: Vec<usize> = (0..triples.len()).collect();
    let mut order = Vec::with_capacity(triples.len());
    while !remaining.is_empty() {
        let (pos, _) = remaining
            .iter()
            .enumerate()
            .max_by_key(|(_, i)| {
                let shared = bnodes[**i].iter().filter(|b| bound.contains(*b)).count();
                (shared, std::cmp::Reverse(candidates[**i].len()))
            })
            .unwrap();
        let i = remaining.swap_remove(pos);
        bound.extend(bnodes[i].iter().copied());
        order.push(i);
    }
    order
}
