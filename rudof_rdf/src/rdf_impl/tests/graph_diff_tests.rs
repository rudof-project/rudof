//! Test suite for RDF graph diffs: the triples of each graph that the other
//! one does not have, up to blank node renaming.
//!
//! Each case gives two Turtle documents, the expected triples only in each of
//! them (also as Turtle, with the blank node labels of their documents), the
//! expected relation and whether the diff is exact. Every case is also
//! checked with the graphs swapped.

use crate::rdf_core::RDFFormat;
use crate::rdf_impl::{DiffOptions, GraphDiff, GraphRelation, OxigraphInMemory, ReaderMode, SubgraphMode};
use GraphRelation::*;
use SubgraphMode::{Entailment, Homomorphic, Injective};
use oxrdf::{BlankNode, Term, Triple};
use std::collections::BTreeSet;

const PREFIXES: &str = r#"
PREFIX :     <http://example.org/>
PREFIX rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
"#;

fn ttl(data: &str) -> OxigraphInMemory {
    let doc = format!("{PREFIXES}\n{data}");
    OxigraphInMemory::from_str(&doc, &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .unwrap_or_else(|e| panic!("Invalid Turtle in test case: {e}\n{doc}"))
}

fn triples(data: &str) -> BTreeSet<String> {
    ttl(data).quads().map(|q| Triple::from(q).to_string()).collect()
}

fn strings(ts: &[Triple]) -> BTreeSet<String> {
    ts.iter().map(ToString::to_string).collect()
}

const DEFAULT: DiffOptions = DiffOptions {
    exact_search_limit: crate::rdf_impl::DEFAULT_EXACT_SEARCH_LIMIT,
};
const HEURISTIC: DiffOptions = DiffOptions { exact_search_limit: 0 };

struct Case<'a> {
    name: &'a str,
    first: &'a str,
    second: &'a str,
    mode: SubgraphMode,
    options: DiffOptions,
    relation: GraphRelation,
    only_in_first: &'a str,
    only_in_second: &'a str,
    exact: bool,
}

impl Case<'_> {
    fn check(&self) -> GraphDiff {
        let (g1, g2) = (ttl(self.first), ttl(self.second));
        let name = self.name;

        let d = g1.diff_with(&g2, self.mode, &self.options);
        assert_eq!(d.relation, self.relation, "{name}: relation");
        assert_eq!(d.mode, self.mode, "{name}: mode");
        assert_eq!(d.exact, self.exact, "{name}: exact");
        assert_eq!(
            strings(&d.only_in_first),
            triples(self.only_in_first),
            "{name}: only in first"
        );
        assert_eq!(
            strings(&d.only_in_second),
            triples(self.only_in_second),
            "{name}: only in second"
        );

        let swapped = g2.diff_with(&g1, self.mode, &self.options);
        assert_eq!(swapped.relation, self.relation.inverse(), "{name} swapped: relation");
        assert_eq!(swapped.exact, self.exact, "{name} swapped: exact");
        assert_eq!(swapped.only_in_first, d.only_in_second, "{name} swapped: only in first");
        assert_eq!(
            swapped.only_in_second, d.only_in_first,
            "{name} swapped: only in second"
        );
        d
    }
}

fn bnode(label: &str) -> BlankNode {
    BlankNode::new_unchecked(label)
}

fn iri(local: &str) -> Term {
    oxrdf::NamedNode::new_unchecked(format!("http://example.org/{local}")).into()
}

// ---------------------------------------------------------------------------
// Graphs without differences, and ground differences
// ---------------------------------------------------------------------------

#[test]
fn isomorphic_graphs_have_no_differences() {
    let d = Case {
        name: "isomorphic",
        first: ":a :p _:x . _:x :q _:y .",
        second: ":a :p _:m . _:m :q _:n .",
        mode: Injective,
        options: DEFAULT,
        relation: Isomorphic,
        only_in_first: "",
        only_in_second: "",
        exact: true,
    }
    .check();
    assert!(d.is_empty());
    assert_eq!(
        d.first_to_second,
        vec![(bnode("x"), bnode("m").into()), (bnode("y"), bnode("n").into())]
    );
    assert_eq!(
        d.second_to_first,
        vec![(bnode("m"), bnode("x").into()), (bnode("n"), bnode("y").into())]
    );
}

