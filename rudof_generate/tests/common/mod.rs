//! Helpers shared by the test suites: schema fixtures, generation, and the
//! validator oracle.
//!
//! The oracle runs rudof's own ShEx and SHACL validators against the *source*
//! schema, never against the generator's intermediate representation, so a
//! passing conformance test is independent evidence that the output is valid.
#![allow(dead_code)]

use rudof_generate::config::GeneratorConfig;
use rudof_generate::{DataGenerator, SchemaFormat};
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::formats::{
    DataFormat, InputSpec, ResultShExValidationFormat, ResultShaclValidationFormat, ShExFormat, ShaclFormat,
    ShapeMapFormat,
};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

/// A schema written to a temporary directory, plus where the generated data goes.
pub struct Fixture {
    pub dir: TempDir,
    pub schema: PathBuf,
    pub format: SchemaFormat,
}

impl Fixture {
    pub fn shex(body: &str) -> Self {
        Self::new("schema.shex", body, SchemaFormat::ShEx)
    }

    pub fn shacl(body: &str) -> Self {
        Self::new("schema.ttl", body, SchemaFormat::Shacl)
    }

    fn new(name: &str, body: &str, format: SchemaFormat) -> Self {
        let dir = TempDir::new().expect("temp dir");
        let schema = dir.path().join(name);
        std::fs::write(&schema, body).expect("write schema");
        Self { dir, schema, format }
    }

    pub fn output(&self) -> PathBuf {
        self.dir.path().join("out.ttl")
    }
}

/// A small, deterministic generator configuration writing Turtle to `output`.
pub fn config(output: &Path, entities: usize, seed: u64) -> GeneratorConfig {
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = entities;
    config.generation.seed = Some(seed);
    config.output.path = output.to_path_buf();
    config.output.write_stats = false;
    config
}

/// Generate data for `fixture` with `config` and return the Turtle written.
pub async fn generate(fixture: &Fixture, config: GeneratorConfig) -> String {
    let output = config.output.path.clone();
    let mut generator = DataGenerator::new(config).expect("generator");
    generator
        .run_with_format(&fixture.schema, Some(fixture.format))
        .await
        .expect("generation");
    std::fs::read_to_string(output).expect("generated output")
}

/// Outcome of validating one dataset.
#[derive(Debug, Default)]
pub struct Verdict {
    /// Node/shape pairs (ShEx) or focus nodes (SHACL) reported as conforming.
    pub conformant: usize,
    /// Node/shape pairs (ShEx) or validation results (SHACL) reported as violations.
    pub violations: usize,
    /// The serialized validator report, for failure messages.
    pub report: String,
    /// Nodes reported as failing (`<iri>` form).
    pub failing: std::collections::BTreeSet<String>,
}

impl Verdict {
    pub fn conforms(&self) -> bool {
        self.violations == 0
    }
}

/// Validate Turtle `data` against a ShEx schema for every node/shape pair in `shapemap`
/// (compact syntax, e.g. `{FOCUS a <C>}@<S>`).
pub fn validate_shex(data: &str, schema: &str, shapemap: &str) -> Verdict {
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof
        .load_data()
        .with_data(&[InputSpec::str(data)])
        .with_data_format(&DataFormat::Turtle)
        .execute()
        .expect("load data");
    rudof
        .load_shex_schema(&InputSpec::str(schema))
        .with_shex_schema_format(&ShExFormat::ShExC)
        .execute()
        .expect("load ShEx schema");
    rudof
        .load_shapemap(&InputSpec::str(shapemap))
        .with_shapemap_format(&ShapeMapFormat::Compact)
        .execute()
        .expect("load shapemap");
    rudof.validate_shex().execute().expect("ShEx validation");

    let mut buffer = Vec::new();
    rudof
        .serialize_shex_validation_results(&mut buffer)
        .with_result_shex_validation_format(&ResultShExValidationFormat::Csv)
        .execute()
        .expect("serialize ShEx results");
    let report = String::from_utf8(buffer).expect("utf8 report");
    let mut verdict = Verdict {
        report: report.clone(),
        ..Verdict::default()
    };
    // CSV rows are `node,shape,status,details`; `details` is quoted and may span
    // several lines, so only lines that start with a node carry a status.
    for line in report.lines().filter(|l| l.starts_with('<') || l.starts_with("_:")) {
        match line.split(',').nth(2) {
            Some("OK") => verdict.conformant += 1,
            Some("FAIL") => {
                verdict.violations += 1;
                if let Some(node) = line.split(',').next() {
                    verdict.failing.insert(node.to_string());
                }
            },
            other => panic!("unexpected ShEx status {other:?} in line: {line}"),
        }
    }
    verdict
}

/// Validate Turtle `data` against a SHACL shapes graph (targets come from the shapes).
pub fn validate_shacl(data: &str, shapes: &str) -> Verdict {
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof
        .load_data()
        .with_data(&[InputSpec::str(data)])
        .with_data_format(&DataFormat::Turtle)
        .execute()
        .expect("load data");
    rudof
        .load_shacl_shapes()
        .with_shacl_schema(&InputSpec::str(shapes))
        .with_shacl_schema_format(&ShaclFormat::Turtle)
        .execute()
        .expect("load SHACL shapes");
    rudof.validate_shacl().execute().expect("SHACL validation");

    let mut buffer = Vec::new();
    rudof
        .serialize_shacl_validation_results(&mut buffer)
        .with_result_shacl_validation_format(&ResultShaclValidationFormat::NTriples)
        .execute()
        .expect("serialize SHACL results");
    let report = String::from_utf8(buffer).expect("utf8 report");
    let violations = report
        .lines()
        .filter(|l| l.contains("<http://www.w3.org/ns/shacl#resultSeverity>"))
        .count();
    let conforms_true = report
        .lines()
        .any(|l| l.contains("<http://www.w3.org/ns/shacl#conforms>") && l.contains("true"));
    let failing = top_level_focus_nodes(&report);
    Verdict {
        conformant: usize::from(conforms_true),
        violations,
        report,
        failing,
    }
}

/// Focus nodes of the report's own `sh:result`s. Nested `sh:detail` results
/// (e.g. why an `sh:node` reference failed) name the referenced node instead,
/// which is not itself a target of the report.
fn top_level_focus_nodes(report_ntriples: &str) -> std::collections::BTreeSet<String> {
    use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
    let sh = |l: &str| format!("<http://www.w3.org/ns/shacl#{l}>");
    let graph = OxigraphInMemory::from_str(report_ntriples, &RDFFormat::NTriples, None, &ReaderMode::Strict)
        .expect("SHACL report parses");
    let triples: Vec<_> = graph.triples().expect("report triples").collect();
    let results: std::collections::BTreeSet<String> = triples
        .iter()
        .filter(|t| t.predicate.to_string() == sh("result"))
        .map(|t| t.object.to_string())
        .collect();
    triples
        .iter()
        .filter(|t| t.predicate.to_string() == sh("focusNode") && results.contains(&t.subject.to_string()))
        .map(|t| t.object.to_string())
        .collect()
}

/// Subjects of `rdf:type <class>` triples in Turtle `data` (parsed, not text-matched).
pub fn instances_of(data: &str, class: &str) -> Vec<String> {
    use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
    let graph = OxigraphInMemory::from_str(data, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse output");
    let mut subjects: Vec<String> = graph
        .triples()
        .expect("triples")
        .filter(|t| t.predicate.to_string() == format!("<{RDF_TYPE}>") && t.object.to_string() == format!("<{class}>"))
        .map(|t| t.subject.to_string())
        .collect();
    subjects.sort();
    subjects.dedup();
    subjects
}
