#![cfg(not(target_family = "wasm"))]

mod common;

use common::graph_of;
use common::{DOCUMENT, TOOL, submission_report, submission_report_graph, turtle};
use rudof_earl::model::{Assertion, TestResult};
use rudof_earl::report::EarlReport;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;

#[test]
fn a_submission_report_matches_the_published_shape() {
    let built = graph_of(&submission_report());
    let expected = submission_report_graph();

    assert!(
        built.is_isomorphic(&expected),
        "the built report is not isomorphic to the expected one, the REPORT const in \
         tests/common/report.rs.\n\
         If the change is deliberate, update that const and review the diff.\n\
         --- built ({} triples) ---\n{}\n--- expected ({} triples) ---\n{}",
        built.len(),
        turtle(&built),
        expected.len(),
        turtle(&expected),
    );
}

#[test]
fn a_report_missing_an_assertion_does_not_match() {
    let incomplete = EarlReport::new()
        .with_document(IriS::new_unchecked(DOCUMENT))
        .with_assertion(Assertion::new(
            Object::iri(IriS::new_unchecked(TOOL)),
            Object::iri(IriS::new_unchecked("urn:x-shacl-test:/core/node/disjoint-001")),
            TestResult::passed(),
        ));

    assert!(!graph_of(&incomplete).is_isomorphic(&submission_report_graph()));
}
