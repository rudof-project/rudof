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
    /// Trap labels mark changes that must NOT be reported.
    fn expected_failing(&self) -> BTreeSet<String> {
        self.labels
            .iter()
            .filter(|l| l["kind"] != "trap")
            .map(|l| format!("<{}>", l["node"].as_str().unwrap()))
            .collect()
    }

    fn traps(&self) -> Vec<&Value> {
        self.labels.iter().filter(|l| l["kind"] == "trap").collect()
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

const OPERATORS: [&str; 11] = [
    "drop-required",
    "exceed-max",
    "wrong-datatype",
    "ill-typed-literal",
    "out-of-range",
    "not-in-value-set",
    "pattern-break",
    "lang-string-for-string",
    "derived-datatype",
    "lookalike-value",
    "unmatched-value",
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

const SHEX_LANG: &str = r#"PREFIX ex: <http://example.org/>
ex:PersonShape { a [ex:Person] ; ex:label [@en @es] }"#;

const SHEX_LANG_STEM: &str = r#"PREFIX ex: <http://example.org/>
ex:PersonShape { a [ex:Person] ; ex:label [@en~] }"#;

fn shacl_lang() -> String {
    format!(
        r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <{EX}> .
ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;
  sh:property [ sh:path ex:label ; sh:minCount 1 ; sh:languageIn ( "en" "es" ) ; sh:uniqueLang true ] .
"#
    )
}

#[tokio::test]
async fn wrong_language_is_exact_in_shex_and_shacl() {
    assert_shex_exact(
        SHEX_LANG,
        "rate = 1.0\noperators = [\"wrong-language\"]",
        "wrong-language",
    )
    .await;
    assert_shacl_exact(
        &shacl_lang(),
        "rate = 1.0\noperators = [\"wrong-language\"]",
        "wrong-language",
    )
    .await;
}

#[tokio::test]
async fn duplicate_language_breaks_unique_lang() {
    assert_shacl_exact(
        &shacl_lang(),
        "rate = 1.0\noperators = [\"duplicate-language\"]",
        "duplicate-language",
    )
    .await;
}

/// A trap changes the data without breaking it; reporting it is a false positive.
async fn assert_trap_not_reported_shex(schema: &str, operator: &str) {
    let r = run(
        Fixture::shex(schema),
        &format!("rate = 1.0\noperators = [\"{operator}\"]"),
        8,
    )
    .await;
    assert!(
        !r.traps().is_empty(),
        "no trap planted for {operator}
{}",
        r.data
    );
    let v = validate_shex(&r.data, schema, &r.shapemap);
    assert!(
        v.failing.is_empty(),
        "{operator} must not be reported
{}
{}",
        v.report,
        r.data
    );
}

#[tokio::test]
async fn language_tags_compare_case_insensitively() {
    assert_trap_not_reported_shex(SHEX_LANG, "trap-language-case").await;
    let r = run(
        Fixture::shacl(&shacl_lang()),
        "rate = 1.0\noperators = [\"trap-language-case\"]",
        8,
    )
    .await;
    assert!(!r.traps().is_empty(), "{}", r.data);
    let v = validate_shacl(&r.data, &shacl_lang());
    assert!(
        v.failing.is_empty(),
        "{}
{}",
        v.report,
        r.data
    );
}

#[tokio::test]
async fn a_language_range_accepts_subtags() {
    assert_trap_not_reported_shex(SHEX_LANG_STEM, "trap-language-subtag").await;
    let r = run(
        Fixture::shacl(&shacl_lang()),
        "rate = 1.0\noperators = [\"trap-language-subtag\"]",
        8,
    )
    .await;
    assert!(!r.traps().is_empty(), "{}", r.data);
    let v = validate_shacl(&r.data, &shacl_lang());
    assert!(
        v.failing.is_empty(),
        "{}
{}",
        v.report,
        r.data
    );
}

#[tokio::test]
async fn an_exact_language_tag_rejects_subtags_so_no_trap_is_planted() {
    // ShEx [@en] matches the tag exactly: en-GB would be a real violation, not a trap.
    let r = run(
        Fixture::shex(SHEX_LANG),
        "rate = 1.0\noperators = [\"trap-language-subtag\"]",
        6,
    )
    .await;
    assert!(r.labels.is_empty(), "{:?}", r.labels);
}

#[tokio::test]
async fn numeric_ranges_compare_values_not_lexical_forms() {
    assert_trap_not_reported_shex(SHEX_PERSON, "trap-numeric-lexical").await;
    let shapes = shacl_person();
    let r = run(
        Fixture::shacl(&shapes),
        "rate = 1.0\noperators = [\"trap-numeric-lexical\"]",
        8,
    )
    .await;
    assert!(!r.traps().is_empty(), "{}", r.data);
    // Turtle writes integers in shorthand, so the planted form shows as `0NN`.
    assert!(
        r.data.contains(&format!("<{EX}age> 0")),
        "a non-canonical lexical form was planted\n{}",
        r.data
    );
    let v = validate_shacl(&r.data, &shapes);
    assert!(v.failing.is_empty(), "{}\n{}", v.report, r.data);
}

/// One-property schemas whose value set holds IRIs, integers or strings.
fn value_set_schemas() -> Vec<(String, String)> {
    let shacl = |members: &str| {
        format!(
            r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <{EX}> .
ex:ThingShape a sh:NodeShape ; sh:targetClass ex:Thing ;
  sh:property [ sh:path ex:kind ; sh:minCount 1 ; sh:maxCount 1 ; sh:in ( {members} ) ] .
"#
        )
    };
    let shex = |members: &str| format!("PREFIX ex: <{EX}>\nex:ThingShape {{ a [ex:Thing] ; ex:kind [{members}] }}");
    ["ex:A ex:B", "1 2", "\"active\" \"retired\""]
        .into_iter()
        .map(|m| (shex(m), shacl(m)))
        .collect()
}

#[tokio::test]
async fn a_lookalike_of_a_value_set_member_is_not_a_member() {
    // Same text, different term: "http://...A" for <...A>, "1" for 1, "active"@en for "active".
    let ops = "rate = 1.0\noperators = [\"lookalike-value\"]";
    for (shex, shacl) in value_set_schemas() {
        assert_shex_exact(&shex, ops, "lookalike-value").await;
        assert_shacl_exact(&shacl, ops, "lookalike-value").await;
    }
}

const SHEX_SCORE: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:ThingShape { a [ex:Thing] ; ex:score xsd:decimal MININCLUSIVE 0 MAXINCLUSIVE 10 }"#;

fn shacl_score() -> String {
    format!(
        r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <{EX}> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
ex:ThingShape a sh:NodeShape ; sh:targetClass ex:Thing ;
  sh:property [ sh:path ex:score ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:decimal ;
                sh:minInclusive 0 ; sh:maxInclusive 10 ] .
"#
    )
}

#[tokio::test]
async fn decimal_ranges_compare_values_not_trailing_zeros() {
    assert_trap_not_reported_shex(SHEX_SCORE, "trap-decimal-lexical").await;
    let shapes = shacl_score();
    let r = run(
        Fixture::shacl(&shapes),
        "rate = 1.0\noperators = [\"trap-decimal-lexical\"]",
        8,
    )
    .await;
    assert!(!r.traps().is_empty(), "{}", r.data);
    let v = validate_shacl(&r.data, &shapes);
    assert!(v.failing.is_empty(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn an_operator_spreads_over_every_property_it_applies_to() {
    // derived-datatype applies to ex:age, ex:code, ex:name and ex:nick of the person shape.
    let r = run(
        Fixture::shex(SHEX_PERSON),
        "rate = 1.0\noperators = [\"derived-datatype\"]",
        24,
    )
    .await;
    let properties: BTreeSet<&str> = r.direct().iter().map(|l| l["property"].as_str().unwrap()).collect();
    assert!(properties.len() >= 3, "broken properties: {properties:?}");
    let v = validate_shex(&r.data, SHEX_PERSON, &r.shapemap);
    assert_eq!(v.failing, r.expected_failing(), "{}", v.report);
}

const SHEX_EXTRA: &str = r#"PREFIX ex: <http://example.org/>
ex:ThingShape EXTRA ex:kind { a [ex:Thing] ; ex:kind [ex:A ex:B] }"#;

#[tokio::test]
async fn extra_lets_a_shex_shape_carry_values_that_match_no_constraint() {
    assert_trap_not_reported_shex(SHEX_EXTRA, "trap-extra-value").await;
}

#[tokio::test]
async fn without_extra_an_unmatched_value_of_a_mentioned_predicate_fails() {
    let schema = SHEX_EXTRA.replace("EXTRA ex:kind ", "");
    assert_shex_exact(
        &schema,
        "rate = 1.0\noperators = [\"unmatched-value\"]",
        "unmatched-value",
    )
    .await;
}

#[tokio::test]
async fn unmatched_value_leaves_extra_predicates_alone() {
    let r = run(
        Fixture::shex(SHEX_EXTRA),
        "rate = 1.0\noperators = [\"unmatched-value\"]",
        6,
    )
    .await;
    assert!(r.direct().is_empty(), "{:?}", r.labels);
}

fn shacl_team() -> String {
    format!(
        r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <{EX}> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
ex:TeamShape a sh:NodeShape ; sh:targetClass ex:Team ;
  sh:property [ sh:path ex:member ; sh:qualifiedValueShape ex:PersonShape ;
                sh:qualifiedMinCount 2 ; sh:qualifiedMaxCount 3 ] .
ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;
  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:datatype xsd:string ] .
"#
    )
}

#[tokio::test]
async fn qualified_value_shapes_are_generated_valid() {
    let shapes = shacl_team();
    let r = run(Fixture::shacl(&shapes), "rate = 0.0", 12).await;
    assert!(r.data.contains(&format!("<{EX}member>")), "{}", r.data);
    let v = validate_shacl(&r.data, &shapes);
    assert!(v.conforms(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn a_qualified_value_shape_tolerates_non_conforming_extra_values() {
    // Only the count of conforming values is constrained, unlike sh:node.
    let shapes = shacl_team();
    let r = run(
        Fixture::shacl(&shapes),
        "rate = 1.0\noperators = [\"trap-extra-value\"]\nshapes = [\"ex:TeamShape\"]",
        12,
    )
    .await;
    assert!(!r.traps().is_empty(), "{}", r.data);
    let v = validate_shacl(&r.data, &shapes);
    assert!(v.failing.is_empty(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn a_dangling_qualified_value_fails_only_below_the_qualified_minimum() {
    assert_shacl_exact(
        &shacl_team(),
        "rate = 1.0\noperators = [\"dangling-reference\"]\nshapes = [\"ex:TeamShape\"]",
        "dangling-reference",
    )
    .await;
}

#[tokio::test]
async fn an_unmatched_shape_reference_fails_in_shex() {
    let schema = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:TeamShape { a [ex:Team] ; ex:member @ex:PersonShape {2,3} }
ex:PersonShape { a [ex:Person] ; ex:name xsd:string }"#;
    assert_shex_exact(
        schema,
        "rate = 1.0\noperators = [\"unmatched-value\"]\nshapes = [\"ex:TeamShape\"]",
        "unmatched-value",
    )
    .await;
}

const SHEX_RECURSIVE: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:knows @ex:PersonShape {1,2} }"#;

#[tokio::test]
async fn recursive_shapes_are_generated_valid() {
    let r = run(Fixture::shex(SHEX_RECURSIVE), "rate = 0.0", 16).await;
    let v = validate_shex(&r.data, SHEX_RECURSIVE, &r.shapemap);
    assert!(v.conforms(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn a_failure_propagates_transitively_through_recursive_references() {
    let r = run(
        Fixture::shex(SHEX_RECURSIVE),
        "rate = 0.2\noperators = [\"wrong-datatype\"]",
        12,
    )
    .await;
    assert!(!r.direct().is_empty(), "{}", r.data);
    let induced = r.labels.iter().filter(|l| l["kind"] == "induced").count();
    assert!(
        induced > r.direct().len(),
        "chains of knows should induce more failures: {:?}",
        r.labels
    );
    let v = validate_shex(&r.data, SHEX_RECURSIVE, &r.shapemap);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn a_broken_member_fails_a_qualified_team_only_below_the_minimum() {
    let shapes = shacl_team();
    let r = run(
        Fixture::shacl(&shapes),
        "rate = 0.4\noperators = [\"wrong-datatype\"]\nshapes = [\"ex:PersonShape\"]",
        40,
    )
    .await;
    assert!(!r.direct().is_empty(), "{}", r.data);
    let v = validate_shacl(&r.data, &shapes);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn an_extra_value_does_not_count_towards_a_qualified_minimum() {
    // A team with a trap extra member and a broken member keeps only one
    // conforming member, below the qualified minimum of 2.
    let shapes = shacl_team();
    let r = run(
        Fixture::shacl(&shapes),
        "rate = 0.6\noperators = [\"trap-extra-value\", \"drop-required\"]",
        40,
    )
    .await;
    assert!(!r.traps().is_empty() && !r.direct().is_empty(), "{:?}", r.labels);
    let v = validate_shacl(&r.data, &shapes);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.data);
}

#[tokio::test]
async fn an_extra_value_does_not_count_towards_a_minimum_under_shex_extra() {
    let schema = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:TeamShape EXTRA ex:member { a [ex:Team] ; ex:member @ex:PersonShape {2,4} }
ex:PersonShape { a [ex:Person] ; ex:name xsd:string }"#;
    let r = run(
        Fixture::shex(schema),
        "rate = 0.6\noperators = [\"trap-extra-value\", \"drop-required\"]",
        40,
    )
    .await;
    assert!(!r.traps().is_empty() && !r.direct().is_empty(), "{:?}", r.labels);
    let v = validate_shex(&r.data, schema, &r.shapemap);
    assert_eq!(v.failing, r.expected_failing(), "{}\n{}", v.report, r.data);
}
