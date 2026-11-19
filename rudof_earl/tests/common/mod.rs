//! Helpers for the EARL report tests

#![allow(dead_code, unused_imports)]

mod report;

pub use report::{DOCUMENT, ORG, TEST, TOOL, submission_report, submission_report_graph};
use rudof_earl::report::EarlReport;
use rudof_rdf::rdf_core::term::Triple;
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::OxigraphInMemory;

/// The report's triples
pub fn triples_of(report: &EarlReport) -> Vec<(String, String, String)> {
    let graph = graph_of(report);
    let mut triples: Vec<_> = graph
        .triples()
        .unwrap()
        .map(|t| {
            let (s, p, o) = t.into_components();
            (s.to_string(), p.to_string(), o.to_string())
        })
        .collect();
    triples.sort();
    triples
}

/// The report as an in-memory graph
pub fn graph_of(report: &EarlReport) -> OxigraphInMemory {
    let mut graph = OxigraphInMemory::new();
    report.to_rdf(&mut graph).expect("the report should serialize");
    graph
}

/// The report serialized in `format`
pub fn serialize(report: &EarlReport, format: RDFFormat) -> String {
    use rudof_rdf::rdf_core::BuildRDF;
    let graph = graph_of(report);
    let mut out = Vec::new();
    graph.serialize(&format, &mut out).expect("the graph should serialize");
    String::from_utf8(out).expect("the serialization should be UTF-8")
}

/// How many triples in `report` have `predicate` as their predicate
pub fn count_predicate(report: &EarlReport, predicate: &str) -> usize {
    triples_of(report).iter().filter(|(_, p, _)| p == predicate).count()
}

// Serializes the store into a turtle string
pub fn turtle(graph: &OxigraphInMemory) -> String {
    use rudof_rdf::rdf_core::BuildRDF;
    let mut out = Vec::new();
    graph.serialize(&RDFFormat::Turtle, &mut out).expect("serializable");
    String::from_utf8(out).expect("UTF-8")
}
