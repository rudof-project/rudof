//! Differences between two RDF graphs: the triples of each graph that the
//! other one does not have, up to blank node renaming.
//!
//! Triples without blank nodes are compared directly. For the others, the
//! blank nodes of one graph must be matched with terms of the other one:
//!
//! - When one graph is contained in the other ([`GraphRelation::Subgraph`],
//!   [`GraphRelation::Supergraph`], [`GraphRelation::Equivalent`] or
//!   [`GraphRelation::Isomorphic`]), the mapping found by the containment
//!   check is used, and the diff is exact.
//! - Otherwise, the mapping that leaves the fewest unmatched triples is
//!   searched for. That problem is NP-hard, so the search is bounded by
//!   [`DiffOptions::exact_search_limit`]: past it, a heuristic mapping (blank
//!   nodes paired by the shape of their neighbourhood) is used and the diff
//!   is marked as not [`exact`](GraphDiff::exact).
//!
//! Both lists come from one mapping, from a *source* graph into the other one:
//! the triples of the source that the mapping does not map to a triple of the
//! other graph, and the triples of the other graph that no matched triple is
//! mapped to. The source is the contained graph when only one is contained in
//! the other; otherwise, the graph whose (containment or best) mapping leaves
//! fewer unmatched triples in total (the first one on a tie). With [`SubgraphMode::Injective`]
//! the mapping is one-to-one, so the source does not matter.

use super::graph_comparison::{
    GraphRelation, Mapping, SubgraphMode, blank_nodes, candidate_triples, embedding, is_isomorphic, triple_is_ground,
    unify_triple,
};
use oxrdf::{BlankNode, BlankNodeRef, Graph, NamedOrBlankNode, NamedOrBlankNodeRef, Term, TermRef, Triple, TripleRef};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

/// The default for [`DiffOptions::exact_search_limit`].
pub const DEFAULT_EXACT_SEARCH_LIMIT: usize = 100_000;

/// Options of [`diff_graphs`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiffOptions {
    /// Maximum number of steps of the exact search for the best blank node
    /// mapping, in each direction. Past it, a heuristic mapping is used. `0`
    /// always uses the heuristic.
    pub exact_search_limit: usize,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            exact_search_limit: DEFAULT_EXACT_SEARCH_LIMIT,
        }
    }
}

/// The differences between two RDF graphs, `first` and `second`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDiff {
    /// The relation between both graphs, as [`compare_graphs_with`](super::compare_graphs_with) returns it.
    pub relation: GraphRelation,
    /// The mode used to map blank nodes.
    pub mode: SubgraphMode,
    /// `true` if the mapping is proven to be the best one: no other mapping
    /// leaves fewer unmatched triples (with the homomorphic and entailment
    /// modes, fewer unmatched triples of its source graph). Always `true` when
    /// one graph is contained in the other, and for graphs without blank nodes.
    pub exact: bool,
    /// Triples of `first` not matched in `second`, sorted.
    pub only_in_first: Vec<Triple>,
    /// Triples of `second` not matched in `first`, sorted.
    pub only_in_second: Vec<Triple>,
    /// How the blank nodes of `first` used in matched triples map to terms of
    /// `second`, sorted. Empty if the mapping goes from `second` to `first`
    /// (except with [`SubgraphMode::Injective`]).
    pub first_to_second: Vec<(BlankNode, Term)>,
    /// How the blank nodes of `second` used in matched triples map to terms of
    /// `first`, sorted. With [`SubgraphMode::Injective`], the inverse of
    /// `first_to_second`; otherwise empty if the mapping goes from `first` to `second`.
    pub second_to_first: Vec<(BlankNode, Term)>,
}

impl GraphDiff {
    /// `true` if neither graph has unmatched triples.
    ///
    /// With [`SubgraphMode::Injective`], only for isomorphic graphs. With the
    /// other modes the graphs may also be equivalent, or one only contained in
    /// the other: e.g. `_:b :p _:b` is contained in `:a :p :a`, and its mapping
    /// (`_:b` to `:a`) matches every triple of both.
    pub fn is_empty(&self) -> bool {
        self.only_in_first.is_empty() && self.only_in_second.is_empty()
    }
}

