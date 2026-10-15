use std::str::FromStr;

use crate::{
    Rudof, RudofConfig,
    formats::{InputSpec, NodeInspectionMode},
    types::NeighborArc,
};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::{ArcDirection, term::Object};

const GRAPH: &str = r#"
        prefix : <http://example.org/>
        :a :p :b ;
           :r :c .
        :b :q :c ;
           :s 1 .
        :e :p :a .
    "#;

fn iri(local: &str) -> IriS {
    IriS::new_unchecked(&format!("http://example.org/{local}"))
}

fn node(local: &str) -> Object {
    Object::iri(iri(local))
}

#[test]
fn test_node_neighborhood_incoming_outgoing_depth_1() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let data = InputSpec::from_str(GRAPH).unwrap();

    rudof.load_data().with_data(&[data]).execute().unwrap();

    let mut neighborhood = rudof
        .node_neighborhood("<http://example.org/a>")
        .with_mode(&NodeInspectionMode::Both)
        .execute()
        .unwrap();

    let arc1 = neighborhood.next().unwrap().unwrap();
    assert_eq!(
        arc1,
        NeighborArc {
            root: node("a"),
            direction: ArcDirection::Outgoing,
            depth: 1,
            node: node("a"),
            predicate: iri("p"),
            neighbor: node("b"),
            is_last: false
        }
    );

    let arc2 = neighborhood.next().unwrap().unwrap();
    assert_eq!(
        arc2,
        NeighborArc {
            root: node("a"),
            direction: ArcDirection::Outgoing,
            depth: 1,
            node: node("a"),
            predicate: iri("r"),
            neighbor: node("c"),
            is_last: true
        }
    );

    let arc3 = neighborhood.next().unwrap().unwrap();
    assert_eq!(
        arc3,
        NeighborArc {
            root: node("a"),
            direction: ArcDirection::Incoming,
            depth: 1,
            node: node("a"),
            predicate: iri("p"),
            neighbor: node("e"),
            is_last: true
        }
    );

    assert!(neighborhood.next().is_none());
}

#[test]
fn test_node_neighborhood_resolves_relative_iris_against_the_base() {
    let config = RudofConfig::from_str("base_iri = \"http://example.org/\"\n").unwrap();
    let mut rudof = Rudof::new(config);
    let data = InputSpec::from_str(GRAPH).unwrap();
    rudof.load_data().with_data(&[data]).execute().unwrap();

    // The node and the predicates: a relative IRI used to be looked up as it was
    // written, which matches nothing in a graph whose IRIs are absolute.
    let arcs: Vec<NeighborArc> = rudof
        .node_neighborhood("<a>")
        .with_predicates(&["<p>".to_string()])
        .with_mode(&NodeInspectionMode::Outgoing)
        .execute()
        .unwrap()
        .map(Result::unwrap)
        .collect();

    assert_eq!(arcs.len(), 1);
    assert_eq!(arcs[0].node, node("a"));
    assert_eq!(arcs[0].predicate, iri("p"));
    assert_eq!(arcs[0].neighbor, node("b"));
}
