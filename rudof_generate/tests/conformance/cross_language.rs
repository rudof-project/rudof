//! One graph, three languages. A graph generated from a PG-Schema and broken once
//! is written both as a property graph and as RDF, and validated against the
//! PG-Schema and against ShEx and SHACL translations of it. Each output has its
//! own labels, because the languages place a reference failure differently: in a
//! property graph the failing edge is labelled and nothing propagates further; in
//! RDF the node that holds the reference fails, and so does every node that
//! reaches it. Both label sets must equal what the validators report.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::OutputFormat;
use serde_json::Value;
use std::collections::BTreeSet;

/// Person -worksFor-> Org -locatedIn-> City: two reference steps, no recursion
/// (SHACL leaves recursion undefined).
const CHAIN_PGS: &str = "CREATE NODE TYPE ( PersonType : Person {
    name: STRING,
    OPTIONAL age: INTEGER CHECK > 17
}) ;
CREATE NODE TYPE ( OrgType : Org {
    label: STRING
}) ;
CREATE NODE TYPE ( CityType : City {
    name: STRING
}) ;
CREATE EDGE TYPE (:@PersonType) -[WorksForType : worksFor]-> (:@OrgType) ;
CREATE EDGE TYPE (:@OrgType) -[LocatedInType : locatedIn]-> (:@CityType)";

/// The PG-Schema above over its RDF encoding (keys, labels and edges as IRIs).
/// Records are closed in PG-Schema, so the shapes are closed too.
const CHAIN_SHEX: &str = "PREFIX t: <http://rudof.example/pg/type/>
PREFIX k: <http://rudof.example/pg/key/>
PREFIX l: <http://rudof.example/pg/label/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
t:PersonType CLOSED {
  a [ l:Person ] ;
  k:name xsd:string ;
  k:age xsd:integer MinExclusive 17 ? ;
  <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> @t:OrgType *
}
t:OrgType CLOSED {
  a [ l:Org ] ;
  k:label xsd:string ;
  <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> @t:CityType *
}
t:CityType CLOSED {
  a [ l:City ] ;
  k:name xsd:string
}";

const CHAIN_SHACL: &str = "@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix t: <http://rudof.example/pg/type/> .
@prefix k: <http://rudof.example/pg/key/> .
@prefix l: <http://rudof.example/pg/label/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
t:PersonType a sh:NodeShape ; sh:targetClass l:Person ;
  sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:age ; sh:datatype xsd:integer ; sh:minExclusive 17 ; sh:maxCount 1 ] ;
  sh:property [ sh:path <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> ;
                sh:node t:OrgType ] .
t:OrgType a sh:NodeShape ; sh:targetClass l:Org ;
  sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:label ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> ;
                sh:node t:CityType ] .
t:CityType a sh:NodeShape ; sh:targetClass l:City ;
  sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .";

struct Run {
    graph: String,
    type_map: String,
    pg_labels: Vec<Value>,
    rdf: String,
    rdf_shapemap: String,
    rdf_labels: Vec<Value>,
}

fn read_labels(path: &std::path::Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("label line is JSON"))
        .collect()
}

fn failing(labels: &[Value]) -> BTreeSet<String> {
    labels
        .iter()
        .filter(|l| l["kind"] != "trap")
        .map(|l| l["node"].as_str().unwrap().to_string())
        .collect()
}

