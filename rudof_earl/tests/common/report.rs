use rudof_earl::model::{Assertion, TestResult};
use rudof_earl::report::EarlReport;
use rudof_earl::{Release, Software};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, XsdDateTime};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};

pub const TOOL: &str = "https://github.com/rudof-project/rudof";
pub const ORG: &str = "https://github.com/rudof-project";
pub const DOCUMENT: &str = "https://rudof-project.github.io/reports/rudof-shacl-earl.ttl";
pub const TEST: &str = "urn:x-shacl-test:/core/node/disjoint-001";

const REPORT: &str = r#"
@prefix dct:  <http://purl.org/dc/terms/> .
@prefix doap: <http://usefulinc.com/ns/doap#> .
@prefix earl: <http://www.w3.org/ns/earl#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .

<https://rudof-project.github.io/reports/rudof-shacl-earl.ttl>
    dct:title         "rudof SHACL implementation report" ;
    dct:issued        "2026-11-17T09:30:00Z"^^xsd:dateTime ;
    foaf:maker        <https://github.com/rudof-project> ;
    foaf:primaryTopic <https://github.com/rudof-project/rudof> .

<https://github.com/rudof-project/rudof>
    a                          doap:Project, earl:Software, earl:TestSubject ;
    doap:name                  "rudof" ;
    doap:developer             <https://github.com/rudof-project> ;
    doap:homepage              <https://rudof-project.github.io/rudof> ;
    doap:programming-language  "Rust" ;
    doap:release [
        a             doap:Version ;
        doap:revision "0.3.39" ;
        doap:created  "2026-11-17"^^xsd:date ;
    ] .

[] a                earl:Assertion ;
   earl:assertedBy  <https://github.com/rudof-project> ;
   earl:subject     <https://github.com/rudof-project/rudof> ;
   earl:test        <urn:x-shacl-test:/core/node/disjoint-001> ;
   earl:result [
       a            earl:TestResult ;
       earl:mode    earl:automatic ;
       earl:outcome earl:passed ;
   ] .

[] a                earl:Assertion ;
   earl:assertedBy  <https://github.com/rudof-project> ;
   earl:subject     <https://github.com/rudof-project/rudof> ;
   earl:test        <urn:x-shacl-test:/core/node/class-002> ;
   earl:result [
       a            earl:TestResult ;
       earl:mode    earl:automatic ;
       earl:outcome earl:failed ;
       earl:info    "expected sh:conforms false, got true" ;
   ] .

"#;

pub fn submission_report_graph() -> OxigraphInMemory {
    OxigraphInMemory::from_str(REPORT, &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .unwrap_or_else(|e| panic!("an error happened while parsing the string: {e}"))
}

pub fn submission_report() -> EarlReport {
    EarlReport::new()
        .with_document(IriS::new_unchecked(DOCUMENT))
        .with_title("rudof SHACL implementation report")
        .with_issued(XsdDateTime::new("2026-11-17T09:30:00Z").expect("a valid xsd:dateTime"))
        .with_maker(Object::iri(IriS::new_unchecked(ORG)))
        .with_primary_topic(Object::iri(IriS::new_unchecked(TOOL)))
        .with_software_item(
            Into::<Software>::into(IriS::new_unchecked(TOOL))
                .with_name("rudof")
                .with_developer(Object::iri(IriS::new_unchecked(ORG)))
                .with_homepage(IriS::new_unchecked("https://rudof-project.github.io/rudof"))
                .with_programming_language("Rust")
                .with_release(Release::new("0.3.39").with_created("2026-11-17")),
        )
        .with_assertion(
            Assertion::new(
                Object::iri(IriS::new_unchecked(TOOL)),
                Object::iri(IriS::new_unchecked("urn:x-shacl-test:/core/node/disjoint-001")),
                TestResult::passed(),
            )
            .with_asserted_by(Object::iri(IriS::new_unchecked(ORG))),
        )
        .with_assertion(
            Assertion::new(
                Object::iri(IriS::new_unchecked(TOOL)),
                Object::iri(IriS::new_unchecked("urn:x-shacl-test:/core/node/class-002")),
                TestResult::failed().with_info(ConcreteLiteral::str("expected sh:conforms false, got true")),
            )
            .with_asserted_by(Object::iri(IriS::new_unchecked(ORG))),
        )
}