/// Serialized with triples in N-Triples (without the final `.`) and the
/// mappings as `[blank node, term]` pairs.
impl Serialize for GraphDiff {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        fn triples(ts: &[Triple]) -> Vec<String> {
            ts.iter().map(ToString::to_string).collect()
        }
        fn pairs(ps: &[(BlankNode, Term)]) -> Vec<[String; 2]> {
            ps.iter().map(|(b, t)| [b.to_string(), t.to_string()]).collect()
        }
        let mut s = serializer.serialize_struct("GraphDiff", 7)?;
        s.serialize_field("relation", &self.relation)?;
        s.serialize_field("mode", &self.mode)?;
        s.serialize_field("exact", &self.exact)?;
        s.serialize_field("only_in_first", &triples(&self.only_in_first))?;
        s.serialize_field("only_in_second", &triples(&self.only_in_second))?;
        s.serialize_field("first_to_second", &pairs(&self.first_to_second))?;
        s.serialize_field("second_to_first", &pairs(&self.second_to_first))?;
        s.end()
    }
}

/// Returns the [`GraphDiff`] between `first` and `second`, blank nodes being
/// mapped according to `mode`.
pub fn diff_graphs(first: &Graph, second: &Graph, mode: SubgraphMode, options: &DiffOptions) -> GraphDiff {
    let limit = options.exact_search_limit;
    let diff = |relation, exact, lists: Lists| GraphDiff {
        relation,
        mode,
        exact,
        only_in_first: lists.only_in_first,
        only_in_second: lists.only_in_second,
        first_to_second: lists.first_to_second,
        second_to_first: lists.second_to_first,
    };
    if is_isomorphic(first, second) {
        let m = embedding(first, second, SubgraphMode::Injective).expect("isomorphic graphs embed");
        return diff(GraphRelation::Isomorphic, true, Lists::injective(first, second, &m));
    }
    let fwd = embedding(first, second, mode);
    let bwd = embedding(second, first, mode);
    let relation = match (fwd.is_some(), bwd.is_some()) {
        (true, true) => GraphRelation::Equivalent,
        (true, false) => GraphRelation::Subgraph,
        (false, true) => GraphRelation::Supergraph,
        (false, false) => GraphRelation::Different,
    };
    if mode == SubgraphMode::Injective {
        let (m, exact) = match (fwd, bwd) {
            (Some(m), _) => (m, true),
            (None, Some(b)) => (invert(&b), true),
            (None, None) => best_mapping(first, second, mode, limit),
        };
        return diff(relation, exact, Lists::injective(first, second, &m));
    }
    match (fwd, bwd) {
        // Equivalent graphs: both mappings are exact, the smaller diff is kept
        (Some(f), Some(b)) => {
            let forward = Lists::from_first(first, second, &f);
            let backward = Lists::from_second(first, second, &b);
            let lists = if backward.size() < forward.size() {
                backward
            } else {
                forward
            };
            diff(relation, true, lists)
        },
        (Some(m), None) => diff(relation, true, Lists::from_first(first, second, &m)),
        (None, Some(m)) => diff(relation, true, Lists::from_second(first, second, &m)),
        (None, None) => {
            let (mf, forward_exact) = best_mapping(first, second, mode, limit);
            let (mb, backward_exact) = best_mapping(second, first, mode, limit);
            let forward = Lists::from_first(first, second, &mf);
            let backward = Lists::from_second(first, second, &mb);
            let lists = if backward.size() < forward.size() {
                backward
            } else {
                forward
            };
            diff(relation, forward_exact && backward_exact, lists)
        },
    }
}

type BlankNodeMap<'a, 'b> = HashMap<BlankNodeRef<'a>, TermRef<'b>>;

/// The inverse of a one-to-one mapping of blank nodes to blank nodes.
fn invert<'a, 'b>(m: &BlankNodeMap<'a, 'b>) -> BlankNodeMap<'b, 'a> {
    m.iter()
        .filter_map(|(from, to)| match to {
            TermRef::BlankNode(to) => Some((*to, TermRef::BlankNode(*from))),
            _ => None,
        })
        .collect()
}

