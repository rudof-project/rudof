mod common;

use crate::common::{ORG, TEST, TOOL, submission_report};
use common::{count_predicate, serialize, triples_of};
use rudof_earl::model::{Assertion, Outcome, TestMode, TestResult};
use rudof_earl::report::{Assertor, EarlReport};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;

#[test]
fn an_assertion_carries_subject_test_and_assertor() {
    let triples = triples_of(&submission_report());

    assert!(
        triples
            .iter()
            .any(|(_, p, o)| p == "<http://www.w3.org/ns/earl#subject>" && o == &format!("<{TOOL}>")),
        "earl:subject should point at the tool, got {triples:#?}"
    );
    assert!(
        triples
            .iter()
            .any(|(_, p, o)| p == "<http://www.w3.org/ns/earl#test>" && o == &format!("<{TEST}>")),
        "earl:test should point at the test case, got {triples:#?}"
    );
    assert!(
        triples
            .iter()
            .any(|(_, p, o)| p == "<http://www.w3.org/ns/earl#assertedBy>" && o == &format!("<{ORG}>")),
        "earl:assertedBy should point at the assertor, got {triples:#?}"
    );
}

#[test]
fn an_assertion_nests_a_typed_test_result() {
    let triples = triples_of(&submission_report());

    let result_node = triples
        .iter()
        .find(|(_, p, _)| p == "<http://www.w3.org/ns/earl#result>")
        .map(|(_, _, o)| o.clone())
        .expect("the assertion should have an earl:result");

    assert!(
        triples.iter().any(|(s, p, o)| s == &result_node
            && p == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>"
            && o == "<http://www.w3.org/ns/earl#TestResult>"),
        "the result node should be typed earl:TestResult, got {triples:#?}"
    );
    assert!(
        triples.iter().any(|(s, p, o)| s == &result_node
            && p == "<http://www.w3.org/ns/earl#outcome>"
            && o == "<http://www.w3.org/ns/earl#passed>"),
        "the result should carry earl:passed, got {triples:#?}"
    );
    assert!(
        triples.iter().any(|(s, p, o)| s == &result_node
            && p == "<http://www.w3.org/ns/earl#mode>"
            && o == "<http://www.w3.org/ns/earl#automatic>"),
        "the result should carry earl:automatic, got {triples:#?}"
    );
}

#[test]
fn the_mode_is_written_on_the_result_not_the_assertion() {
    let report = submission_report();
    let triples = triples_of(&report);

    let assertion_node = triples
        .iter()
        .find(|(_, p, o)| {
            p == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>" && o == "<http://www.w3.org/ns/earl#Assertion>"
        })
        .map(|(s, _, _)| s.clone())
        .expect("there should be an earl:Assertion");

    assert!(
        !triples
            .iter()
            .any(|(s, p, _)| s == &assertion_node && p == "<http://www.w3.org/ns/earl#mode>"),
        "the assertion itself should carry no earl:mode"
    );
    assert_eq!(count_predicate(&report, "<http://www.w3.org/ns/earl#mode>"), 2);
}

