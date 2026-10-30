//! A cyclic or exploding SHACL path is rejected instead of overflowing the stack or running out of memory.

use crate::rdf_core::{FocusRDF, RDFError, RDFFormat, SHACLPath};
use crate::rdf_impl::{OxigraphInMemory, ReaderMode};
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::Term as OxTerm;
use rudof_iri::IriS;

/// Parses `_:p` as the `sh:path` of `:s`, given the triples in `paths`.
fn parse_path(paths: &str) -> Result<Option<SHACLPath>, RDFError> {
    let ttl = format!(
        "prefix : <http://example.org/>
        prefix sh: <http://www.w3.org/ns/shacl#>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        :s sh:path _:p .
        {paths}"
    );
    let mut graph = OxigraphInMemory::from_str(&ttl, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    let s: OxTerm = OxNamedNode::new_unchecked("http://example.org/s").into();
    let sh_path = OxNamedNode::new_unchecked("http://www.w3.org/ns/shacl#path");
    graph.get_path_for(&s, &sh_path)
}

#[test]
fn cyclic_paths_are_rejected() {
    for cyclic in [
        "_:p sh:zeroOrMorePath _:p .",
        "_:p sh:inversePath [ sh:zeroOrOnePath _:p ] .",
        "_:p rdf:first :q ; rdf:rest ( _:p ) .",
        "_:p sh:alternativePath ( :q _:p ) .",
    ] {
        let err = parse_path(cyclic).unwrap_err().to_string();
        assert!(err.contains("is cyclic"), "{cyclic}: {err}");
    }
}

#[test]
fn exploding_shared_path_is_rejected() {
    // _:l1 is ( :q :q ) and each _:lN is ( _:lN-1 _:lN-1 ), so _:l30 has 2^30 steps
    let mut paths = "_:p sh:zeroOrMorePath _:l30 . _:l1 rdf:first :q ; rdf:rest ( :q ) .".to_string();
    for n in 2..=30 {
        let m = n - 1;
        paths.push_str(&format!(" _:l{n} rdf:first _:l{m} ; rdf:rest ( _:l{m} ) ."));
    }
    let err = parse_path(&paths).unwrap_err().to_string();
    assert!(err.contains("more than 4096 steps plus 16 per triple"), "{err}");
}

#[test]
fn long_alternative_path_is_parsed() {
    let preds: String = (0..5000).map(|i| format!(" :p{i}")).collect();
    let path = parse_path(&format!("_:p sh:alternativePath ({preds} ) .")).unwrap();
    assert!(matches!(path, Some(SHACLPath::Alternative { paths }) if paths.len() == 5000));
}

#[test]
fn shared_sub_path_is_parsed() {
    let path = parse_path("_:p sh:alternativePath ( _:q _:q ) . _:q sh:inversePath :r .").unwrap();
    let r = SHACLPath::inverse(SHACLPath::iri(IriS::new_unchecked("http://example.org/r")));
    assert_eq!(path, Some(SHACLPath::alternative(vec![r.clone(), r])));
}