/// The lists of a [`GraphDiff`].
struct Lists {
    only_in_first: Vec<Triple>,
    only_in_second: Vec<Triple>,
    first_to_second: Vec<(BlankNode, Term)>,
    second_to_first: Vec<(BlankNode, Term)>,
}

impl Lists {
    fn size(&self) -> usize {
        self.only_in_first.len() + self.only_in_second.len()
    }

    /// With a mapping from `first` into `second`
    fn from_first(first: &Graph, second: &Graph, m: &BlankNodeMap<'_, '_>) -> Self {
        let (only_in_first, only_in_second, first_to_second) = lists(first, second, m);
        Lists {
            only_in_first,
            only_in_second,
            first_to_second,
            second_to_first: Vec::new(),
        }
    }

    /// With a mapping from `second` into `first`
    fn from_second(first: &Graph, second: &Graph, m: &BlankNodeMap<'_, '_>) -> Self {
        let (only_in_second, only_in_first, second_to_first) = lists(second, first, m);
        Lists {
            only_in_first,
            only_in_second,
            first_to_second: Vec::new(),
            second_to_first,
        }
    }

    /// With a one-to-one mapping from `first` into `second`: both directions are given
    fn injective(first: &Graph, second: &Graph, m: &BlankNodeMap<'_, '_>) -> Self {
        let mut lists = Lists::from_first(first, second, m);
        let mut inverse: Vec<(BlankNode, Term)> = lists
            .first_to_second
            .iter()
            .filter_map(|(b, t)| match t {
                Term::BlankNode(t) => Some((t.clone(), b.clone().into())),
                _ => None,
            })
            .collect();
        inverse.sort_by_cached_key(|(b, _)| b.to_string());
        lists.second_to_first = inverse;
        lists
    }
}

/// With a mapping `m` from `source` into `target`: the triples of `source` it
/// does not map to a triple of `target`, the triples of `target` no matched
/// triple is mapped to, and the part of `m` used by the matched triples, all sorted.
fn lists(
    source: &Graph,
    target: &Graph,
    m: &BlankNodeMap<'_, '_>,
) -> (Vec<Triple>, Vec<Triple>, Vec<(BlankNode, Term)>) {
    let mut only_in_source = Vec::new();
    let mut images = HashSet::new();
    let mut used = HashSet::new();
    for t in source.iter() {
        match image(t, m).filter(|i| target.contains(i)) {
            Some(i) => {
                images.insert(i);
                let mut bnodes = Vec::new();
                blank_nodes(t, &mut bnodes);
                used.extend(bnodes);
            },
            None => only_in_source.push(t.into_owned()),
        }
    }
    let mut only_in_target: Vec<Triple> = target
        .iter()
        .map(TripleRef::into_owned)
        .filter(|t| !images.contains(t))
        .collect();
    only_in_source.sort_by_cached_key(ToString::to_string);
    only_in_target.sort_by_cached_key(ToString::to_string);
    let mut pairs: Vec<(BlankNode, Term)> = used.into_iter().map(|b| (b.into_owned(), m[&b].into_owned())).collect();
    pairs.sort_by_cached_key(|(b, _)| b.to_string());
    (only_in_source, only_in_target, pairs)
}

/// The image of `t` under `m`, if every blank node of `t` is mapped (to a
/// term that is valid where the blank node is).
fn image(t: TripleRef<'_>, m: &BlankNodeMap<'_, '_>) -> Option<Triple> {
    let subject: NamedOrBlankNode = match t.subject {
        NamedOrBlankNodeRef::BlankNode(b) => match m.get(&b)? {
            TermRef::NamedNode(n) => n.into_owned().into(),
            TermRef::BlankNode(b) => b.into_owned().into(),
            _ => return None,
        },
        s => s.into_owned(),
    };
    let object: Term = match t.object {
        TermRef::BlankNode(b) => m.get(&b)?.into_owned(),
        TermRef::Triple(inner) => image(inner.as_ref(), m)?.into(),
        o => o.into_owned(),
    };
    Some(Triple::new(subject, t.predicate.into_owned(), object))
}

fn matches(t: TripleRef<'_>, m: &BlankNodeMap<'_, '_>, to: &Graph) -> bool {
    image(t, m).is_some_and(|i| to.contains(&i))
}

