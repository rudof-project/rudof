//! Tests of the `compare_data` API. The comparison algorithms themselves are
//! tested in `rudof_rdf` (`graph_comparison_tests`); these check the plumbing:
//! inputs, formats, base IRIs, errors and the loaded data being left alone.

use crate::{
    Rudof, RudofConfig,
    formats::{DataFormat, InputSpec},
    types::{GraphRelation, SubgraphMode},
};

fn rudof() -> Rudof {
    Rudof::new(RudofConfig::default())
}

const TTL: &str = r#"
PREFIX : <http://example.org/>
:alice :knows [ :name "Bob" ] .
"#;

#[test]
fn compare_isomorphic_turtle_graphs() {
    let rudof = rudof();
    let data1 = InputSpec::str(TTL);
    let data2 = InputSpec::str("PREFIX ex: <http://example.org/> _:x ex:name \"Bob\" . ex:alice ex:knows _:x .");
    let relation = rudof.compare_data(&data1, &data2).execute().unwrap();
    assert_eq!(relation, GraphRelation::Isomorphic);
}

#[test]
fn compare_subgraph_and_supergraph() {
    let rudof = rudof();
    let small = InputSpec::str(TTL);
    let big = InputSpec::str(&format!("{TTL}\n:alice :name \"Alice\" ."));
    assert_eq!(
        rudof.compare_data(&small, &big).execute().unwrap(),
        GraphRelation::Subgraph
    );
    assert_eq!(
        rudof.compare_data(&big, &small).execute().unwrap(),
        GraphRelation::Supergraph
    );
}

#[test]
fn compare_different_graphs() {
    let rudof = rudof();
    let data1 = InputSpec::str(TTL);
    let data2 = InputSpec::str("PREFIX : <http://example.org/> :alice :knows [ :name \"Carol\" ] .");
    assert_eq!(
        rudof.compare_data(&data1, &data2).execute().unwrap(),
        GraphRelation::Different
    );
}

#[test]
fn compare_turtle_with_ntriples() {
    let rudof = rudof();
    let data1 = InputSpec::str(TTL);
    let data2 = InputSpec::str(
        "<http://example.org/alice> <http://example.org/knows> _:b0 .\n_:b0 <http://example.org/name> \"Bob\" .\n",
    );
    let relation = rudof
        .compare_data(&data1, &data2)
        .with_format2(&DataFormat::NTriples)
        .execute()
        .unwrap();
    assert_eq!(relation, GraphRelation::Isomorphic);
}

#[test]
fn compare_rdf12_triple_terms() {
    let rudof = rudof();
    let data1 = InputSpec::str("PREFIX : <http://example.org/> :s :p :o {| :source :w |} .");
    let data2 = InputSpec::str(
        "<http://example.org/s> <http://example.org/p> <http://example.org/o> .\n\
         _:r <http://www.w3.org/1999/02/22-rdf-syntax-ns#reifies> <<( <http://example.org/s> <http://example.org/p> <http://example.org/o> )>> .\n\
         _:r <http://example.org/source> <http://example.org/w> .\n",
    );
    let relation = rudof
        .compare_data(&data1, &data2)
        .with_format2(&DataFormat::NTriples)
        .execute()
        .unwrap();
    assert_eq!(relation, GraphRelation::Isomorphic);
}

#[test]
fn bases_resolve_relative_iris_per_graph() {
    let rudof = rudof();
    let data1 = InputSpec::str("<a> <p> <b> .");
    let data2 = InputSpec::str("<http://one.example/a> <http://one.example/p> <http://one.example/b> .");
    let same = rudof
        .compare_data(&data1, &data2)
        .with_base1("http://one.example/")
        .execute()
        .unwrap();
    assert_eq!(same, GraphRelation::Isomorphic);
    let other = rudof
        .compare_data(&data1, &data2)
        .with_base1("http://two.example/")
        .execute()
        .unwrap();
    assert_eq!(other, GraphRelation::Different);
}

