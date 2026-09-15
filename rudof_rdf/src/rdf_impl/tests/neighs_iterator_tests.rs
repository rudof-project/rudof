use crate::rdf_core::{ArcDirection, Neigh, NeighArc, NeighsIterator, RDFFormat};
use crate::rdf_impl::{OxigraphInMemory, ReaderMode};
use oxrdf::NamedNode as OxNamedNode;

const GRAPH: &str = r#"
        prefix : <http://example.org/>
        :a :p :b ;
           :r :c .
        :b :q :c ;
           :s 1 .
        :e :p :a .
    "#;

fn iri(local: &str) -> OxNamedNode {
    OxNamedNode::new_unchecked(format!("http://example.org/{local}"))
}

#[test]
fn test_incoming_outgoing_neighs_depth_1() {
    let graph = OxigraphInMemory::from_str(GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

    let mut neighs =
        NeighsIterator::new(&graph, iri("a").into()).with_directions(&[ArcDirection::Outgoing, ArcDirection::Incoming]);

    let arc1 = neighs.next().unwrap().unwrap();
    assert_eq!(
        arc1,
        NeighArc {
            depth: 1,
            node: iri("a").into(),
            neigh: Neigh::direct(iri("p"), iri("b").into()),
            is_last: false
        }
    );

    let arc2 = neighs.next().unwrap().unwrap();
    assert_eq!(
        arc2,
        NeighArc {
            depth: 1,
            node: iri("a").into(),
            neigh: Neigh::direct(iri("r"), iri("c").into()),
            is_last: true
        }
    );

    let arc3 = neighs.next().unwrap().unwrap();
    assert_eq!(
        arc3,
        NeighArc {
            depth: 1,
            node: iri("a").into(),
            neigh: Neigh::inverse(iri("p"), iri("e").into()),
            is_last: true
        }
    );

    assert!(neighs.next().is_none());
}
