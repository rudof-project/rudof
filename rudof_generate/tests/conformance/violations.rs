//! Controlled violations: the generator breaks chosen constraints on chosen
//! entities and writes ground-truth labels; the real validators must report
//! exactly the labelled nodes (direct violations plus failures induced through
//! shape references), no more and no fewer.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use serde_json::Value;
use std::collections::BTreeSet;

const EX: &str = "http://example.org/";

struct Run {
    data: String,
    labels: Vec<Value>,
    shapemap: String,
}

impl Run {
    /// Nodes the labels say must fail (direct and induced), in `<iri>` form.
    fn expected_failing(&self) -> BTreeSet<String> {
        self.labels
            .iter()
            .map(|l| format!("<{}>", l["node"].as_str().unwrap()))
            .collect()
    }

    fn direct(&self) -> Vec<&Value> {
        self.labels.iter().filter(|l| l["kind"] == "direct").collect()
    }
}

async fn run(fixture: Fixture, violations_toml: &str, entities: usize) -> Run {
    let toml = format!(
        "version = 2\n[prefixes]\nex = \"{EX}\"\n[run]\nseed = 11\nscale = {{ entities = {entities} }}\n[violations]\n{violations_toml}\n"
    );
    let mut cfg = GeneratorConfig::from_toml_str(&toml).unwrap_or_else(|e| panic!("{e}\n{toml}"));
    cfg.output.path = fixture.output();
    cfg.output.write_stats = false;
    let labels_path = fixture.dir.path().join("labels.jsonl");
    let shapemap_path = fixture.dir.path().join("out.shapemap");
    cfg.output.shapemap = Some(shapemap_path.clone());
    if let Some(v) = cfg.generation.violations.as_mut() {
        v.labels = Some(labels_path.clone());
    }
    let data = generate(&fixture, cfg).await;
    let labels = std::fs::read_to_string(&labels_path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("label line is JSON"))
        .collect();
    let shapemap = std::fs::read_to_string(&shapemap_path).unwrap();
    Run { data, labels, shapemap }
}

async fn assert_shex_exact(schema: &str, violations_toml: &str, operator: &str) {
    let r = run(Fixture::shex(schema), violations_toml, 12).await;
    assert!(
        !r.direct().is_empty(),
        "no violation injected for {operator}\n{}",
        r.data
    );
    assert!(r.direct().iter().all(|l| l["operator"] == operator), "{:?}", r.labels);
    let v = validate_shex(&r.data, schema, &r.shapemap);
    assert_eq!(
        v.failing,
        r.expected_failing(),
        "validator verdicts must equal the labels for {operator}\n--- report ---\n{}\n--- data ---\n{}",
        v.report,
        r.data
    );
}

async fn assert_shacl_exact(shapes: &str, violations_toml: &str, operator: &str) {
    let r = run(Fixture::shacl(shapes), violations_toml, 12).await;
    assert!(
        !r.direct().is_empty(),
        "no violation injected for {operator}\n{}",
        r.data
    );
    let v = validate_shacl(&r.data, shapes);
    assert_eq!(
        v.failing,
        r.expected_failing(),
        "validator verdicts must equal the labels for {operator}\n--- report ---\n{}\n--- data ---\n{}",
        v.report,
        r.data
    );
}

const SHEX_PERSON: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:PersonShape {
  a [ex:Person] ;
  ex:name xsd:string ;
  ex:nick xsd:string {0,2} ;
  ex:age xsd:integer MININCLUSIVE 18 MAXINCLUSIVE 65 ;
  ex:status ["active" "retired"] ;
  ex:code xsd:string /^[A-Z]{3}$/
}"#;

fn shacl_person() -> String {
    format!(
        r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <{EX}> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;
  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:string ] ;
  sh:property [ sh:path ex:nick ; sh:maxCount 2 ; sh:datatype xsd:string ] ;
  sh:property [ sh:path ex:age ; sh:minCount 1 ; sh:datatype xsd:integer ; sh:minInclusive 18 ; sh:maxInclusive 65 ] ;
  sh:property [ sh:path ex:status ; sh:minCount 1 ; sh:in ( "active" "retired" ) ] ;
  sh:property [ sh:path ex:code ; sh:minCount 1 ; sh:datatype xsd:string ; sh:pattern "^[A-Z]{{3}}$" ] .
"#
    )
}

const OPERATORS: [&str; 7] = [
    "drop-required",
    "exceed-max",
    "wrong-datatype",
    "ill-typed-literal",
    "out-of-range",
    "not-in-value-set",
    "pattern-break",
];

