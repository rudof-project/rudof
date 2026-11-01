//! Test suite for RDF graph comparison (isomorphism and subgraph checks).
//!
//! Each case is a pair of Turtle documents and the expected [`GraphRelation`]
//! between them. Every case is checked in both directions, and the boolean
//! helpers (`is_isomorphic`, `is_subgraph_of`) are checked to agree with it.

use crate::rdf_core::RDFFormat;
use crate::rdf_impl::{GraphRelation, OxigraphInMemory, ReaderMode, SubgraphMode};
use GraphRelation::*;
use SubgraphMode::{Entailment, Homomorphic, Injective};

const PREFIXES: &str = r#"
PREFIX :     <http://example.org/>
PREFIX rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX xsd:  <http://www.w3.org/2001/XMLSchema#>
"#;

fn ttl(data: &str) -> OxigraphInMemory {
    let doc = format!("{PREFIXES}\n{data}");
    OxigraphInMemory::from_str(&doc, &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .unwrap_or_else(|e| panic!("Invalid Turtle in test case: {e}\n{doc}"))
}

/// Checks `expected` in both directions and its consistency with the boolean helpers.
fn check(name: &str, first: &str, second: &str, expected: GraphRelation) {
    let g1 = ttl(first);
    let g2 = ttl(second);

    assert_eq!(g1.compare_with(&g2), expected, "{name}: compare(first, second)");
    assert_eq!(
        g1.compare_with_mode(&g2, Injective),
        expected,
        "{name}: injective mode is the default"
    );
    assert_eq!(
        g2.compare_with(&g1),
        expected.inverse(),
        "{name}: compare(second, first)"
    );

    let iso = expected == Isomorphic;
    assert_eq!(g1.is_isomorphic(&g2), iso, "{name}: is_isomorphic(first, second)");
    assert_eq!(g2.is_isomorphic(&g1), iso, "{name}: is_isomorphic(second, first)");

    let sub = matches!(expected, Isomorphic | Subgraph);
    let sup = matches!(expected, Isomorphic | Supergraph);
    assert_eq!(g1.is_subgraph_of(&g2), sub, "{name}: is_subgraph_of(first, second)");
    assert_eq!(g2.is_subgraph_of(&g1), sup, "{name}: is_subgraph_of(second, first)");
}

// ---------------------------------------------------------------------------
// GraphRelation helpers
// ---------------------------------------------------------------------------

#[test]
fn inverse_relation() {
    assert_eq!(Isomorphic.inverse(), Isomorphic);
    assert_eq!(Subgraph.inverse(), Supergraph);
    assert_eq!(Supergraph.inverse(), Subgraph);
    assert_eq!(Different.inverse(), Different);
}

#[test]
fn every_graph_is_isomorphic_to_itself() {
    let g = ttl(":a :p _:x . _:x :q _:y . _:y :r <<( _:x :p :o )>> .");
    assert_eq!(g.compare_with(&g), Isomorphic);
}

// ---------------------------------------------------------------------------
// Ground graphs (no blank nodes): plain set comparison
// ---------------------------------------------------------------------------

#[test]
fn empty_graphs_are_isomorphic() {
    check("empty", "", "", Isomorphic);
}

#[test]
fn identical_ground_graphs() {
    check(
        "identical ground",
        ":a :p :b . :b :q 1 .",
        ":a :p :b . :b :q 1 .",
        Isomorphic,
    );
}

#[test]
fn order_and_syntax_do_not_matter() {
    check(
        "order and syntax",
        ":a :p :b ; :q \"x\" .",
        "<http://example.org/a> <http://example.org/q> \"x\" . :a :p :b .",
        Isomorphic,
    );
}

#[test]
fn duplicate_triples_collapse() {
    check("duplicates", ":a :p :b . :a :p :b .", ":a :p :b .", Isomorphic);
}

#[test]
fn literal_datatype_matters() {
    check("datatype", ":a :p 1 .", ":a :p \"1\" .", Different);
}

#[test]
fn literal_language_tag_matters() {
    check("lang tag", ":a :p \"chat\"@en .", ":a :p \"chat\"@fr .", Different);
}

#[test]
fn literal_lexical_form_matters() {
    // "01"^^xsd:integer and "1"^^xsd:integer are different terms (same value)
    check("lexical form", ":a :p \"01\"^^xsd:integer .", ":a :p 1 .", Different);
}

#[test]
fn ground_proper_subset() {
    check("ground subset", ":a :p :b .", ":a :p :b . :b :q :c .", Subgraph);
}

#[test]
fn empty_graph_is_subgraph_of_any_graph() {
    check("empty subgraph", "", ":a :p :b .", Subgraph);
}

#[test]
fn ground_overlapping_graphs() {
    check("overlap", ":a :p :b . :a :p :c .", ":a :p :b . :a :p :d .", Different);
}

#[test]
fn ground_disjoint_graphs() {
    check("disjoint", ":a :p :b .", ":c :q :d .", Different);
}

// ---------------------------------------------------------------------------
// Blank nodes
// ---------------------------------------------------------------------------

#[test]
fn renamed_blank_node() {
    check("renamed bnode", ":a :p _:x .", ":a :p _:y .", Isomorphic);
}

#[test]
fn labelled_and_anonymous_blank_nodes() {
    check(
        "anonymous bnode",
        ":a :knows [ :name \"Bob\" ] .",
        ":a :knows _:b . _:b :name \"Bob\" .",
        Isomorphic,
    );
}

#[test]
fn same_label_is_not_the_same_node_across_graphs() {
    // Labels are local to a graph: `_:x` in both graphs is irrelevant
    check("same label", "_:x :p :a .", "_:x :p :b .", Different);
}

#[test]
fn blank_node_chain_renamed() {
    check(
        "chain",
        ":a :p _:1 . _:1 :p _:2 . _:2 :p :b .",
        ":a :p _:z . _:z :p _:y . _:y :p :b .",
        Isomorphic,
    );
}

#[test]
fn blank_node_is_not_an_iri() {
    check("bnode vs iri", ":a :p _:x .", ":a :p :x .", Different);
}

#[test]
fn two_blank_nodes_vs_self_loop() {
    // Mapping both _:x and _:y to _:z would make the first an instance of the
    // second (simple entailment), but blank node mappings are injective here.
    check("self loop", "_:x :p _:y .", "_:z :p _:z .", Different);
}

#[test]
fn swapped_blank_node_roles() {
    check(
        "swapped roles",
        "_:x :name \"A\" . _:y :name \"B\" . _:x :knows _:y .",
        "_:x :name \"B\" . _:y :name \"A\" . _:y :knows _:x .",
        Isomorphic,
    );
}

#[test]
fn blank_node_wrong_edge_direction() {
    check(
        "edge direction",
        "_:x :name \"A\" . _:y :name \"B\" . _:x :knows _:y .",
        "_:x :name \"A\" . _:y :name \"B\" . _:y :knows _:x .",
        Different,
    );
}

#[test]
fn rdf_lists_syntax_vs_explicit() {
    check(
        "list",
        ":a :p ( 1 2 ) .",
        ":a :p _:l1 . _:l1 rdf:first 1 ; rdf:rest _:l2 . _:l2 rdf:first 2 ; rdf:rest rdf:nil .",
        Isomorphic,
    );
}

#[test]
fn rdf_lists_order_matters() {
    check("list order", ":a :p ( 1 2 ) .", ":a :p ( 2 1 ) .", Different);
}

// Hard cases: all blank nodes look the same locally (same degree, same
// predicate), so a pure "hash the neighbourhood" approach cannot tell them apart.

#[test]
fn cycle_renamed_with_other_starting_point() {
    check(
        "4-cycle",
        "_:a :p _:b . _:b :p _:c . _:c :p _:d . _:d :p _:a .",
        "_:w :p _:x . _:x :p _:y . _:y :p _:z . _:z :p _:w .",
        Isomorphic,
    );
}

#[test]
fn six_cycle_vs_two_three_cycles() {
    check(
        "6-cycle vs 2x3-cycle",
        "_:a :p _:b . _:b :p _:c . _:c :p _:d . _:d :p _:e . _:e :p _:f . _:f :p _:a .",
        "_:a :p _:b . _:b :p _:c . _:c :p _:a . _:d :p _:e . _:e :p _:f . _:f :p _:d .",
        Different,
    );
}

#[test]
fn symmetric_blank_nodes_with_distinct_tail() {
    check(
        "symmetric",
        "_:a :p _:b . _:b :p _:a . _:a :q 1 .",
        "_:x :p _:y . _:y :p _:x . _:y :q 1 .",
        Isomorphic,
    );
}

// Subgraphs with blank nodes

#[test]
fn blank_node_subgraph_with_renaming() {
    check(
        "bnode subgraph",
        "_:x :name \"A\" .",
        "_:y :name \"A\" ; :age 3 .",
        Subgraph,
    );
}

#[test]
fn blank_node_subgraph_needs_injective_mapping() {
    // Both blank nodes of the first graph would need the single one of the second
    check(
        "injective",
        "_:a :p :o . _:b :p :o .",
        "_:c :p :o . :s :q :o .",
        Different,
    );
}

#[test]
fn blank_node_subgraph_needs_backtracking() {
    // A greedy match _:b -> _:y fails on the second triple, _:b -> _:z succeeds
    check(
        "backtracking",
        "_:a :p _:b . _:b :q :o .",
        "_:x :p _:y . _:x :p _:z . _:z :q :o .",
        Subgraph,
    );
}

#[test]
fn ground_part_mismatch_with_blank_nodes() {
    check(
        "ground mismatch",
        "_:a :p :o . :s :p :o .",
        "_:x :p :o . _:x :q :r .",
        Different,
    );
}

#[test]
fn larger_blank_node_subgraph() {
    check(
        "larger subgraph",
        "_:p1 :name \"Alice\" ; :knows _:p2 . _:p2 :name \"Bob\" .",
        "_:a :name \"Alice\" ; :knows _:b, _:c . _:b :name \"Carol\" . _:c :name \"Bob\" ; :knows _:a .",
        Subgraph,
    );
}

// ---------------------------------------------------------------------------
// RDF 1.2: triple terms, reifiers and annotations
// ---------------------------------------------------------------------------

#[test]
fn rdf12_identical_ground_triple_terms() {
    check(
        "ground triple term",
        ":a :says <<( :s :p :o )>> .",
        ":a :says <<( :s :p :o )>> .",
        Isomorphic,
    );
}

#[test]
fn rdf12_triple_term_inner_object_matters() {
    check(
        "triple term object",
        ":a :says <<( :s :p :o1 )>> .",
        ":a :says <<( :s :p :o2 )>> .",
        Different,
    );
}

#[test]
fn rdf12_blank_node_inside_triple_term_renamed() {
    check(
        "bnode in triple term",
        ":a :says <<( _:x :p :o )>> .",
        ":a :says <<( _:y :p :o )>> .",
        Isomorphic,
    );
}

#[test]
fn rdf12_blank_node_as_object_inside_triple_term() {
    check(
        "bnode object in triple term",
        ":a :says <<( :s :p _:x )>> .",
        ":a :says <<( :s :p _:y )>> .",
        Isomorphic,
    );
}

#[test]
fn rdf12_blank_node_shared_inside_and_outside_triple_term() {
    check(
        "shared bnode",
        "_:x :name \"A\" . :a :says <<( _:x :p :o )>> .",
        "_:y :name \"A\" . :a :says <<( _:y :p :o )>> .",
        Isomorphic,
    );
}

#[test]
fn rdf12_blank_node_not_shared_inside_and_outside_triple_term() {
    check(
        "unshared bnode",
        "_:x :name \"A\" . :a :says <<( _:x :p :o )>> .",
        "_:x :name \"A\" . :a :says <<( _:y :p :o )>> .",
        Different,
    );
}

#[test]
fn rdf12_nested_triple_terms_with_blank_nodes() {
    check(
        "nested triple terms",
        "_:w :says <<( :a :says <<( _:x :p _:w )>> )>> .",
        "_:v :says <<( :a :says <<( _:z :p _:v )>> )>> .",
        Isomorphic,
    );
}

#[test]
fn rdf12_nested_triple_terms_wrong_link() {
    check(
        "nested triple terms wrong link",
        "_:w :says <<( :a :says <<( _:x :p _:w )>> )>> .",
        "_:v :says <<( :a :says <<( _:v :p _:z )>> )>> .",
        Different,
    );
}

#[test]
fn rdf12_reified_triples_with_anonymous_reifiers() {
    check(
        "reifier",
        "<< :s :p :o >> :source :w .",
        "<< :s :p :o >> :source :w .",
        Isomorphic,
    );
}

#[test]
fn rdf12_annotation_vs_explicit_reification() {
    check(
        "annotation",
        ":s :p :o {| :source :w |} .",
        ":s :p :o . _:r rdf:reifies <<( :s :p :o )>> ; :source :w .",
        Isomorphic,
    );
}

#[test]
fn rdf12_asserted_triple_vs_triple_term() {
    check(
        "asserted vs triple term",
        ":s :p :o .",
        "_:r rdf:reifies <<( :s :p :o )>> .",
        Different,
    );
}

#[test]
fn rdf12_triple_term_subgraph() {
    check(
        "triple term subgraph",
        ":a :says <<( _:x :p :o )>> .",
        ":a :says <<( _:y :p :o )>> . _:y :name \"Y\" .",
        Subgraph,
    );
}

#[test]
fn rdf12_reifiers_subgraph_needs_backtracking() {
    check(
        "reifier backtracking",
        "_:r rdf:reifies <<( :s :p :o )>> ; :source :w .",
        "_:r1 rdf:reifies <<( :s :p :o )>> . _:r2 rdf:reifies <<( :s :p :o )>> ; :source :w .",
        Subgraph,
    );
}

#[test]
fn rdf12_directional_language_strings() {
    check(
        "dir lang",
        ":a :p \"Shalom\"@he--rtl .",
        ":a :p \"Shalom\"@he--ltr .",
        Different,
    );
}

// ---------------------------------------------------------------------------
// Subgraph modes: injective (default), homomorphic and simple entailment
// ---------------------------------------------------------------------------

/// Checks the expected relation for each mode, `[injective, homomorphic, entailment]`,
/// in both directions, and its consistency with `is_subgraph_of_with`.
fn check_modes(name: &str, first: &str, second: &str, expected: [GraphRelation; 3]) {
    let g1 = ttl(first);
    let g2 = ttl(second);
    for (mode, expected) in [Injective, Homomorphic, Entailment].into_iter().zip(expected) {
        assert_eq!(
            g1.compare_with_mode(&g2, mode),
            expected,
            "{name} [{mode}]: compare(first, second)"
        );
        assert_eq!(
            g2.compare_with_mode(&g1, mode),
            expected.inverse(),
            "{name} [{mode}]: compare(second, first)"
        );
        let sub = matches!(expected, Isomorphic | Equivalent | Subgraph);
        let sup = matches!(expected, Isomorphic | Equivalent | Supergraph);
        assert_eq!(
            g1.is_subgraph_of_with(&g2, mode),
            sub,
            "{name} [{mode}]: first in second"
        );
        assert_eq!(
            g2.is_subgraph_of_with(&g1, mode),
            sup,
            "{name} [{mode}]: second in first"
        );
    }
}

#[test]
fn equivalent_is_its_own_inverse() {
    assert_eq!(Equivalent.inverse(), Equivalent);
}

#[test]
fn subgraph_mode_parses_and_displays() {
    assert_eq!(SubgraphMode::default(), Injective);
    for mode in [Injective, Homomorphic, Entailment] {
        assert_eq!(mode.to_string().parse::<SubgraphMode>().unwrap(), mode);
    }
    assert_eq!("ENTAILMENT".parse::<SubgraphMode>().unwrap(), Entailment);
    assert!("subset".parse::<SubgraphMode>().is_err());
}

#[test]
fn modes_isomorphic_graphs_stay_isomorphic() {
    check_modes(
        "renamed",
        ":a :p _:x . _:x :q _:y .",
        ":a :p _:m . _:m :q _:n .",
        [Isomorphic, Isomorphic, Isomorphic],
    );
}

#[test]
fn modes_two_blank_nodes_vs_self_loop() {
    check_modes(
        "self loop",
        "_:x :p _:y .",
        "_:z :p _:z .",
        [Different, Subgraph, Subgraph],
    );
}

#[test]
fn modes_two_blank_nodes_into_one() {
    check_modes(
        "merge bnodes",
        "_:a :p :o . _:b :p :o .",
        "_:c :p :o . :s :q :o .",
        [Different, Subgraph, Subgraph],
    );
}

#[test]
fn modes_redundant_blank_node_is_equivalent() {
    // The second graph is not lean: `_:y :p :o` adds nothing to `_:x :p :o`
    check_modes(
        "lean",
        "_:x :p :o .",
        "_:x :p :o . _:y :p :o .",
        [Subgraph, Equivalent, Equivalent],
    );
}

#[test]
fn modes_blank_node_instantiated_by_iri() {
    check_modes(
        "bnode to iri",
        ":a :p _:x .",
        ":a :p :x .",
        [Different, Different, Subgraph],
    );
}

#[test]
fn modes_blank_node_instantiated_by_literal() {
    check_modes(
        "bnode to literal",
        ":a :p _:x .",
        ":a :p \"lit\" .",
        [Different, Different, Subgraph],
    );
}

#[test]
fn modes_blank_node_chain_entailed_by_iri_loop() {
    check_modes(
        "chain to loop",
        "_:a :p _:b . _:b :p _:c .",
        ":n :p :n .",
        [Different, Different, Subgraph],
    );
}

#[test]
fn modes_blank_node_must_be_instantiated_consistently() {
    check_modes(
        "consistent instantiation",
        "_:x :p :o . _:x :q :r .",
        ":a :p :o . :b :q :r .",
        [Different, Different, Different],
    );
}

#[test]
fn modes_generalization_holds_in_every_mode() {
    check_modes(
        "generalization",
        "_:x :p _:y .",
        "_:z :p _:w . _:w :p _:z .",
        [Subgraph, Subgraph, Subgraph],
    );
}

#[test]
fn modes_six_cycle_maps_onto_three_cycles() {
    // A 6-cycle wraps twice around a 3-cycle, but an odd cycle has no
    // homomorphism into an even one
    check_modes(
        "6-cycle vs 2x3-cycle",
        "_:a :p _:b . _:b :p _:c . _:c :p _:d . _:d :p _:e . _:e :p _:f . _:f :p _:a .",
        "_:a :p _:b . _:b :p _:c . _:c :p _:a . _:d :p _:e . _:e :p _:f . _:f :p _:d .",
        [Different, Subgraph, Subgraph],
    );
}

#[test]
fn modes_rdf12_blank_node_instantiated_by_triple_term() {
    check_modes(
        "bnode to triple term",
        ":a :says _:x .",
        ":a :says <<( :s :p :o )>> .",
        [Different, Different, Subgraph],
    );
}

#[test]
fn modes_rdf12_blank_node_inside_triple_term_instantiated() {
    check_modes(
        "bnode in triple term to iri",
        ":a :says <<( _:x :p :o )>> .",
        ":a :says <<( :s :p :o )>> .",
        [Different, Different, Subgraph],
    );
}

#[test]
fn modes_rdf12_duplicated_reifiers_are_equivalent() {
    check_modes(
        "duplicated reifier",
        "_:r1 rdf:reifies <<( :s :p :o )>> ; :source :w . _:r2 rdf:reifies <<( :s :p :o )>> ; :source :w .",
        "<< :s :p :o >> :source :w .",
        [Supergraph, Equivalent, Equivalent],
    );
}

// ---------------------------------------------------------------------------
// Property tests: random graphs over a small vocabulary
//
// They cross-check the two independent algorithms used by the implementation
// (canonicalization for isomorphism, backtracking search for subgraphs):
// for finite graphs, `G1 ≅ G2` iff `|G1| = |G2|` and `G1 ⊆ G2` up to renaming.
// ---------------------------------------------------------------------------

#[cfg(not(target_family = "wasm"))]
mod properties {
    use super::*;
    use oxrdf::{BlankNode, Graph, NamedNode, NamedOrBlankNode, NamedOrBlankNodeRef, Term, Triple};
    use proptest::prelude::*;

    const EX: &str = "http://example.org/";

    /// A node: 0..3 are IRIs, 3..8 are blank nodes.
    fn subject(n: u8, bnode_prefix: &str, perm: &[u8]) -> NamedOrBlankNode {
        if n < 3 {
            NamedNode::new_unchecked(format!("{EX}n{n}")).into()
        } else {
            BlankNode::new_unchecked(format!("{bnode_prefix}{}", perm[(n - 3) as usize])).into()
        }
    }

    /// Raw triple: (subject, predicate, object, object is a triple term built from the next 3 fields)
    type RawTriple = (u8, u8, u8, bool, u8, u8);

    fn build(raw: &[RawTriple], bnode_prefix: &str, perm: &[u8]) -> Graph {
        let mut g = Graph::new();
        for &(s, p, o, nested, ns, no) in raw {
            let pred = NamedNode::new_unchecked(format!("{EX}p{p}"));
            let object: Term = if nested {
                Triple::new(
                    subject(ns, bnode_prefix, perm),
                    NamedNode::new_unchecked(format!("{EX}p0")),
                    Term::from(subject(no, bnode_prefix, perm)),
                )
                .into()
            } else {
                subject(o, bnode_prefix, perm).into()
            };
            g.insert(&Triple::new(subject(s, bnode_prefix, perm), pred, object));
        }
        g
    }

    fn raw_triples() -> impl Strategy<Value = Vec<RawTriple>> {
        prop::collection::vec(
            (0u8..8, 0u8..2, 0u8..8, prop::bool::weighted(0.2), 0u8..8, 0u8..8),
            0..10,
        )
    }

    fn permutation() -> impl Strategy<Value = Vec<u8>> {
        Just((0u8..5).collect::<Vec<_>>()).prop_shuffle()
    }

    const IDENTITY: [u8; 5] = [0, 1, 2, 3, 4];

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Renaming blank nodes (and reordering triples) gives an isomorphic graph
        #[test]
        fn renaming_gives_isomorphic_graph(raw in raw_triples(), perm in permutation()) {
            let g1 = build(&raw, "a", &IDENTITY);
            let mut reversed = raw.clone();
            reversed.reverse();
            let g2 = build(&reversed, "b", &perm);
            prop_assert!(crate::rdf_impl::is_isomorphic(&g1, &g2));
            prop_assert_eq!(crate::rdf_impl::compare_graphs(&g1, &g2), Isomorphic);
        }

        /// Removing a triple from a renamed copy gives a proper subgraph
        #[test]
        fn removing_a_triple_gives_subgraph(raw in raw_triples(), perm in permutation(), idx in any::<prop::sample::Index>()) {
            let g1 = build(&raw, "a", &IDENTITY);
            prop_assume!(!g1.is_empty());
            let g2 = build(&raw, "b", &perm);
            let victim = g2.iter().nth(idx.index(g2.len())).unwrap().into_owned();
            let mut smaller = g2.clone();
            smaller.remove(&victim);
            prop_assert!(crate::rdf_impl::is_subgraph(&smaller, &g1));
            prop_assert_eq!(crate::rdf_impl::compare_graphs(&smaller, &g1), Subgraph);
            prop_assert_eq!(crate::rdf_impl::compare_graphs(&g1, &smaller), Supergraph);
        }

        /// Both algorithms agree on arbitrary pairs of graphs
        #[test]
        fn isomorphism_agrees_with_mutual_subgraph(raw1 in raw_triples(), raw2 in raw_triples()) {
            let g1 = build(&raw1, "a", &IDENTITY);
            let g2 = build(&raw2, "b", &IDENTITY);
            let iso = crate::rdf_impl::is_isomorphic(&g1, &g2);
            let sub = g1.len() == g2.len() && crate::rdf_impl::is_subgraph(&g1, &g2);
            prop_assert_eq!(iso, sub);
            let rel = crate::rdf_impl::compare_graphs(&g1, &g2);
            prop_assert_eq!(crate::rdf_impl::compare_graphs(&g2, &g1), rel.inverse());
        }
    }

    // Mode properties
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Each mode is more permissive than the previous one
        #[test]
        fn modes_are_increasingly_permissive(raw1 in raw_triples(), raw2 in raw_triples()) {
            let g1 = build(&raw1, "a", &IDENTITY);
            let g2 = build(&raw2, "b", &IDENTITY);
            let inj = crate::rdf_impl::is_subgraph_with(&g1, &g2, Injective);
            let hom = crate::rdf_impl::is_subgraph_with(&g1, &g2, Homomorphic);
            let ent = crate::rdf_impl::is_subgraph_with(&g1, &g2, Entailment);
            prop_assert_eq!(inj, crate::rdf_impl::is_subgraph(&g1, &g2));
            prop_assert!(!inj || hom, "injective but not homomorphic");
            prop_assert!(!hom || ent, "homomorphic but not entailed");
            for mode in [Injective, Homomorphic, Entailment] {
                let rel = crate::rdf_impl::compare_graphs_with(&g1, &g2, mode);
                prop_assert_eq!(crate::rdf_impl::compare_graphs_with(&g2, &g1, mode), rel.inverse());
            }
        }

        /// A graph merged with a renamed copy of itself is equivalent to it
        #[test]
        fn graph_merged_with_renamed_copy_is_equivalent(raw in raw_triples(), perm in permutation()) {
            let g = build(&raw, "a", &IDENTITY);
            let mut merged = g.clone();
            for t in build(&raw, "b", &perm).iter() {
                merged.insert(t);
            }
            for mode in [Homomorphic, Entailment] {
                let rel = crate::rdf_impl::compare_graphs_with(&g, &merged, mode);
                prop_assert!(matches!(rel, Isomorphic | Equivalent), "{mode}: {rel}");
            }
        }

        /// Replacing blank nodes by IRIs gives a graph that entails the original
        #[test]
        fn instance_entails_original(raw in raw_triples(), to_iri in prop::collection::vec(any::<bool>(), 5)) {
            let g = build(&raw, "a", &IDENTITY);
            // Blank node `aN` becomes IRI `nN` (a node of the vocabulary) when `to_iri[N]`
            let mut instance = Graph::new();
            for t in g.iter() {
                instance.insert(&instantiate(t, &to_iri));
            }
            prop_assert!(crate::rdf_impl::is_subgraph_with(&g, &instance, Entailment));
        }
    }

    fn instantiate_node(n: NamedOrBlankNodeRef<'_>, to_iri: &[bool]) -> NamedOrBlankNode {
        match n {
            NamedOrBlankNodeRef::BlankNode(b) => {
                let i: usize = b.as_str()[1..].parse().unwrap();
                if to_iri[i] {
                    NamedNode::new_unchecked(format!("{EX}n{}", i % 3)).into()
                } else {
                    b.into_owned().into()
                }
            },
            other => other.into_owned(),
        }
    }

    fn instantiate(t: oxrdf::TripleRef<'_>, to_iri: &[bool]) -> Triple {
        let object: Term = match t.object {
            oxrdf::TermRef::BlankNode(b) => Term::from(instantiate_node(b.into(), to_iri)),
            oxrdf::TermRef::Triple(inner) => instantiate(inner.as_ref(), to_iri).into(),
            other => other.into_owned(),
        };
        Triple::new(instantiate_node(t.subject, to_iri), t.predicate.into_owned(), object)
    }

    // Random pairs are mostly `Different`: also generate pairs that differ in
    // a single triple, which exercise near-misses.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        #[test]
        fn near_miss_pairs_agree(raw in raw_triples(), perm in permutation(), extra in (0u8..8, 0u8..2, 0u8..8, Just(false), Just(0u8), Just(0u8))) {
            let g1 = build(&raw, "a", &IDENTITY);
            let mut raw2 = raw.clone();
            if !raw2.is_empty() { raw2.pop(); }
            raw2.push(extra);
            let g2 = build(&raw2, "b", &perm);
            let iso = crate::rdf_impl::is_isomorphic(&g1, &g2);
            let sub = g1.len() == g2.len() && crate::rdf_impl::is_subgraph(&g1, &g2);
            prop_assert_eq!(iso, sub);
        }
    }
}
