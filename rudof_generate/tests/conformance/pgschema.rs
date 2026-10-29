//! PG-Schema: generated property graphs must validate against the schema they
//! came from, with rudof's PG-Schema validator, and controlled violations must
//! be reported exactly: the broken nodes, plus the edges whose type requires a
//! broken endpoint to conform (`(:@T)`). Nothing propagates further: node types
//! say nothing about edges.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::OutputFormat;
use serde_json::Value;
use std::collections::BTreeSet;

const PERSONS: &str = "CREATE NODE TYPE ( PersonType : Person {
    name: STRING,
    OPTIONAL age: INTEGER CHECK > 17,
    birthDate: DATE,
    active: BOOL
}) ;
CREATE NODE TYPE ( OrgType : Org {
    label: STRING
}) ;
CREATE EDGE TYPE (:@PersonType) -[WorksForType : worksFor]-> (:@OrgType) ;
CREATE EDGE TYPE (:@PersonType) -[KnowsType : knows]-> (:@PersonType)";

struct PgRun {
    graph: String,
    type_map: String,
    labels: Vec<Value>,
}

impl PgRun {
    fn expected_failing(&self) -> BTreeSet<String> {
        self.labels
            .iter()
            .filter(|l| l["kind"] != "trap")
            .map(|l| l["node"].as_str().unwrap().to_string())
            .collect()
    }

    fn of_kind(&self, kind: &str) -> Vec<&Value> {
        self.labels.iter().filter(|l| l["kind"] == kind).collect()
    }
}

async fn run(schema: &str, violations_toml: &str, entities: usize) -> PgRun {
    let fixture = Fixture::pgschema(schema);
    let toml =
        format!("version = 2\n[run]\nseed = 5\nscale = {{ entities = {entities} }}\n[violations]\n{violations_toml}\n");
    let mut cfg = GeneratorConfig::from_toml_str(&toml).unwrap_or_else(|e| panic!("{e}\n{toml}"));
    cfg.output.path = fixture.dir.path().join("out.pg");
    cfg.output.format = OutputFormat::Pg;
    cfg.output.write_stats = false;
    let map_path = fixture.dir.path().join("out.map");
    cfg.output.shapemap = Some(map_path.clone());
    let labels_path = fixture.dir.path().join("labels.jsonl");
    if let Some(v) = cfg.generation.violations.as_mut() {
        v.labels = Some(labels_path.clone());
    }
    let graph = generate(&fixture, cfg).await;
    let labels = std::fs::read_to_string(&labels_path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("label line is JSON"))
        .collect();
    let type_map = std::fs::read_to_string(&map_path).unwrap();
    PgRun {
        graph,
        type_map,
        labels,
    }
}

#[test]
fn pgschema_oracle_flags_a_missing_required_key() {
    let graph = "(n1 {Org} [ label: \"ACME\" ])\n(n2 {Org})\n";
    let v = validate_pgschema(graph, PERSONS, "n1: OrgType,\nn2: OrgType\n");
    assert_eq!(v.failing, BTreeSet::from(["n2".to_string()]), "{}", v.report);
}

#[tokio::test]
async fn generated_property_graphs_conform() {
    let r = run(PERSONS, "rate = 0.0", 30).await;
    assert!(r.graph.contains("{Person}") && r.graph.contains("{Org}"), "{}", r.graph);
    assert!(r.graph.contains("{worksFor}"), "no edge generated\n{}", r.graph);
    let v = validate_pgschema(&r.graph, PERSONS, &r.type_map);
    assert!(v.conformant > 30, "nodes and edges must be checked\n{}", r.type_map);
    assert!(v.conforms(), "{}\n{}", v.report, r.graph);
}

#[tokio::test]
async fn node_types_with_several_labels_conform() {
    let schema = "CREATE NODE TYPE ( PersonType : Person { name: STRING }) ;
CREATE NODE TYPE ( SalariedType : Salaried { salary: INTEGER }) ;
CREATE NODE TYPE ( EmployeeType : @PersonType & @SalariedType )";
    let r = run(schema, "rate = 0.0", 12).await;
    assert!(r.graph.contains("{Person, Salaried}"), "{}", r.graph);
    let v = validate_pgschema(&r.graph, schema, &r.type_map);
    assert!(v.conforms(), "{}\n{}", v.report, r.graph);
}

#[tokio::test]
async fn each_property_graph_operator_is_exact() {
    for operator in [
        "drop-required",
        "exceed-max",
        "wrong-datatype",
        "out-of-range",
        "closed-extra",
    ] {
        let r = run(PERSONS, &format!("rate = 0.5\noperators = [\"{operator}\"]"), 30).await;
        assert!(
            !r.of_kind("direct").is_empty(),
            "{operator}: nothing injected\n{:?}",
            r.labels
        );
        let v = validate_pgschema(&r.graph, PERSONS, &r.type_map);
        assert_eq!(v.failing, r.expected_failing(), "{operator}\n{}\n{}", v.report, r.graph);
    }
}

#[tokio::test]
async fn a_broken_node_fails_its_edges_but_not_the_nodes_that_point_at_it() {
    let r = run(
        PERSONS,
        "rate = 1.0\noperators = [\"drop-required\"]\nshapes = [\"http://rudof.example/pg/type/OrgType\"]",
        30,
    )
    .await;
    let induced = r.of_kind("induced");
    assert!(!induced.is_empty(), "{:?}", r.labels);
    assert!(induced.iter().all(|l| l["shape"] == "WorksForType"), "{induced:?}");
    let v = validate_pgschema(&r.graph, PERSONS, &r.type_map);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.graph);
}

#[tokio::test]
async fn a_dangling_edge_fails_the_edge_only() {
    let r = run(PERSONS, "rate = 0.5\noperators = [\"dangling-reference\"]", 30).await;
    let direct = r.of_kind("direct");
    assert!(!direct.is_empty(), "{:?}", r.labels);
    assert!(
        direct.iter().all(|l| l["node"].as_str().unwrap().contains("__")),
        "{direct:?}"
    );
    let v = validate_pgschema(&r.graph, PERSONS, &r.type_map);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.graph);
}

#[tokio::test]
async fn a_broken_person_fails_its_edges_and_no_other_person() {
    let r = run(
        PERSONS,
        "rate = 0.3\noperators = [\"drop-required\"]\nshapes = [\"http://rudof.example/pg/type/PersonType\"]",
        40,
    )
    .await;
    assert!(
        r.of_kind("induced").iter().any(|l| l["shape"] == "KnowsType"),
        "{:?}",
        r.labels
    );
    assert!(
        r.of_kind("direct").iter().all(|l| l["shape"] == "PersonType"),
        "{:?}",
        r.labels
    );
    let v = validate_pgschema(&r.graph, PERSONS, &r.type_map);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.graph);
}