#[tokio::test]
async fn each_shex_operator_yields_exactly_the_labelled_failures() {
    for op in OPERATORS {
        assert_shex_exact(SHEX_PERSON, &format!("rate = 1.0\noperators = [\"{op}\"]"), op).await;
    }
}

#[tokio::test]
async fn each_shacl_operator_yields_exactly_the_labelled_failures() {
    for op in OPERATORS {
        assert_shacl_exact(&shacl_person(), &format!("rate = 1.0\noperators = [\"{op}\"]"), op).await;
    }
}

const SHEX_REF: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:worksFor @ex:OrgShape }
ex:OrgShape { a [ex:Org] ; ex:label xsd:string }"#;

#[tokio::test]
async fn a_dangling_reference_breaks_the_referring_node() {
    assert_shex_exact(
        SHEX_REF,
        "rate = 1.0\noperators = [\"dangling-reference\"]\nshapes = [\"ex:PersonShape\"]",
        "dangling-reference",
    )
    .await;
}

#[tokio::test]
async fn breaking_a_referenced_node_also_fails_the_nodes_that_reference_it() {
    let r = run(
        Fixture::shex(SHEX_REF),
        "rate = 1.0\noperators = [\"drop-required\"]\nshapes = [\"ex:OrgShape\"]",
        12,
    )
    .await;
    let induced: Vec<_> = r.labels.iter().filter(|l| l["kind"] == "induced").collect();
    assert!(
        !induced.is_empty(),
        "persons pointing at broken orgs must be labelled\n{:?}",
        r.labels
    );
    assert!(
        induced
            .iter()
            .all(|l| l["shape"] == format!("{EX}PersonShape") && l["cause"].is_string())
    );
    let v = validate_shex(&r.data, SHEX_REF, &r.shapemap);
    assert_eq!(v.failing, r.expected_failing(), "{}", v.report);
}

#[tokio::test]
async fn a_zero_rate_injects_nothing_and_the_data_conforms() {
    let r = run(Fixture::shex(SHEX_PERSON), "rate = 0.0", 12).await;
    assert!(r.labels.is_empty(), "{:?}", r.labels);
    let v = validate_shex(&r.data, SHEX_PERSON, &r.shapemap);
    assert!(v.conforms(), "{}", v.report);
}

#[tokio::test]
async fn a_partial_rate_breaks_some_entities_and_the_rest_conform() {
    let r = run(Fixture::shex(SHEX_PERSON), "rate = 0.5", 40).await;
    let direct = r.direct().len();
    assert!(direct > 5 && direct < 35, "about half of 40 entities: got {direct}");
    let v = validate_shex(&r.data, SHEX_PERSON, &r.shapemap);
    assert_eq!(v.failing, r.expected_failing(), "{}", v.report);
    assert_eq!(v.conformant, 40 - direct);
}

#[tokio::test]
async fn the_same_seed_injects_the_same_violations() {
    let a = run(Fixture::shex(SHEX_PERSON), "rate = 0.5", 20).await;
    let b = run(Fixture::shex(SHEX_PERSON), "rate = 0.5", 20).await;
    assert_eq!(a.labels, b.labels);
}

#[test]
fn an_unknown_operator_is_rejected() {
    let err = GeneratorConfig::from_toml_str("version = 2\n[violations]\nrate = 0.1\noperators = [\"explode\"]\n")
        .unwrap_err()
        .to_string();
    assert!(err.contains("explode"), "{err}");
}

#[test]
fn the_rate_must_be_a_probability() {
    let err = GeneratorConfig::from_toml_str("version = 2\n[violations]\nrate = 2.0\n")
        .unwrap_err()
        .to_string();
    assert!(err.contains("rate"), "{err}");
}

#[tokio::test]
async fn closed_extra_breaks_closed_shex_shapes_only() {
    let schema = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:PersonShape CLOSED { a [ex:Person] ; ex:name xsd:string }"#;
    assert_shex_exact(schema, "rate = 1.0\noperators = [\"closed-extra\"]", "closed-extra").await;
}

#[tokio::test]
async fn closed_extra_breaks_closed_shacl_shapes() {
    let shapes = format!(
        r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <{EX}> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;
  sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:datatype xsd:string ] .
"#
    );
    assert_shacl_exact(&shapes, "rate = 1.0\noperators = [\"closed-extra\"]", "closed-extra").await;
}

#[tokio::test]
async fn closed_extra_does_not_apply_to_open_shapes() {
    let r = run(
        Fixture::shex(SHEX_PERSON),
        "rate = 1.0\noperators = [\"closed-extra\"]",
        6,
    )
    .await;
    assert!(
        r.labels.is_empty(),
        "open shapes allow extra properties: {:?}",
        r.labels
    );
}