/// `reading` is extra TOML for `[violations.rdf]` (how the RDF translation reads
/// the schema's defaults); `operators` restricts the operators when not empty.
async fn run(entities: usize, rate: f64, operators: &[&str], reading: &str) -> Run {
    let fixture = Fixture::pgschema(CHAIN_PGS);
    let dir = fixture.dir.path();
    let toml = format!(
        "version = 2\n[run]\nseed = 11\nscale = {{ entities = {entities} }}\n[violations]\nrate = {rate}\n\
         {}labels = \"{}\"\n[violations.rdf]\nlabels = \"{}\"\n{reading}\n\
         [output]\nformat = \"pg\"\nrdf = \"{}\"\nrdf_shapemap = \"{}\"\n",
        if operators.is_empty() {
            String::new()
        } else {
            let quoted: Vec<String> = operators.iter().map(|o| format!("\"{o}\"")).collect();
            format!("operators = [{}]\n", quoted.join(", "))
        },
        dir.join("labels.jsonl").display().to_string().replace('\\', "/"),
        dir.join("rdf_labels.jsonl").display().to_string().replace('\\', "/"),
        dir.join("out.ttl").display().to_string().replace('\\', "/"),
        dir.join("out.shapemap").display().to_string().replace('\\', "/"),
    );
    let mut cfg = GeneratorConfig::from_toml_str(&toml).unwrap_or_else(|e| panic!("{e}\n{toml}"));
    assert_eq!(cfg.output.format, OutputFormat::Pg);
    cfg.output.path = dir.join("out.pg");
    cfg.output.write_stats = false;
    cfg.output.shapemap = Some(dir.join("out.map"));
    let graph = generate(&fixture, cfg).await;
    Run {
        graph,
        type_map: std::fs::read_to_string(dir.join("out.map")).unwrap(),
        pg_labels: read_labels(&dir.join("labels.jsonl")),
        rdf: std::fs::read_to_string(dir.join("out.ttl")).unwrap(),
        rdf_shapemap: std::fs::read_to_string(dir.join("out.shapemap")).unwrap(),
        rdf_labels: read_labels(&dir.join("rdf_labels.jsonl")),
    }
}

/// IRIs as the validators print them: without angle brackets.
fn bare(set: &BTreeSet<String>) -> BTreeSet<String> {
    set.iter()
        .map(|n| n.trim_matches(|c| c == '<' || c == '>').to_string())
        .collect()
}

#[test]
fn the_rdf_translations_accept_a_conforming_chain() {
    let data = "@prefix k: <http://rudof.example/pg/key/> .
@prefix l: <http://rudof.example/pg/label/> .
<http://x/p> a l:Person ; k:name \"Ann\" ;
  <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> <http://x/o> .
<http://x/o> a l:Org ; k:label \"ACME\" ;
  <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> <http://x/c> .
<http://x/c> a l:City ; k:name \"Oviedo\" .";
    let map = "<http://x/p>@<http://rudof.example/pg/type/PersonType>,\n\
               <http://x/o>@<http://rudof.example/pg/type/OrgType>,\n\
               <http://x/c>@<http://rudof.example/pg/type/CityType>";
    let shex = validate_shex(data, CHAIN_SHEX, map);
    assert!(shex.failing.is_empty(), "{}", shex.report);
    let shacl = validate_shacl(data, CHAIN_SHACL);
    assert!(shacl.failing.is_empty(), "{}", shacl.report);
    // A city without a name fails, and so do the org and the person that reach it.
    let broken = data.replace("k:name \"Oviedo\"", "k:label \"Oviedo\"");
    let shacl = validate_shacl(&broken, CHAIN_SHACL);
    assert_eq!(
        bare(&shacl.failing),
        BTreeSet::from(["http://x/p".into(), "http://x/o".into(), "http://x/c".into()]),
        "{}",
        shacl.report
    );
}

#[tokio::test]
async fn a_clean_graph_conforms_in_all_three_languages() {
    let r = run(60, 0.0, &[], "").await;
    assert!(r.pg_labels.is_empty() && r.rdf_labels.is_empty());
    let pg = validate_pgschema(&r.graph, CHAIN_PGS, &r.type_map);
    assert!(pg.failing.is_empty(), "{}", pg.report);
    let shex = validate_shex(&r.rdf, CHAIN_SHEX, &r.rdf_shapemap);
    assert!(shex.failing.is_empty(), "{}", shex.report);
    let shacl = validate_shacl(&r.rdf, CHAIN_SHACL);
    assert!(shacl.failing.is_empty(), "{}", shacl.report);
}

