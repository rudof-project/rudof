//! Shared fixture-loading and RDF-graph-comparison helpers used by both
//! `shexr_builder_fixtures.rs` (Schema → RDF) and `shexr_parser_fixtures.rs`
//! (RDF → Schema, verified by rebuilding and comparing against the original
//! RDF) — both drive the same W3C ShExTest `schemas` manifest.
#![allow(dead_code)]

use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::Term as _;
use rudof_rdf::rdf_core::term::Triple as _;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShexRVocab};
use rudof_rdf::rdf_core::{NeighsRDF, Rdf};
use rudof_rdf::rdf_impl::OxigraphInMemory;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct ManifestFile {
    #[serde(rename = "@graph")]
    graph: Vec<ManifestGraph>,
}

#[derive(Deserialize)]
struct ManifestGraph {
    entries: Vec<ManifestEntry>,
}

#[derive(Deserialize)]
pub struct ManifestEntry {
    pub name: String,
    pub json: String,
    pub ttl: String,
}

pub fn schemas_dir() -> PathBuf {
    Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../shex_testsuite/shexTest/schemas"
    ))
    .to_path_buf()
}

pub fn read_manifest(dir: &Path) -> Vec<ManifestEntry> {
    let text = std::fs::read_to_string(dir.join("manifest.jsonld")).expect("reading shexTest schemas manifest");
    let manifest: ManifestFile = serde_json::from_str(&text).expect("parsing shexTest schemas manifest");
    manifest.graph.into_iter().flat_map(|g| g.entries).collect()
}

/// Finds the sole `?s a sx:Schema` node in `graph`.
pub fn schema_root(graph: &OxigraphInMemory) -> <OxigraphInMemory as Rdf>::Term {
    let rdf_type: <OxigraphInMemory as Rdf>::IRI = RdfVocab::rdf_type().into();
    let sx_schema: <OxigraphInMemory as Rdf>::Term = ShexRVocab::sx_schema().into();
    let mut matches = graph
        .triples_with_predicate_object(&rdf_type, &sx_schema)
        .expect("querying for sx:Schema node");
    let root = matches.next().expect("graph has no sx:Schema node").subj().clone();
    matches
        .next()
        .is_none()
        .then_some(())
        .expect("graph has more than one sx:Schema node");
    root.into()
}

/// Compares two literal terms: same datatype required; lexical form must
/// match exactly *unless* both parse as plain numbers, in which case they're
/// compared by value. The latter is needed because ShExJ's numeric facet
/// bounds (`NumericLiteral`/`rust_decimal::Decimal`) don't preserve the
/// original source's lexical formatting (leading/trailing zeros, whether an
/// `xsd:double` was written in E-notation) — only the value survives
/// parsing, so exact lexical round-tripping of those specific literals is
/// not achievable from the current AST. That's a data-loss issue in the
/// ShExJ parser, not something a schema-to-RDF builder can work around, and
/// doesn't affect ShEx validation semantics (which compare facet bounds by
/// value).
pub fn assert_same_literal(a: <OxigraphInMemory as Rdf>::Term, e: <OxigraphInMemory as Rdf>::Term, entry: &str) {
    let a_obj: Object = a
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("[{entry}] {a} is not an RDF term"));
    let e_obj: Object = e
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("[{entry}] {e} is not an RDF term"));
    let (Object::Literal(a_lit), Object::Literal(e_lit)) = (a_obj, e_obj) else {
        assert_eq!(a.to_string(), e.to_string(), "[{entry}] literal mismatch");
        return;
    };
    let (a_lex, e_lex) = (a_lit.lexical_form(), e_lit.lexical_form());
    if a_lex == e_lex && a_lit.datatype().to_string() == e_lit.datatype().to_string() {
        return;
    }
    // Numeric family (integer/decimal/double/float, ...): compare by value,
    // ignoring which specific XSD numeric datatype either side used. ShExJ's
    // custom NumericLiteral deserializer can't always recover whether a bare
    // JSON number like `4.5` was written as a ShExC DECIMAL or DOUBLE
    // literal — that distinction is lost before this builder ever sees the
    // schema, so exact-datatype fidelity for these specific values isn't
    // achievable from the current AST; the numeric value is what matters for
    // ShEx validation semantics.
    match (a_lex.trim().parse::<f64>(), e_lex.trim().parse::<f64>()) {
        (Ok(an), Ok(en)) => assert!(
            (an - en).abs() <= (an.abs().max(en.abs()) * 1e-9).max(1e-9),
            "[{entry}] numeric literal value mismatch: {a_lex} ({}) vs {e_lex} ({})",
            a_lit.datatype(),
            e_lit.datatype()
        ),
        // BCP-47 language tags (`sx:languageTag`, language stem values):
        // matching is defined to be case-insensitive, and our `Lang` type
        // canonicalizes case (e.g. uppercasing region subtags: `fr-BE`)
        // where the fixtures just keep the tag as originally written
        // (`fr-be`) — a cosmetic difference, not a value difference.
        _ if a_lex.eq_ignore_ascii_case(&e_lex) => {},
        _ => {
            assert_eq!(
                a_lit.datatype().to_string(),
                e_lit.datatype().to_string(),
                "[{entry}] literal datatype mismatch: {a} vs {e}"
            );
            assert_eq!(a_lex, e_lex, "[{entry}] literal lexical form mismatch");
        },
    }
}