#[test]
fn ground_differences_are_a_set_difference() {
    let d = Case {
        name: "ground",
        first: ":a :p 1 . :a :q 2 .",
        second: ":a :p 1 . :a :q 3 .",
        mode: Injective,
        options: DEFAULT,
        relation: Different,
        only_in_first: ":a :q 2 .",
        only_in_second: ":a :q 3 .",
        exact: true,
    }
    .check();
    assert!(d.first_to_second.is_empty() && d.second_to_first.is_empty());
}

#[test]
fn ground_graphs_are_exact_even_without_search() {
    Case {
        name: "ground heuristic",
        first: ":a :p 1 . :a :q 2 .",
        second: ":a :p 1 . :a :q 3 .",
        mode: Injective,
        options: HEURISTIC,
        relation: Different,
        only_in_first: ":a :q 2 .",
        only_in_second: ":a :q 3 .",
        exact: true,
    }
    .check();
}

// ---------------------------------------------------------------------------
// One graph contained in the other: the containment mapping gives the diff
// ---------------------------------------------------------------------------

#[test]
fn subgraph_lists_only_the_extra_triples() {
    Case {
        name: "subgraph",
        first: "_:x :name \"Bob\" .",
        second: "_:y :name \"Bob\" ; :age 3 .",
        mode: Injective,
        options: DEFAULT,
        relation: Subgraph,
        only_in_first: "",
        only_in_second: "_:y :age 3 .",
        exact: true,
    }
    .check();
}

#[test]
fn containment_is_exact_even_without_search() {
    Case {
        name: "subgraph heuristic",
        first: "_:x :name \"Bob\" .",
        second: "_:y :name \"Bob\" ; :age 3 .",
        mode: Injective,
        options: HEURISTIC,
        relation: Subgraph,
        only_in_first: "",
        only_in_second: "_:y :age 3 .",
        exact: true,
    }
    .check();
}

#[test]
fn subgraph_backtracking_picks_the_right_blank_node() {
    Case {
        name: "subgraph backtracking",
        first: "_:a :p _:b . _:b :q :o .",
        second: "_:x :p _:y . _:x :p _:z . _:z :q :o .",
        mode: Injective,
        options: DEFAULT,
        relation: Subgraph,
        only_in_first: "",
        only_in_second: "_:x :p _:y .",
        exact: true,
    }
    .check();
}

#[test]
fn entailment_maps_blank_nodes_to_iris() {
    let d = Case {
        name: "entailment subgraph",
        first: ":alice :knows _:b . _:b :name \"Bob\" .",
        second: ":alice :knows :bob . :bob :name \"Bob\" ; :age 3 .",
        mode: Entailment,
        options: DEFAULT,
        relation: Subgraph,
        only_in_first: "",
        only_in_second: ":bob :age 3 .",
        exact: true,
    }
    .check();
    // The mapping goes from the contained graph into the other one
    assert_eq!(d.first_to_second, vec![(bnode("b"), iri("bob"))]);
    assert!(d.second_to_first.is_empty());
}

// ---------------------------------------------------------------------------
// Different graphs: the best blank node mapping
// ---------------------------------------------------------------------------

#[test]
fn changed_literal_behind_a_blank_node() {
    let d = Case {
        name: "changed literal",
        first: ":alice :knows _:b . _:b :name \"Bob\" .",
        second: ":alice :knows _:c . _:c :name \"Robert\" .",
        mode: Injective,
        options: DEFAULT,
        relation: Different,
        only_in_first: "_:b :name \"Bob\" .",
        only_in_second: "_:c :name \"Robert\" .",
        exact: true,
    }
    .check();
    assert_eq!(d.first_to_second, vec![(bnode("b"), bnode("c").into())]);
    assert_eq!(d.second_to_first, vec![(bnode("c"), bnode("b").into())]);
}

