//! Conformance: generated data must validate against the schema it came from,
//! checked by rudof's own ShEx and SHACL validators (never by the generator's IR).
//!
//! Construct matrix (✓ covered here, · not expressible in that language):
//!
//! | construct                         | ShEx | SHACL |
//! |-----------------------------------|------|-------|
//! | required literal, datatype        |  ✓   |  ✓    |
//! | cardinality interval {2,3}        |  ✓   |  ✓    |
//! | optional property                 |  ✓   |  ✓    |
//! | shape reference                   |  ✓   |  ✓    |
//! | pattern                           |  ✓   |  ✓    |
//! | numeric range                     |  ✓   |  ✓    |
//! | value set / sh:in                 |  ✓   |  ✓    |
//! | low fill keeps required props     |  ✓   |  ✓    |
//! | max_properties keeps required     |  ✓   |  ✓    |
//!
//! Oracle sanity tests come first: if the oracle cannot see a violation in data
//! written by hand to contain one, every other test here would be meaningless.
#![cfg(not(target_family = "wasm"))]

#[path = "../common/mod.rs"]
mod common;
mod violations;

use common::*;

const EX: &str = "http://example.org/";

fn shex_schema(body: &str) -> String {
    format!(
        "PREFIX ex: <{EX}>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\nPREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n{body}"
    )
}

fn shacl_schema(body: &str) -> String {
    format!(
        "@prefix sh: <http://www.w3.org/ns/shacl#> .\n@prefix ex: <{EX}> .\n@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n{body}"
    )
}

/// Every instance of `ex:Person` must satisfy `ex:PersonShape`.
const PERSON_SHAPEMAP: &str = "{FOCUS a <http://example.org/Person>}@<http://example.org/PersonShape>";

async fn assert_shex_conforms(body: &str, tweak: impl FnOnce(&mut rudof_generate::GeneratorConfig)) {
    let schema = shex_schema(body);
    let fixture = Fixture::shex(&schema);
    let mut cfg = config(&fixture.output(), 30, 7);
    tweak(&mut cfg);
    let data = generate(&fixture, cfg).await;
    assert!(
        !instances_of(&data, &format!("{EX}Person")).is_empty(),
        "no ex:Person generated; the shapemap would vacuously pass\n{data}"
    );
    let verdict = validate_shex(&data, &schema, PERSON_SHAPEMAP);
    assert!(
        verdict.conforms() && verdict.conformant > 0,
        "ShEx violations: {}\n--- report ---\n{}\n--- data ---\n{data}",
        verdict.violations,
        verdict.report
    );
}

async fn assert_shacl_conforms(body: &str, tweak: impl FnOnce(&mut rudof_generate::GeneratorConfig)) {
    let schema = shacl_schema(body);
    let fixture = Fixture::shacl(&schema);
    let mut cfg = config(&fixture.output(), 30, 7);
    tweak(&mut cfg);
    let data = generate(&fixture, cfg).await;
    assert!(
        !instances_of(&data, &format!("{EX}Person")).is_empty(),
        "no ex:Person generated; validation would vacuously pass\n{data}"
    );
    let verdict = validate_shacl(&data, &schema);
    assert!(
        verdict.conforms(),
        "SHACL violations: {}\n--- report ---\n{}\n--- data ---\n{data}",
        verdict.violations,
        verdict.report
    );
}

fn keep(_: &mut rudof_generate::GeneratorConfig) {}

// ---------------------------------------------------------------- oracle sanity

const SANITY_SHEX: &str = "ex:PersonShape { a [ex:Person] ; ex:name xsd:string }";
const SANITY_SHACL: &str = "ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;\n  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:datatype xsd:string ] .";

#[test]
fn shex_oracle_accepts_valid_data() {
    let data = format!("<{EX}p1> a <{EX}Person> ; <{EX}name> \"Ann\" .");
    let v = validate_shex(&data, &shex_schema(SANITY_SHEX), PERSON_SHAPEMAP);
    assert!(v.conforms() && v.conformant == 1, "{}", v.report);
}