/// The triples of `first` with blank nodes and what they could be mapped to.
struct Problem<'a, 'b> {
    mode: SubgraphMode,
    second: &'b Graph,
    /// Triples of `first` with blank nodes
    pending: Vec<TripleRef<'a>>,
    /// Blank nodes of `first`, most used first
    bnodes: Vec<BlankNodeRef<'a>>,
    /// Indices in `pending` of the triples of each blank node
    triples_of: HashMap<BlankNodeRef<'a>, Vec<usize>>,
}

impl<'a, 'b> Problem<'a, 'b> {
    fn new(first: &'a Graph, second: &'b Graph, mode: SubgraphMode) -> Self {
        let pending: Vec<TripleRef<'a>> = first.iter().filter(|t| !triple_is_ground(*t)).collect();
        let mut triples_of: HashMap<BlankNodeRef<'a>, Vec<usize>> = HashMap::new();
        for (i, t) in pending.iter().enumerate() {
            let mut bnodes = Vec::new();
            blank_nodes(*t, &mut bnodes);
            bnodes.sort_by_key(|b| b.as_str());
            bnodes.dedup();
            for b in bnodes {
                triples_of.entry(b).or_default().push(i);
            }
        }
        let mut bnodes: Vec<_> = triples_of.keys().copied().collect();
        bnodes.sort_by(|x, y| {
            triples_of[y]
                .len()
                .cmp(&triples_of[x].len())
                .then_with(|| x.as_str().cmp(y.as_str()))
        });
        Problem {
            mode,
            second,
            pending,
            bnodes,
            triples_of,
        }
    }

    /// The terms of `second` that `b` could be mapped to, the ones that could
    /// match more of its triples first.
    fn domain(&self, b: BlankNodeRef<'a>) -> Vec<TermRef<'b>> {
        let mut counts: HashMap<TermRef<'b>, usize> = HashMap::new();
        for i in &self.triples_of[&b] {
            let t = self.pending[*i];
            let mut seen = HashSet::new();
            for c in candidate_triples(t, self.second) {
                let mut m = Mapping::new(self.mode);
                if unify_triple(t, c, &mut m)
                    && let Some(image) = m.forward.get(&b)
                    && seen.insert(*image)
                {
                    *counts.entry(*image).or_default() += 1;
                }
            }
        }
        let mut domain: Vec<(TermRef<'b>, usize, String)> =
            counts.into_iter().map(|(t, n)| (t, n, t.to_string())).collect();
        domain.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.2.cmp(&y.2)));
        domain.into_iter().map(|(t, _, _)| t).collect()
    }

    /// Number of triples with blank nodes that `m` matches
    fn score(&self, m: &BlankNodeMap<'a, 'b>) -> usize {
        self.pending.iter().filter(|t| matches(**t, m, self.second)).count()
    }

    /// Matched triples of `b`, given `m`
    fn gain(&self, b: BlankNodeRef<'a>, m: &BlankNodeMap<'a, 'b>) -> usize {
        self.triples_of[&b]
            .iter()
            .filter(|i| matches(self.pending[**i], m, self.second))
            .count()
    }
}

/// A mapping of the blank nodes of `first` to terms of `second` that matches
/// as many triples as possible, and whether it is proven to be the best one.
///
/// Only called when `first` is not contained in `second` (in `mode`): some
/// triple is then unmatched by any mapping, which bounds the search.
fn best_mapping<'a, 'b>(
    first: &'a Graph,
    second: &'b Graph,
    mode: SubgraphMode,
    limit: usize,
) -> (BlankNodeMap<'a, 'b>, bool) {
    let problem = Problem::new(first, second, mode);
    if problem.bnodes.is_empty() {
        return (HashMap::new(), true);
    }
    // If every ground triple is in `second`, a triple with blank nodes is unmatched
    let ground_contained = first.iter().all(|t| !triple_is_ground(t) || second.contains(t));
    let upper_bound = problem.pending.len() - usize::from(ground_contained);
    let heuristic = heuristic_mapping(&problem, first);
    let heuristic_score = problem.score(&heuristic);
    if heuristic_score == upper_bound {
        return (heuristic, true);
    }
    if limit == 0 {
        return (heuristic, false);
    }
    let mut search = ExactSearch::new(&problem, heuristic, heuristic_score, upper_bound, limit);
    let exact = search.run();
    (search.best, exact)
}