#[test]
fn heuristic_result_can_be_proven_optimal() {
    // The containment check failed, so at least one triple is unmatched: the
    // heuristic leaves only one, so it is optimal without any search
    Case {
        name: "changed literal heuristic",
        first: ":alice :knows _:b . _:b :name \"Bob\" .",
        second: ":alice :knows _:c . _:c :name \"Robert\" .",
        mode: Injective,
        options: HEURISTIC,
        relation: Different,
        only_in_first: "_:b :name \"Bob\" .",
        only_in_second: "_:c :name \"Robert\" .",
        exact: true,
    }
    .check();
}

#[test]
fn best_alignment_of_several_blank_nodes() {
    Case {
        name: "alignment",
        first: "_:a :name \"A\" ; :age 1 . _:b :name \"B\" ; :age 2 .",
        second: "_:x :name \"A\" ; :age 1 . _:y :name \"B\" ; :age 3 .",
        mode: Injective,
        options: DEFAULT,
        relation: Different,
        only_in_first: "_:b :age 2 .",
        only_in_second: "_:y :age 3 .",
        exact: true,
    }
    .check();
}

#[test]
fn blank_nodes_that_look_alike() {
    Case {
        name: "look alike",
        first: "_:a :p :o . _:b :p :o . _:a :q 1 .",
        second: "_:c :p :o . _:d :p :o . _:d :q 2 .",
        mode: Injective,
        options: DEFAULT,
        relation: Different,
        only_in_first: "_:a :q 1 .",
        only_in_second: "_:d :q 2 .",
        exact: true,
    }
    .check();
}

#[test]
fn unmatched_blank_node_keeps_all_its_triples() {
    Case {
        name: "unmatched",
        first: ":a :p 1 . _:x :q 2 ; :r 3 .",
        second: ":a :p 1 .",
        mode: Injective,
        options: DEFAULT,
        relation: Supergraph,
        only_in_first: "_:x :q 2 ; :r 3 .",
        only_in_second: "",
        exact: true,
    }
    .check();
}

#[test]
fn homomorphic_mode_lists_each_direction() {
    Case {
        name: "homomorphic",
        first: "_:a :p :o . _:a :q 1 .",
        second: "_:c :p :o . _:d :q 2 .",
        mode: Homomorphic,
        options: DEFAULT,
        relation: Different,
        only_in_first: "_:a :q 1 .",
        only_in_second: "_:d :q 2 .",
        exact: true,
    }
    .check();
}

#[test]
fn entailment_diff_can_be_empty_without_equivalence() {
    // `:a :p :a` entails `_:b :p _:b`, not the other way round, but each triple
    // has a counterpart: the difference is in the mapping
    let d = Case {
        name: "empty entailment diff",
        first: ":a :p :a .",
        second: "_:b :p _:b .",
        mode: Entailment,
        options: DEFAULT,
        relation: Supergraph,
        only_in_first: "",
        only_in_second: "",
        exact: true,
    }
    .check();
    assert_eq!(d.second_to_first, vec![(bnode("b"), iri("a"))]);
}

#[test]
fn entailment_mode_lists_what_is_not_entailed() {
    // `_:x` can only be one of :a or :b: mapping it to :a matches two triples
    let d = Case {
        name: "not entailed",
        first: "_:x :p :o ; :q :r ; :s :t .",
        second: ":a :p :o ; :q :r . :b :s :u .",
        mode: Entailment,
        options: DEFAULT,
        relation: Different,
        only_in_first: "_:x :s :t .",
        only_in_second: ":b :s :u .",
        exact: true,
    }
    .check();
    assert_eq!(d.first_to_second, vec![(bnode("x"), iri("a"))]);
}

// ---------------------------------------------------------------------------
// RDF 1.2
// ---------------------------------------------------------------------------