#[test]
fn shex_oracle_flags_a_missing_required_property() {
    let data = format!("<{EX}p1> a <{EX}Person> .");
    let v = validate_shex(&data, &shex_schema(SANITY_SHEX), PERSON_SHAPEMAP);
    assert_eq!(v.violations, 1, "{}", v.report);
}

#[test]
fn shacl_oracle_accepts_valid_data() {
    let data = format!("<{EX}p1> a <{EX}Person> ; <{EX}name> \"Ann\" .");
    let v = validate_shacl(&data, &shacl_schema(SANITY_SHACL));
    assert!(v.conforms(), "{}", v.report);
}

#[test]
fn shacl_oracle_flags_a_missing_required_property() {
    let data = format!("<{EX}p1> a <{EX}Person> .");
    let v = validate_shacl(&data, &shacl_schema(SANITY_SHACL));
    assert_eq!(v.violations, 1, "{}", v.report);
}

// ---------------------------------------------------------------- constructs: ShEx

#[tokio::test]
async fn shex_required_literals_with_datatypes_conform() {
    assert_shex_conforms(
        "ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:age xsd:integer }",
        keep,
    )
    .await;
}

#[tokio::test]
async fn shex_cardinality_interval_conforms() {
    assert_shex_conforms("ex:PersonShape { a [ex:Person] ; ex:nick xsd:string {2,3} }", keep).await;
}

#[tokio::test]
async fn shex_optional_property_conforms() {
    assert_shex_conforms(
        "ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:nick xsd:string ? }",
        keep,
    )
    .await;
}

#[tokio::test]
async fn shex_shape_reference_conforms() {
    assert_shex_conforms(
        "ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:worksFor @ex:OrgShape }\n\
         ex:OrgShape { a [ex:Org] ; ex:label xsd:string }",
        keep,
    )
    .await;
}

#[tokio::test]
async fn shex_pattern_conforms() {
    assert_shex_conforms(
        r#"ex:PersonShape { a [ex:Person] ; ex:code xsd:string /^[A-Z]{3}-[0-9]{4}$/ }"#,
        keep,
    )
    .await;
}

#[tokio::test]
async fn shex_numeric_range_conforms() {
    assert_shex_conforms(
        "ex:PersonShape { a [ex:Person] ; ex:age xsd:integer MININCLUSIVE 18 MAXINCLUSIVE 65 }",
        keep,
    )
    .await;
}

#[tokio::test]
async fn shex_value_set_conforms() {
    assert_shex_conforms(
        r#"ex:PersonShape { a [ex:Person] ; ex:status ["active" "retired"] }"#,
        keep,
    )
    .await;
}

#[tokio::test]
async fn shex_low_fill_probability_keeps_required_properties() {
    assert_shex_conforms(
        "ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:nick xsd:string ? ; ex:mail xsd:string * }",
        |c| c.generation.property_fill_probability = 0.0,
    )
    .await;
}

#[tokio::test]
async fn shex_max_properties_per_instance_keeps_required_properties() {
    assert_shex_conforms(
        "ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:age xsd:integer ; ex:nick xsd:string ? }",
        |c| c.generation.max_properties_per_instance = 1,
    )
    .await;
}

// ---------------------------------------------------------------- constructs: SHACL

const SHACL_PERSON: &str = "ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;";

#[tokio::test]
async fn shacl_required_literals_with_datatypes_conform() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:string ] ;\n  sh:property [ sh:path ex:age ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:integer ] ."
        ),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_cardinality_interval_conforms() {
    assert_shacl_conforms(
        &format!("{SHACL_PERSON}\n  sh:property [ sh:path ex:nick ; sh:minCount 2 ; sh:maxCount 3 ; sh:datatype xsd:string ] ."),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_optional_property_conforms() {
    assert_shacl_conforms(
        &format!("{SHACL_PERSON}\n  sh:property [ sh:path ex:nick ; sh:maxCount 1 ; sh:datatype xsd:string ] ."),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_shape_reference_conforms() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:worksFor ; sh:minCount 1 ; sh:maxCount 1 ; sh:node ex:OrgShape ] .\n\
             ex:OrgShape a sh:NodeShape ; sh:targetClass ex:Org ;\n  sh:property [ sh:path ex:label ; sh:minCount 1 ; sh:datatype xsd:string ] ."
        ),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_pattern_conforms() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:code ; sh:minCount 1 ; sh:datatype xsd:string ; sh:pattern \"^[A-Z]{{3}}-[0-9]{{4}}$\" ] ."
        ),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_numeric_range_conforms() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:age ; sh:minCount 1 ; sh:datatype xsd:integer ; sh:minInclusive 18 ; sh:maxInclusive 65 ] ."
        ),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_value_set_conforms() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:status ; sh:minCount 1 ; sh:in ( \"active\" \"retired\" ) ] ."
        ),
        keep,
    )
    .await;
}