#[tokio::test]
async fn one_broken_graph_is_labelled_for_pg_schema_and_for_rdf() {
    let r = run(300, 0.3, &[], "").await;

    let pg = validate_pgschema(&r.graph, CHAIN_PGS, &r.type_map);
    assert_eq!(pg.failing, failing(&r.pg_labels), "{}", pg.report);

    let expected_rdf = bare(&failing(&r.rdf_labels));
    let shex = validate_shex(&r.rdf, CHAIN_SHEX, &r.rdf_shapemap);
    assert_eq!(bare(&shex.failing), expected_rdf, "{}", shex.report);
    let shacl = validate_shacl(&r.rdf, CHAIN_SHACL);
    assert_eq!(bare(&shacl.failing), expected_rdf, "{}", shacl.report);

    // The two semantics differ: in RDF a person fails when the city of its
    // organisation fails (two steps); in the property graph no person does.
    let induced_rdf: Vec<_> = r.rdf_labels.iter().filter(|l| l["kind"] == "induced").collect();
    assert!(
        induced_rdf
            .iter()
            .any(|l| l["shape"].as_str().unwrap().ends_with("PersonType")
                && l["cause"].as_str().is_some_and(|c| c.contains("OrgType"))),
        "expected a two-step induced failure: {induced_rdf:?}"
    );
    assert!(
        r.pg_labels
            .iter()
            .all(|l| l["kind"] != "induced" || l["node"].as_str().unwrap().contains("__")),
        "property graph induced failures are edges"
    );
}

/// The SHACL a modeller would write for the same schema: shapes open, and keys
/// without `sh:minCount`, since SHACL properties are optional by default.
const CHAIN_SHACL_IDIOMATIC: &str = "@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix t: <http://rudof.example/pg/type/> .
@prefix k: <http://rudof.example/pg/key/> .
@prefix l: <http://rudof.example/pg/label/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
t:PersonType a sh:NodeShape ; sh:targetClass l:Person ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:age ; sh:datatype xsd:integer ; sh:minExclusive 17 ; sh:maxCount 1 ] ;
  sh:property [ sh:path <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> ;
                sh:node t:OrgType ] .
t:OrgType a sh:NodeShape ; sh:targetClass l:Org ;
  sh:property [ sh:path k:label ; sh:datatype xsd:string ; sh:maxCount 1 ] ;
  sh:property [ sh:path <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> ;
                sh:node t:CityType ] .
t:CityType a sh:NodeShape ; sh:targetClass l:City ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:maxCount 1 ] .";

/// Default closedness and default cardinality differ between the languages, so
/// the same mutation fails in PG-Schema and is harmless in idiomatic SHACL: an
/// undeclared key (records are closed, shapes are open) and a dropped key (keys
/// are required unless OPTIONAL, SHACL properties are optional).
#[tokio::test]
async fn language_defaults_decide_whether_the_same_mutation_fails() {
    let r = run(
        300,
        0.3,
        &["closed-extra", "drop-required"],
        "closed = false\nkeys_required = false",
    )
    .await;

    let pg = validate_pgschema(&r.graph, CHAIN_PGS, &r.type_map);
    assert_eq!(pg.failing, failing(&r.pg_labels), "{}", pg.report);
    let shacl = validate_shacl(&r.rdf, CHAIN_SHACL_IDIOMATIC);
    assert_eq!(bare(&shacl.failing), bare(&failing(&r.rdf_labels)), "{}", shacl.report);

    // Every node broken in the property graph is a trap in the RDF reading.
    let broken_pg = r.pg_labels.iter().filter(|l| l["kind"] == "direct").count();
    assert!(broken_pg > 20, "too few broken nodes: {broken_pg}");
    let rdf_traps = r.rdf_labels.iter().filter(|l| l["kind"] == "trap").count();
    assert_eq!(rdf_traps, broken_pg, "{:?}", r.rdf_labels);
    assert!(shacl.failing.is_empty(), "{}", shacl.report);
}