#[test]
fn rdf12_blank_nodes_inside_triple_terms() {
    let d = Case {
        name: "triple terms",
        first: ":a :says <<( _:x :p :o1 )>> . _:x :name \"X\" .",
        second: ":a :says <<( _:y :p :o2 )>> . _:y :name \"X\" .",
        mode: Injective,
        options: DEFAULT,
        relation: Different,
        only_in_first: ":a :says <<( _:x :p :o1 )>> .",
        only_in_second: ":a :says <<( _:y :p :o2 )>> .",
        exact: true,
    }
    .check();
    assert_eq!(d.first_to_second, vec![(bnode("x"), bnode("y").into())]);
}

#[test]
fn rdf12_changed_annotation() {
    Case {
        name: "annotation",
        first: "_:r rdf:reifies <<( :s :p :o )>> ; :source :w1 .",
        second: "_:q rdf:reifies <<( :s :p :o )>> ; :source :w2 .",
        mode: Injective,
        options: DEFAULT,
        relation: Different,
        only_in_first: "_:r :source :w1 .",
        only_in_second: "_:q :source :w2 .",
        exact: true,
    }
    .check();
}

// ---------------------------------------------------------------------------
// Serialization
// ---------------------------------------------------------------------------

#[test]
fn diff_serializes_triples_as_ntriples() {
    let d = ttl(":alice :knows _:b . _:b :name \"Bob\" .").diff_with(
        &ttl(":alice :knows _:c . _:c :name \"Robert\" ."),
        Injective,
        &DEFAULT,
    );
    let json = serde_json::to_value(&d).unwrap();
    assert_eq!(json["relation"], "different");
    assert_eq!(json["mode"], "injective");
    assert_eq!(json["exact"], true);
    assert_eq!(
        json["only_in_first"],
        serde_json::json!(["_:b <http://example.org/name> \"Bob\""])
    );
    assert_eq!(json["first_to_second"], serde_json::json!([["_:b", "_:c"]]));
}

// ---------------------------------------------------------------------------
// Property tests on random graphs
// ---------------------------------------------------------------------------

#[cfg(not(target_family = "wasm"))]
mod properties {
    use super::*;
    use crate::rdf_impl::tests::graph_comparison_tests::properties::{
        EX, IDENTITY, RawTriple, build, permutation, raw_triples,
    };
    use oxrdf::{Graph, NamedNode, NamedOrBlankNode};
    use proptest::prelude::*;
    use std::collections::{HashMap, HashSet};

    fn apply_node(n: &NamedOrBlankNode, m: &HashMap<BlankNode, Term>) -> Option<NamedOrBlankNode> {
        match n {
            NamedOrBlankNode::BlankNode(b) => match m.get(b)? {
                Term::NamedNode(n) => Some(n.clone().into()),
                Term::BlankNode(b) => Some(b.clone().into()),
                _ => None,
            },
            other => Some(other.clone()),
        }
    }

    /// The image of `t` under `m`, if every blank node of `t` is mapped
    fn apply(t: &Triple, m: &HashMap<BlankNode, Term>) -> Option<Triple> {
        let object = match &t.object {
            Term::BlankNode(b) => m.get(b)?.clone(),
            Term::Triple(inner) => apply(inner, m)?.into(),
            other => other.clone(),
        };
        Some(Triple::new(apply_node(&t.subject, m)?, t.predicate.clone(), object))
    }

    fn is_ground(t: &Triple) -> bool {
        !matches!(t.subject, NamedOrBlankNode::BlankNode(_))
            && match &t.object {
                Term::BlankNode(_) => false,
                Term::Triple(inner) => is_ground(inner),
                _ => true,
            }
    }

    fn owned(g: &Graph) -> Vec<Triple> {
        g.iter().map(|t| t.into_owned()).collect()
    }