#[tokio::test]
async fn shacl_low_fill_probability_keeps_required_properties() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:datatype xsd:string ] ;\n  sh:property [ sh:path ex:nick ; sh:datatype xsd:string ] ."
        ),
        |c| c.generation.property_fill_probability = 0.0,
    )
    .await;
}

#[tokio::test]
async fn shacl_max_properties_per_instance_keeps_required_properties() {
    assert_shacl_conforms(
        &format!(
            "{SHACL_PERSON}\n  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:datatype xsd:string ] ;\n  sh:property [ sh:path ex:age ; sh:minCount 1 ; sh:datatype xsd:integer ] ;\n  sh:property [ sh:path ex:nick ; sh:datatype xsd:string ] ."
        ),
        |c| c.generation.max_properties_per_instance = 1,
    )
    .await;
}

// ---------------------------------------------------------------- rudof_lib: generate --validate

async fn generate_and_validate(
    schema_name: &str,
    schema: &str,
    config_toml: Option<&str>,
) -> rudof_lib::GenerationValidation {
    use rudof_lib::formats::{GenerationSchemaFormat, InputSpec};
    let dir = tempfile::TempDir::new().unwrap();
    let schema_path = dir.path().join(schema_name);
    std::fs::write(&schema_path, schema).unwrap();
    let output = dir.path().join("out.ttl");
    let config_path = dir.path().join("gen.toml");
    let rudof = rudof_lib::Rudof::new(rudof_lib::RudofConfig::default());
    let input = InputSpec::path(&schema_path);
    let mut builder = rudof
        .generate_data(&input, &GenerationSchemaFormat::Auto, Some(25))
        .with_seed(3)
        .with_output(&output)
        .with_validation(true);
    if let Some(toml) = config_toml {
        std::fs::write(&config_path, toml).unwrap();
        builder = builder.with_config_file(&config_path);
    }
    builder
        .execute_with_report()
        .await
        .expect("generate + validate")
        .expect("validation requested, so a report is returned")
}

const LIB_SHEX: &str = "PREFIX ex: <http://example.org/>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n\
    ex:PersonShape { a [ex:Person] ; ex:name xsd:string ; ex:knows @ex:PersonShape * }";

#[tokio::test]
async fn generate_with_validation_reports_shex_conformance() {
    let outcome = generate_and_validate("s.shex", LIB_SHEX, None).await;
    assert_eq!(outcome.language, "ShEx");
    assert_eq!(outcome.checked, 25);
    assert!(outcome.conforms(), "{outcome:?}");
}

#[tokio::test]
async fn generate_with_validation_reports_shacl_conformance() {
    let shapes = shacl_schema(&format!(
        "{SHACL_PERSON}\n  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:datatype xsd:string ] ."
    ));
    let outcome = generate_and_validate("s.ttl", &shapes, None).await;
    assert_eq!(outcome.language, "SHACL");
    assert!(outcome.conforms() && outcome.checked == 25, "{outcome:?}");
}

#[tokio::test]
async fn generate_with_validation_catches_dropped_required_properties() {
    // ignore_min_cardinality deliberately breaks the schema; the loop must see it.
    let config = "[generation]\nignore_min_cardinality = true\nproperty_fill_probability = 0.0\n";
    let outcome = generate_and_validate("s.shex", LIB_SHEX, Some(config)).await;
    assert!(
        !outcome.conforms(),
        "dropped required properties must be reported: {outcome:?}"
    );
    assert_eq!(outcome.violations, 25, "{outcome:?}");
}