// ---------------------------------------------------------------------------
// Heuristic: pair blank nodes with the same neighbourhood, then greedily
// ---------------------------------------------------------------------------

/// Number of colour refinement rounds
const REFINEMENT_ROUNDS: usize = 3;

/// A colour for each blank node of `graph` that summarizes its
/// neighbourhood: blank nodes of two graphs with the same colour look the same
/// up to `REFINEMENT_ROUNDS` triples away.
fn colours(graph: &Graph) -> HashMap<BlankNodeRef<'_>, u64> {
    let mut triples_of: HashMap<BlankNodeRef<'_>, Vec<TripleRef<'_>>> = HashMap::new();
    for t in graph.iter() {
        let mut bnodes = Vec::new();
        blank_nodes(t, &mut bnodes);
        bnodes.sort_by_key(|b| b.as_str());
        bnodes.dedup();
        for b in bnodes {
            triples_of.entry(b).or_default().push(t);
        }
    }
    let mut colours: HashMap<BlankNodeRef<'_>, u64> = triples_of.keys().map(|b| (*b, 0)).collect();
    for _ in 0..=REFINEMENT_ROUNDS {
        colours = triples_of
            .iter()
            .map(|(b, ts)| {
                let mut signatures: Vec<String> = ts.iter().map(|t| signature(*t, *b, &colours)).collect();
                signatures.sort();
                let mut hasher = DefaultHasher::new();
                signatures.hash(&mut hasher);
                (*b, hasher.finish())
            })
            .collect();
    }
    colours
}

/// `t` written with `b` as `@` and the other blank nodes as their colour.
fn signature(t: TripleRef<'_>, b: BlankNodeRef<'_>, colours: &HashMap<BlankNodeRef<'_>, u64>) -> String {
    fn node(n: BlankNodeRef<'_>, b: BlankNodeRef<'_>, colours: &HashMap<BlankNodeRef<'_>, u64>) -> String {
        if n == b {
            "@".to_string()
        } else {
            format!("#{}", colours[&n])
        }
    }
    fn term(o: TermRef<'_>, b: BlankNodeRef<'_>, colours: &HashMap<BlankNodeRef<'_>, u64>) -> String {
        match o {
            TermRef::BlankNode(n) => node(n, b, colours),
            TermRef::Triple(inner) => format!("<<( {} )>>", signature(inner.as_ref(), b, colours)),
            o => o.to_string(),
        }
    }
    let subject = match t.subject {
        NamedOrBlankNodeRef::BlankNode(n) => node(n, b, colours),
        s => s.to_string(),
    };
    format!("{subject} {} {}", t.predicate, term(t.object, b, colours))
}