#[test]
fn invalid_rdf_is_an_error() {
    let rudof = rudof();
    let data1 = InputSpec::str(TTL);
    let data2 = InputSpec::str(":not valid turtle");
    assert!(rudof.compare_data(&data1, &data2).execute().is_err());
}

#[test]
fn non_rdf_format_is_an_error() {
    let rudof = rudof();
    let data = InputSpec::str(TTL);
    let result = rudof.compare_data(&data, &data).with_format(&DataFormat::Pg).execute();
    assert!(result.is_err());
}

#[test]
fn loaded_data_is_left_untouched() {
    let mut rudof = rudof();
    let loaded = [InputSpec::str("PREFIX : <http://example.org/> :x :y :z .")];
    rudof.load_data().with_data(&loaded).execute().unwrap();
    let before = rudof.triples().execute().unwrap().count();

    let data1 = InputSpec::str(TTL);
    rudof.compare_data(&data1, &data1).execute().unwrap();

    assert_eq!(rudof.triples().execute().unwrap().count(), before);
}

// Subgraph modes

#[test]
fn mode_defaults_to_injective() {
    let rudof = rudof();
    let first = InputSpec::str("PREFIX : <http://example.org/> _:x :p _:y .");
    let second = InputSpec::str("PREFIX : <http://example.org/> _:z :p _:z .");
    assert_eq!(
        rudof.compare_data(&first, &second).execute().unwrap(),
        GraphRelation::Different
    );
    assert_eq!(
        rudof
            .compare_data(&first, &second)
            .with_mode(&SubgraphMode::Homomorphic)
            .execute()
            .unwrap(),
        GraphRelation::Subgraph
    );
}

#[test]
fn entailment_mode_instantiates_blank_nodes() {
    let rudof = rudof();
    let general = InputSpec::str("PREFIX : <http://example.org/> :alice :knows [ :name \"Bob\" ] .");
    let specific = InputSpec::str("PREFIX : <http://example.org/> :alice :knows :bob . :bob :name \"Bob\" .");
    let compare = |mode: SubgraphMode| {
        rudof
            .compare_data(&general, &specific)
            .with_mode(&mode)
            .execute()
            .unwrap()
    };
    assert_eq!(compare(SubgraphMode::Injective), GraphRelation::Different);
    assert_eq!(compare(SubgraphMode::Homomorphic), GraphRelation::Different);
    assert_eq!(compare(SubgraphMode::Entailment), GraphRelation::Subgraph);
}

#[test]
fn non_lean_graph_is_equivalent_with_homomorphic_mode() {
    let rudof = rudof();
    let lean = InputSpec::str("PREFIX : <http://example.org/> :a :p [] .");
    let redundant = InputSpec::str("PREFIX : <http://example.org/> :a :p [], [] .");
    let relation = rudof
        .compare_data(&lean, &redundant)
        .with_mode(&SubgraphMode::Homomorphic)
        .execute()
        .unwrap();
    assert_eq!(relation, GraphRelation::Equivalent);
}

// Comparing with the loaded data

#[test]
fn compare_loaded_data_with_input() {
    let mut rudof = rudof();
    let loaded = [InputSpec::str(TTL)];
    rudof.load_data().with_data(&loaded).execute().unwrap();
    let bigger = InputSpec::str(&format!("{TTL}\n:alice :name \"Alice\" ."));
    assert_eq!(
        rudof.compare_loaded_data(&bigger).execute().unwrap(),
        GraphRelation::Subgraph
    );
    let renamed = InputSpec::str("PREFIX ex: <http://example.org/> ex:alice ex:knows _:b . _:b ex:name \"Bob\" .");
    assert_eq!(
        rudof.compare_loaded_data(&renamed).execute().unwrap(),
        GraphRelation::Isomorphic
    );
}

#[test]
fn compare_loaded_data_without_data_is_an_error() {
    let rudof = rudof();
    let other = InputSpec::str(TTL);
    assert!(rudof.compare_loaded_data(&other).execute().is_err());
}