#[test]
fn the_software_is_typed_as_project_software_and_test_subject() {
    let triples = triples_of(&submission_report());
    let types: Vec<&String> = triples
        .iter()
        .filter(|(s, p, _)| s == &format!("<{TOOL}>") && p == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")
        .map(|(_, _, o)| o)
        .collect();

    assert!(types.contains(&&"<http://usefulinc.com/ns/doap#Project>".to_string()));
    assert!(types.contains(&&"<http://www.w3.org/ns/earl#Software>".to_string()));
    assert!(types.contains(&&"<http://www.w3.org/ns/earl#TestSubject>".to_string()));
}

#[test]
fn the_release_is_a_doap_version_with_the_revision() {
    let triples = triples_of(&submission_report());

    let release_node = triples
        .iter()
        .find(|(_, p, _)| p == "<http://usefulinc.com/ns/doap#release>")
        .map(|(_, _, o)| o.clone())
        .expect("the software should have a doap:release");

    assert!(triples.iter().any(|(s, p, o)| s == &release_node
        && p == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>"
        && o == "<http://usefulinc.com/ns/doap#Version>"));
    assert!(
        triples
            .iter()
            .any(|(s, p, o)| s == &release_node && p == "<http://usefulinc.com/ns/doap#revision>" && o == "\"0.3.39\"")
    );
}

#[test]
fn every_outcome_serializes_to_its_own_instance() {
    let cases = [
        (Outcome::Passed, "<http://www.w3.org/ns/earl#passed>"),
        (Outcome::Failed, "<http://www.w3.org/ns/earl#failed>"),
        (Outcome::CantTell, "<http://www.w3.org/ns/earl#cantTell>"),
        (Outcome::Inapplicable, "<http://www.w3.org/ns/earl#inapplicable>"),
        (Outcome::Untested, "<http://www.w3.org/ns/earl#untested>"),
    ];

    for (outcome, expected) in cases {
        let report = EarlReport::new().with_assertion(Assertion::new(
            Object::iri(IriS::new_unchecked(TOOL)),
            Object::iri(IriS::new_unchecked(TEST)),
            TestResult::new(outcome),
        ));
        let triples = triples_of(&report);
        assert!(
            triples
                .iter()
                .any(|(_, p, o)| p == "<http://www.w3.org/ns/earl#outcome>" && o == expected),
            "{outcome} should serialize to {expected}, got {triples:#?}"
        );
    }
}

#[test]
fn every_test_mode_serializes_to_its_own_instance() {
    let cases = [
        (TestMode::Automatic, "<http://www.w3.org/ns/earl#automatic>"),
        (TestMode::Manual, "<http://www.w3.org/ns/earl#manual>"),
        (TestMode::SemiAuto, "<http://www.w3.org/ns/earl#semiAuto>"),
        (TestMode::Undisclosed, "<http://www.w3.org/ns/earl#undisclosed>"),
        (TestMode::UnknownMode, "<http://www.w3.org/ns/earl#unknownMode>"),
    ];

    for (mode, expected) in cases {
        let report = EarlReport::new().with_assertion(Assertion::new(
            Object::iri(IriS::new_unchecked(TOOL)),
            Object::iri(IriS::new_unchecked(TEST)),
            TestResult::failed().with_mode(mode),
        ));
        let triples = triples_of(&report);
        assert!(
            triples
                .iter()
                .any(|(_, p, o)| p == "<http://www.w3.org/ns/earl#mode>" && o == expected),
            "{mode} should serialize to {expected}, got {triples:#?}"
        );
    }
}

#[test]
fn info_and_extra_triples_reach_the_result() {
    let severity = IriS::new_unchecked("http://www.w3.org/ns/shacl#Violation");
    let report = EarlReport::new().with_assertion(Assertion::new(
        Object::iri(IriS::new_unchecked(TOOL)),
        Object::iri(IriS::new_unchecked(TEST)),
        TestResult::failed()
            .with_info(ConcreteLiteral::str("Less than 1 values"))
            .with_extra_item(IriS::new_unchecked("http://www.w3.org/ns/shacl#resultSeverity"), Object::iri(severity)),
    ));
    let triples = triples_of(&report);

    assert!(
        triples
            .iter()
            .any(|(_, p, o)| p == "<http://www.w3.org/ns/earl#info>" && o == "\"Less than 1 values\""),
        "got {triples:#?}"
    );
    assert!(
        triples
            .iter()
            .any(|(_, p, o)| p == "<http://www.w3.org/ns/shacl#resultSeverity>"
                && o == "<http://www.w3.org/ns/shacl#Violation>"),
        "an extra triple should let a caller attach a term EARL has no name for, got {triples:#?}"
    );
}

#[test]
fn document_metadata_needs_a_named_document() {
    let without = EarlReport::new()
        .with_maker(Object::iri(IriS::new_unchecked(ORG)))
        .with_title("rudof SHACL report");
    assert_eq!(count_predicate(&without, "<http://purl.org/dc/terms/title>"), 0);

    let with = without.clone().with_document(IriS::new_unchecked("https://example.org/report.ttl"));
    assert_eq!(count_predicate(&with, "<http://purl.org/dc/terms/title>"), 1);
    assert_eq!(count_predicate(&with, "<http://xmlns.com/foaf/0.1/maker>"), 1);
}

#[test]
fn an_assertor_can_be_described() {
    use rudof_rdf::rdf_core::BuildRDF;
    let mut graph = rudof_rdf::rdf_impl::OxigraphInMemory::new();
    Into::<Assertor>::into(IriS::new_unchecked(ORG))
        .with_type(IriS::new_unchecked("http://xmlns.com/foaf/0.1/Organization"))
        .with_name("rudof project")
        .to_rdf(&mut graph)
        .expect("the assertor should serialize");

    let mut out = Vec::new();
    graph.serialize(&RDFFormat::NTriples, &mut out).unwrap();
    let out = String::from_utf8(out).unwrap();

    assert!(out.contains("<http://www.w3.org/ns/earl#Assertor>"), "got {out}");
    assert!(out.contains("<http://xmlns.com/foaf/0.1/Organization>"), "got {out}");
    assert!(out.contains("\"rudof project\""), "got {out}");
}

#[test]
fn counting_outcomes_tallies_the_assertions() {
    let report = EarlReport::new()
        .with_assertion(Assertion::new(
            Object::iri(IriS::new_unchecked(TOOL)),
            Object::iri(IriS::new_unchecked(TEST)),
            TestResult::passed(),
        ))
        .with_assertion(Assertion::new(
            Object::iri(IriS::new_unchecked(TOOL)),
            Object::iri(IriS::new_unchecked("urn:x-shacl-test:/core/node/class-002")),
            TestResult::failed(),
        ))
        .with_assertion(Assertion::new(
            Object::iri(IriS::new_unchecked(TOOL)),
            Object::iri(IriS::new_unchecked("urn:x-shacl-test:/core/node/class-003")),
            TestResult::failed(),
        ));

    assert_eq!(report.count_of(&Outcome::Passed), 1);
    assert_eq!(report.count_of(&Outcome::Failed), 2);
    assert_eq!(report.count_of(&Outcome::CantTell), 0);
    assert_eq!(report.assertions().len(), 3);
}

#[test]
fn turtle_output_uses_the_declared_prefixes() {
    let turtle = serialize(&submission_report(), RDFFormat::Turtle);

    assert!(turtle.contains("@prefix earl:"), "got {turtle}");
    assert!(turtle.contains("@prefix doap:"), "got {turtle}");
    assert!(turtle.contains("earl:Assertion"), "got {turtle}");
    assert!(turtle.contains("earl:passed"), "got {turtle}");
}