fn heuristic_mapping<'a, 'b>(problem: &Problem<'a, 'b>, first: &'a Graph) -> BlankNodeMap<'a, 'b> {
    let first_colours = colours(first);
    let second_colours = colours(problem.second);
    let mut by_colour: HashMap<u64, Vec<BlankNodeRef<'b>>> = HashMap::new();
    for (b, c) in &second_colours {
        by_colour.entry(*c).or_default().push(*b);
    }
    for group in by_colour.values_mut() {
        group.sort_by_key(|b| b.as_str());
    }

    // 1. Blank nodes with the same colour
    let mut mapping = Mapping::new(problem.mode);
    for b in &problem.bnodes {
        let Some(group) = first_colours.get(b).and_then(|c| by_colour.get(c)) else {
            continue;
        };
        for candidate in group {
            let mark = mapping.trail.len();
            if mapping.bind(*b, TermRef::BlankNode(*candidate)) {
                break;
            }
            mapping.undo_to(mark);
        }
    }
    // Drop the pairs that match nothing: they would only block better ones
    let useful: Vec<_> = problem
        .bnodes
        .iter()
        .filter(|b| mapping.forward.contains_key(*b) && problem.gain(**b, &mapping.forward) > 0)
        .map(|b| (*b, mapping.forward[b]))
        .collect();
    let mut mapping = Mapping::new(problem.mode);
    for (b, image) in useful {
        mapping.bind(b, image);
    }

    // 2. Greedily, the image that matches the most triples, until nothing changes
    loop {
        let mut changed = false;
        for b in &problem.bnodes {
            if mapping.forward.contains_key(b) {
                continue;
            }
            let mut best: Option<(usize, TermRef<'b>)> = None;
            for candidate in &problem.domain(*b) {
                let mark = mapping.trail.len();
                if mapping.bind(*b, *candidate) {
                    let gain = problem.gain(*b, &mapping.forward);
                    if gain > 0 && best.is_none_or(|(g, _)| gain > g) {
                        best = Some((gain, *candidate));
                    }
                }
                mapping.undo_to(mark);
            }
            if let Some((_, image)) = best {
                mapping.bind(*b, image);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    mapping.forward
}

// ---------------------------------------------------------------------------
// Exact search: branch and bound over the images of each blank node
// ---------------------------------------------------------------------------

struct ExactSearch<'p, 'a, 'b> {
    problem: &'p Problem<'a, 'b>,
    /// Possible images of each blank node, in search order
    domains: Vec<Vec<TermRef<'b>>>,
    /// Indices in `pending` of the triples whose last blank node (in search order) is each one
    decided_at: Vec<Vec<usize>>,
    /// Number of triples decided at each index or later
    remaining: Vec<usize>,
    mapping: Mapping<'a, 'b>,
    best: BlankNodeMap<'a, 'b>,
    best_score: usize,
    /// The most triples a mapping can match: the search stops when reached
    upper_bound: usize,
    steps: usize,
    limit: usize,
}

impl<'p, 'a, 'b> ExactSearch<'p, 'a, 'b> {
    fn new(
        problem: &'p Problem<'a, 'b>,
        best: BlankNodeMap<'a, 'b>,
        best_score: usize,
        upper_bound: usize,
        limit: usize,
    ) -> Self {
        let position: HashMap<BlankNodeRef<'a>, usize> =
            problem.bnodes.iter().enumerate().map(|(i, b)| (*b, i)).collect();
        let mut decided_at = vec![Vec::new(); problem.bnodes.len()];
        for (i, t) in problem.pending.iter().enumerate() {
            let mut bnodes = Vec::new();
            blank_nodes(*t, &mut bnodes);
            let last = bnodes
                .iter()
                .map(|b| position[b])
                .max()
                .expect("pending triples have blank nodes");
            decided_at[last].push(i);
        }
        let mut remaining = vec![0; problem.bnodes.len() + 1];
        for i in (0..problem.bnodes.len()).rev() {
            remaining[i] = remaining[i + 1] + decided_at[i].len();
        }
        ExactSearch {
            problem,
            domains: problem.bnodes.iter().map(|b| problem.domain(*b)).collect(),
            decided_at,
            remaining,
            mapping: Mapping::new(problem.mode),
            best,
            best_score,
            upper_bound,
            steps: 0,
            limit,
        }
    }

    /// Returns `true` if the search finished within the limit: the best mapping is then optimal.
    fn run(&mut self) -> bool {
        self.visit(0, 0)
    }

    fn visit(&mut self, i: usize, matched: usize) -> bool {
        if self.best_score >= self.upper_bound {
            // Optimal: nothing can match more
            return true;
        }
        self.steps += 1;
        if self.steps > self.limit {
            return false;
        }
        if matched + self.remaining[i] <= self.best_score {
            // Cannot beat the best mapping so far
            return true;
        }
        if i == self.problem.bnodes.len() {
            self.best_score = matched;
            self.best = self.mapping.forward.clone();
            return true;
        }
        let b = self.problem.bnodes[i];
        let images: Vec<Option<TermRef<'b>>> = self.domains[i].iter().copied().map(Some).chain([None]).collect();
        for image in images {
            let mark = self.mapping.trail.len();
            if image.is_none_or(|image| self.mapping.bind(b, image)) {
                let gained = self.decided_at[i]
                    .iter()
                    .filter(|t| matches(self.problem.pending[**t], &self.mapping.forward, self.problem.second))
                    .count();
                let finished = self.visit(i + 1, matched + gained);
                self.mapping.undo_to(mark);
                if !finished {
                    return false;
                }
            } else {
                self.mapping.undo_to(mark);
            }
        }
        true
    }
}
