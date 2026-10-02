//! Round-trip tests for [`BuildRDF::add_shacl_path`]: every `SHACLPath` variant
//! must survive being written to a graph and read back by
//! [`FocusRDF::get_path_for`], which is its inverse.

use crate::rdf_core::vocabs::ShaclVocab;
use crate::rdf_core::{BuildRDF, FocusRDF, Rdf, SHACLPath};
use crate::rdf_impl::OxigraphInMemory;
use rudof_iri::IriS;

fn assert_path_round_trip(path: SHACLPath) {
    let mut graph = OxigraphInMemory::empty();
    let node = graph.add_bnode().unwrap();
    let term = graph.add_shacl_path(&path).unwrap();
    graph.add_triple(node.clone(), ShaclVocab::sh_path(), term).unwrap();

    let subject: <OxigraphInMemory as Rdf>::Term = node.into();
    let predicate: <OxigraphInMemory as Rdf>::IRI = ShaclVocab::sh_path().into();
    let parsed = graph.get_path_for(&subject, &predicate).unwrap().unwrap();
    assert_eq!(parsed, path);
}

fn pred(local: &str) -> SHACLPath {
    SHACLPath::iri(IriS::new_unchecked(&format!("http://example.org/{local}")))
}

#[test]
fn add_shacl_path_predicate() {
    assert_path_round_trip(pred("knows"));
}

#[test]
fn add_shacl_path_inverse() {
    assert_path_round_trip(SHACLPath::inverse(pred("knows")));
}

#[test]
fn add_shacl_path_zero_or_more() {
    assert_path_round_trip(SHACLPath::zero_or_more(pred("knows")));
}

#[test]
fn add_shacl_path_one_or_more() {
    assert_path_round_trip(SHACLPath::one_or_more(pred("knows")));
}

#[test]
fn add_shacl_path_zero_or_one() {
    assert_path_round_trip(SHACLPath::zero_or_one(pred("knows")));
}

#[test]
fn add_shacl_path_sequence() {
    assert_path_round_trip(SHACLPath::sequence(vec![pred("knows"), pred("name")]));
}

#[test]
fn add_shacl_path_alternative() {
    assert_path_round_trip(SHACLPath::alternative(vec![pred("knows"), pred("name")]));
}

#[test]
fn add_shacl_path_nested() {
    assert_path_round_trip(SHACLPath::sequence(vec![
        SHACLPath::alternative(vec![pred("knows"), SHACLPath::inverse(pred("name"))]),
        SHACLPath::zero_or_more(pred("friend")),
    ]));
}

#[test]
fn add_rdf_list_empty_is_nil() {
    let mut graph = OxigraphInMemory::empty();
    let head = graph.add_rdf_list(Vec::new()).unwrap();
    assert_eq!(
        head.to_string(),
        format!("<{}>", crate::rdf_core::vocabs::RdfVocab::RDF_NIL)
    );
}