    /// Checks that `m`, from `source` into `target`, gives the lists: the triples of
    /// `source` it does not match, and the triples of `target` that are not images.
    fn check_from(
        source: &Graph,
        target: &Graph,
        only_in_source: &[Triple],
        only_in_target: &[Triple],
        mapping: &[(BlankNode, Term)],
        mode: SubgraphMode,
    ) -> Result<(), String> {
        for only in [only_in_source, only_in_target] {
            if only.iter().collect::<HashSet<_>>().len() != only.len() {
                return Err("duplicated triples".into());
            }
            if !only.windows(2).all(|w| w[0].to_string() <= w[1].to_string()) {
                return Err("not sorted".into());
            }
        }
        let m: HashMap<BlankNode, Term> = mapping.iter().cloned().collect();
        if m.len() != mapping.len() {
            return Err("a blank node mapped twice".into());
        }
        if mode != Entailment && !m.values().all(|t| matches!(t, Term::BlankNode(_))) {
            return Err(format!("{mode}: non blank target"));
        }
        if mode == Injective && m.values().collect::<HashSet<_>>().len() != m.len() {
            return Err("not injective".into());
        }
        let only_source: HashSet<&Triple> = only_in_source.iter().collect();
        let mut images = HashSet::new();
        for t in owned(source) {
            let image = apply(&t, &m).filter(|i| target.contains(i));
            match (only_source.contains(&t), image) {
                (true, Some(_)) if is_ground(&t) => return Err(format!("ground {t} is in both graphs")),
                (false, None) => return Err(format!("matched {t} has no image")),
                (false, Some(i)) => {
                    images.insert(i);
                },
                _ => {},
            }
        }
        let expected: HashSet<Triple> = owned(target).into_iter().filter(|t| !images.contains(t)).collect();
        if expected != only_in_target.iter().cloned().collect() {
            return Err(format!("the unmatched triples of the target are not {expected:?}"));
        }
        Ok(())
    }

    fn check_diff(
        g1: &Graph,
        g2: &Graph,
        mode: SubgraphMode,
        options: &DiffOptions,
    ) -> Result<GraphDiff, TestCaseError> {
        let d = crate::rdf_impl::diff_graphs(g1, g2, mode, options);
        prop_assert_eq!(d.relation, crate::rdf_impl::compare_graphs_with(g1, g2, mode));
        let from_first = check_from(g1, g2, &d.only_in_first, &d.only_in_second, &d.first_to_second, mode);
        let from_second = check_from(g2, g1, &d.only_in_second, &d.only_in_first, &d.second_to_first, mode);
        if mode == Injective {
            prop_assert!(from_first.is_ok(), "{:?}", from_first);
            prop_assert!(from_second.is_ok(), "{:?}", from_second);
        } else {
            prop_assert!(
                from_first.is_ok() || from_second.is_ok(),
                "{mode}: {:?} / {:?}",
                from_first,
                from_second
            );
            // One direction only, except for isomorphic graphs (one-to-one mapping)
            prop_assert!(d.relation == Isomorphic || d.first_to_second.is_empty() || d.second_to_first.is_empty());
        }
        match d.relation {
            Isomorphic => prop_assert!(d.is_empty()),
            // Each containment mapping leaves nothing of its own graph unmatched
            Equivalent => prop_assert!(d.only_in_first.is_empty() || d.only_in_second.is_empty()),
            // With non-injective modes the containment mapping can also cover the other
            // graph: e.g. `_:b :p _:b` is contained in `:a :p :a`, and matches all of it
            Subgraph => prop_assert!(d.only_in_first.is_empty() && (mode != Injective || !d.only_in_second.is_empty())),
            Supergraph => {
                prop_assert!(d.only_in_second.is_empty() && (mode != Injective || !d.only_in_first.is_empty()))
            },
            Different => prop_assert!(!d.only_in_first.is_empty() || !d.only_in_second.is_empty()),
        }
        if mode == Injective && d.relation == Different {
            prop_assert!(!d.only_in_first.is_empty() && !d.only_in_second.is_empty());
        }
        if d.relation != Different {
            prop_assert!(d.exact, "containment diffs are exact");
        }
        if mode == Injective {
            let inverse: BTreeSet<(String, String)> = d
                .first_to_second
                .iter()
                .map(|(b, t)| (t.to_string(), b.to_string()))
                .collect();
            let second_to_first: BTreeSet<(String, String)> = d
                .second_to_first
                .iter()
                .map(|(b, t)| (b.to_string(), t.to_string()))
                .collect();
            prop_assert_eq!(inverse, second_to_first);
        }
        Ok(d)
    }