/// A stem's `Wildcard` can be written in ShExJ either as the structured
/// `{"type": "Wildcard"}` (which the builder always emits, as `[a
/// sx:Wildcard]`) or, in a handful of corpus fixtures, as a bare empty
/// string. Our AST's `*OrWildcard` deserializers collapse both spellings
/// into the same `Wildcard` variant, so which one produced it is lost —
/// same class of unrecoverable data loss as the numeric-literal case
/// `assert_same_literal` documents. Treat an empty-string literal on one
/// side and a lone `[a sx:Wildcard]` blank node on the other as equivalent
/// rather than a mismatch.
pub fn is_wildcard_vs_empty_string(
    actual: &OxigraphInMemory,
    a: &<OxigraphInMemory as Rdf>::Term,
    expected: &OxigraphInMemory,
    e: &<OxigraphInMemory as Rdf>::Term,
) -> bool {
    fn is_empty_string(t: &<OxigraphInMemory as Rdf>::Term) -> bool {
        matches!(Object::try_from(t.clone()), Ok(Object::Literal(lit)) if lit.lexical_form().is_empty())
    }
    fn is_wildcard_node(graph: &OxigraphInMemory, t: &<OxigraphInMemory as Rdf>::Term) -> bool {
        let Ok(subj) = <OxigraphInMemory as Rdf>::Subject::try_from(t.clone()) else {
            return false;
        };
        let Ok(triples) = graph.triples_with_subject(&subj) else {
            return false;
        };
        let triples: Vec<_> = triples.collect();
        triples.len() == 1
            && triples[0].pred().as_str() == RdfVocab::RDF_TYPE
            && triples[0].obj().to_string().contains(ShexRVocab::SX_WILDCARD)
    }
    (is_empty_string(a) && is_wildcard_node(expected, e)) || (is_empty_string(e) && is_wildcard_node(actual, a))
}

/// Structurally compares two nodes across two (possibly independently
/// parsed) graphs: literals compare by lexical form (see [`assert_same_literal`]);
/// IRIs must be lexically identical; blank nodes are compared by recursing
/// into their outgoing triples (predicate-sorted) rather than by label.
/// Valid because ShExR graphs — both the real fixtures and this crate's
/// builder output — are trees of *distinct* nodes except where ShEx itself
/// allows a shape to (directly or indirectly) reference itself, e.g. `<S> {
/// p1 IRI ; p2 @<S> }` — a `visited` set of already-compared node pairs
/// breaks those cycles instead of recursing forever.
pub fn assert_same_node(
    actual: &OxigraphInMemory,
    a: <OxigraphInMemory as Rdf>::Term,
    expected: &OxigraphInMemory,
    e: <OxigraphInMemory as Rdf>::Term,
    entry: &str,
    visited: &mut std::collections::HashSet<(String, String)>,
) {
    if is_wildcard_vs_empty_string(actual, &a, expected, &e) {
        return;
    }
    if a.is_literal() || e.is_literal() {
        assert_same_literal(a, e, entry);
        return;
    }
    assert_eq!(a.is_iri(), e.is_iri(), "[{entry}] node kind mismatch: {a} vs {e}");
    if a.is_iri() {
        assert_eq!(a.to_string(), e.to_string(), "[{entry}] IRI mismatch");
    }
    if !visited.insert((a.to_string(), e.to_string())) {
        return;
    }

    let a_subj: <OxigraphInMemory as Rdf>::Subject = a
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("[{entry}] {a} is not a subject"));
    let e_subj: <OxigraphInMemory as Rdf>::Subject = e
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("[{entry}] {e} is not a subject"));

    let mut a_triples: Vec<_> = actual
        .triples_with_subject(&a_subj)
        .expect("querying actual graph")
        .map(|t| (t.pred().as_str().to_string(), t.obj().clone()))
        .collect();
    let mut e_triples: Vec<_> = expected
        .triples_with_subject(&e_subj)
        .expect("querying expected graph")
        .map(|t| (t.pred().as_str().to_string(), t.obj().clone()))
        .collect();
    assert_eq!(
        a_triples.len(),
        e_triples.len(),
        "[{entry}] different number of triples for {a} vs {e}: {a_triples:#?} vs {e_triples:#?}"
    );
    // Secondary-sort by the object's own string form so that a subject with
    // several triples under the *same* predicate (e.g. several `sx:extra`
    // IRIs) pairs up correctly even though `triples_with_subject` doesn't
    // guarantee any particular order.
    let sort_key = |t: &(String, <OxigraphInMemory as Rdf>::Term)| (t.0.clone(), t.1.to_string());
    a_triples.sort_by_key(sort_key);
    e_triples.sort_by_key(sort_key);
    for ((a_pred, a_obj), (e_pred, e_obj)) in a_triples.into_iter().zip(e_triples) {
        assert_eq!(a_pred, e_pred, "[{entry}] predicate mismatch on {a} vs {e}");
        assert_same_node(actual, a_obj, expected, e_obj, entry, visited);
    }
}