    /// The fewest unmatched triples of `first` over every injective partial mapping
    /// of its blank nodes to blank nodes of `second`, by brute force
    fn brute_force_min_unmatched(first: &Graph, second: &Graph) -> usize {
        fn bnodes_of(g: &Graph) -> Vec<BlankNode> {
            fn collect(t: &Triple, acc: &mut HashSet<BlankNode>) {
                if let NamedOrBlankNode::BlankNode(b) = &t.subject {
                    acc.insert(b.clone());
                }
                match &t.object {
                    Term::BlankNode(b) => {
                        acc.insert(b.clone());
                    },
                    Term::Triple(inner) => collect(inner, acc),
                    _ => {},
                }
            }
            let mut acc = HashSet::new();
            for t in g.iter() {
                collect(&t.into_owned(), &mut acc);
            }
            acc.into_iter().collect()
        }
        fn go(
            i: usize,
            from: &[BlankNode],
            to: &[BlankNode],
            used: &mut Vec<bool>,
            m: &mut HashMap<BlankNode, Term>,
            ts: &[Triple],
            second: &Graph,
            best: &mut usize,
        ) {
            if i == from.len() {
                let unmatched = ts
                    .iter()
                    .filter(|t| !apply(t, m).is_some_and(|img| second.contains(&img)))
                    .count();
                *best = (*best).min(unmatched);
                return;
            }
            go(i + 1, from, to, used, m, ts, second, best);
            for j in 0..to.len() {
                if !used[j] {
                    used[j] = true;
                    m.insert(from[i].clone(), to[j].clone().into());
                    go(i + 1, from, to, used, m, ts, second, best);
                    m.remove(&from[i]);
                    used[j] = false;
                }
            }
        }
        let (from, to) = (bnodes_of(first), bnodes_of(second));
        let ts = owned(first);
        let mut best = usize::MAX;
        go(
            0,
            &from,
            &to,
            &mut vec![false; to.len()],
            &mut HashMap::new(),
            &ts,
            second,
            &mut best,
        );
        best
    }

    const MODES: [SubgraphMode; 3] = [Injective, Homomorphic, Entailment];

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]

        /// Diffs are sound in every mode, with the exact search and with the heuristic
        #[test]
        fn diffs_are_sound(raw1 in raw_triples(), raw2 in raw_triples()) {
            let g1 = build(&raw1, "a", &IDENTITY);
            let g2 = build(&raw2, "b", &IDENTITY);
            for mode in MODES {
                let exact = check_diff(&g1, &g2, mode, &DEFAULT)?;
                let heuristic = check_diff(&g1, &g2, mode, &HEURISTIC)?;
                prop_assert!(exact.exact, "{mode}: small graphs fit in the default limit");
                if mode == Injective {
                    prop_assert!(
                        heuristic.only_in_first.len() >= exact.only_in_first.len(),
                        "the heuristic beat the exact search"
                    );
                }
            }
        }

        /// Near misses: a renamed copy with one triple replaced
        #[test]
        fn near_miss_diffs_are_sound(
            raw in raw_triples(),
            perm in permutation(),
            extra in (0u8..8, 0u8..2, 0u8..8, Just(false), Just(0u8), Just(0u8)),
        ) {
            let g1 = build(&raw, "a", &IDENTITY);
            let mut raw2: Vec<RawTriple> = raw.clone();
            raw2.pop();
            raw2.push(extra);
            let g2 = build(&raw2, "b", &perm);
            for mode in MODES {
                check_diff(&g1, &g2, mode, &DEFAULT)?;
                check_diff(&g1, &g2, mode, &HEURISTIC)?;
            }
        }

        /// The exact search finds the fewest unmatched triples
        #[test]
        fn exact_diffs_are_minimal(raw1 in raw_triples(), raw2 in raw_triples()) {
            let g1 = build(&raw1, "a", &IDENTITY);
            let g2 = build(&raw2, "b", &IDENTITY);
            let d = crate::rdf_impl::diff_graphs(&g1, &g2, Injective, &DEFAULT);
            prop_assert!(d.exact);
            prop_assert_eq!(d.only_in_first.len(), brute_force_min_unmatched(&g1, &g2));
            prop_assert_eq!(d.only_in_second.len(), brute_force_min_unmatched(&g2, &g1));
        }

        /// Adding one triple to a renamed copy shows exactly that triple
        #[test]
        fn one_added_triple_is_the_whole_diff(raw in raw_triples(), perm in permutation(), s in 0u8..3, o in 0u8..3) {
            let g1 = build(&raw, "a", &IDENTITY);
            let mut g2 = build(&raw, "b", &perm);
            let extra = Triple::new(
                NamedNode::new_unchecked(format!("{EX}n{s}")),
                NamedNode::new_unchecked(format!("{EX}added")),
                NamedNode::new_unchecked(format!("{EX}n{o}")),
            );
            g2.insert(&extra);
            let d = crate::rdf_impl::diff_graphs(&g1, &g2, Injective, &HEURISTIC);
            prop_assert_eq!(d.relation, Subgraph);
            prop_assert!(d.only_in_first.is_empty());
            prop_assert_eq!(d.only_in_second, vec![extra]);
        }
    }
}

// ---------------------------------------------------------------------------
// Larger graphs
// ---------------------------------------------------------------------------

/// A chain of `n` people with blank nodes; `changed` gets another age
fn people(n: usize, prefix: &str, changed: Option<usize>) -> String {
    (0..n)
        .map(|i| {
            let age = if Some(i) == changed { 1000 + i } else { i };
            format!(
                "_:{prefix}{i} :name \"P{i}\" ; :age {age} ; :knows _:{prefix}{} .\n",
                (i + 1) % n
            )
        })
        .collect()
}

#[test]
fn large_graph_with_one_change() {
    let n = 500;
    let (g1, g2) = (ttl(&people(n, "a", None)), ttl(&people(n, "b", Some(42))));
    let start = std::time::Instant::now();
    let d = g1.diff_with(&g2, Injective, &DEFAULT);
    let elapsed = start.elapsed();
    assert_eq!(d.relation, Different);
    assert_eq!(strings(&d.only_in_first), triples("_:a42 :age 42 ."));
    assert_eq!(strings(&d.only_in_second), triples("_:b42 :age 1042 ."));
    assert_eq!(d.first_to_second.len(), n);
    // One change: proven optimal without searching
    assert!(d.exact);
    assert!(elapsed.as_secs() < 10, "took {elapsed:?}");
}

#[test]
fn search_limit_falls_back_to_the_heuristic() {
    // Many blank nodes that look alike and two changes: the heuristic cannot be
    // proven optimal, and the exact search cannot finish in 10 steps
    let first: String = (0..20).map(|i| format!("_:a{i} :p :o . ")).collect::<String>() + "_:a0 :q 1 . _:a1 :r 1 .";
    let second: String = (0..20).map(|i| format!("_:b{i} :p :o . ")).collect::<String>() + "_:b7 :q 2 . _:b3 :r 2 .";
    let d = ttl(&first).diff_with(&ttl(&second), Injective, &DiffOptions { exact_search_limit: 10 });
    assert!(!d.exact);
    assert_eq!(strings(&d.only_in_first), triples("_:a0 :q 1 . _:a1 :r 1 ."));
    assert_eq!(strings(&d.only_in_second), triples("_:b7 :q 2 . _:b3 :r 2 ."));
    // With the default limit, the search proves it
    let d = ttl(&first).diff_with(&ttl(&second), Injective, &DEFAULT);
    assert!(d.exact);
    assert_eq!(d.only_in_first.len(), 2);
}
