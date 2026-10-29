//! Configuration format v2 (see docs/plan/config-design.md in the super repo):
//! strict parsing, scoped policies (property > shape > defaults), CURIEs.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::{CardinalityStrategy, EntityDistribution};

const EX: &str = "http://example.org/";

fn parse(toml: &str) -> GeneratorConfig {
    GeneratorConfig::from_toml_str(toml).unwrap_or_else(|e| panic!("config should parse: {e}"))
}

fn parse_err(toml: &str) -> String {
    match GeneratorConfig::from_toml_str(toml) {
        Ok(_) => panic!("config should be rejected:\n{toml}"),
        Err(e) => e.to_string(),
    }
}

// ---------------------------------------------------------------- parsing

#[test]
fn a_minimal_v2_file_parses_with_defaults() {
    let config = parse("version = 2");
    let default = GeneratorConfig::default();
    assert_eq!(config.generation.entity_count, default.generation.entity_count);
    assert_eq!(config.generation.property_fill_probability, 1.0);
}

#[test]
fn run_and_defaults_are_applied() {
    let config = parse(
        r#"
version = 2
[run]
seed = 42
threads = 3
scale = { entities = 500 }
[defaults]
fill = 0.25
cardinality = "max"
unbounded_cap = 7
"#,
    );
    assert_eq!(config.generation.seed, Some(42));
    assert_eq!(config.parallel.worker_threads, Some(3));
    assert_eq!(config.generation.entity_count, 500);
    assert_eq!(config.generation.property_fill_probability, 0.25);
    assert!(matches!(
        config.generation.cardinality_strategy,
        CardinalityStrategy::Maximum
    ));
    assert_eq!(config.generation.unbounded_property_values, 7);
    assert_eq!(config.generation.unbounded_reference_values, 7);
}

#[test]
fn an_unknown_key_is_rejected_and_named() {
    let err = parse_err("version = 2\n[defaults]\nfil = 0.2\n");
    assert!(err.contains("fil"), "error should name the unknown key: {err}");
}

#[test]
fn an_unsupported_version_is_rejected() {
    let err = parse_err("version = 3");
    assert!(err.contains("version"), "{err}");
}

#[test]
fn sections_that_are_designed_but_not_implemented_are_rejected_explicitly() {
    for section in ["targets", "profile", "validate"] {
        let err = parse_err(&format!("version = 2\n[{section}]\n"));
        assert!(
            err.contains(section) && err.contains("not yet supported"),
            "[{section}] must not be silently ignored: {err}"
        );
    }
}

#[test]
fn a_fill_outside_zero_one_is_rejected() {
    let err = parse_err("version = 2\n[defaults]\nfill = 1.5\n");
    assert!(err.contains("fill"), "{err}");
}

#[test]
fn curies_are_expanded_in_shape_matches() {
    let config = parse(
        r#"
version = 2
[prefixes]
ex = "http://example.org/"
[[shape]]
match = "ex:PersonShape"
fill = 0.5
"#,
    );
    let key = format!("{EX}PersonShape");
    let shape = config.generation.type_overrides.get(&key).unwrap_or_else(|| {
        panic!(
            "expected an override for {key}: {:?}",
            config.generation.type_overrides.keys()
        )
    });
    assert_eq!(shape.property_fill_probability, Some(0.5));
}

#[test]
fn an_undeclared_prefix_is_rejected() {
    let err = parse_err("version = 2\n[[shape]]\nmatch = \"nope:Thing\"\n");
    assert!(err.contains("nope"), "{err}");
}

#[test]
fn shape_counts_become_an_explicit_distribution() {
    let config = parse(
        r#"
version = 2
[[shape]]
match = "http://example.org/A"
count = 10
[[shape]]
match = "http://example.org/B"
count = 5
"#,
    );
    match &config.generation.entity_distribution {
        EntityDistribution::Custom(counts) => {
            assert_eq!(counts.get(&format!("{EX}A")), Some(&10));
            assert_eq!(counts.get(&format!("{EX}B")), Some(&5));
        },
        other => panic!("expected Custom, got {other:?}"),
    }
}

#[test]
fn shape_weights_become_a_weighted_distribution() {
    let config = parse("version = 2\n[[shape]]\nmatch = \"http://example.org/A\"\nweight = 2.0\n");
    assert!(matches!(
        config.generation.entity_distribution,
        EntityDistribution::Weighted(_)
    ));
}

#[test]
fn mixing_counts_and_weights_is_rejected() {
    let err = parse_err(
        "version = 2\n[[shape]]\nmatch = \"http://example.org/A\"\ncount = 3\n[[shape]]\nmatch = \"http://example.org/B\"\nweight = 1.0\n",
    );
    assert!(err.contains("count") && err.contains("weight"), "{err}");
}

#[test]
fn v1_files_still_load() {
    let config = parse("[generation]\nentity_count = 12\n");
    assert_eq!(config.generation.entity_count, 12);
}

// ---------------------------------------------------------------- behaviour

const SCHEMA: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:AShape { a [ex:A] ; ex:name xsd:string ; ex:nick xsd:string ? ; ex:mail xsd:string ? }
ex:BShape { a [ex:B] ; ex:name xsd:string ; ex:nick xsd:string ? }
"#;

async fn generate_with(toml: &str) -> String {
    let fixture = Fixture::shex(SCHEMA);
    let mut config = parse(toml);
    config.output.path = fixture.output();
    config.output.write_stats = false;
    generate(&fixture, config).await
}

/// Number of triples with `predicate` (the Turtle writer puts one triple per line).
fn count(data: &str, predicate: &str) -> usize {
    data.lines()
        .filter(|l| l.contains(&format!("<{EX}{predicate}>")))
        .count()
}

#[tokio::test]
async fn a_property_rule_overrides_the_default_fill_for_that_property_only() {
    let data = generate_with(
        r#"
version = 2
[prefixes]
ex = "http://example.org/"
[run]
seed = 1
scale = { entities = 20 }
[[property]]
match = { path = "ex:nick" }
fill = 0.0
"#,
    )
    .await;
    assert_eq!(count(&data, "nick"), 0, "ex:nick must be suppressed everywhere\n{data}");
    assert!(
        count(&data, "mail") > 0,
        "ex:mail keeps the default fill of 1.0\n{data}"
    );
    assert!(
        count(&data, "name") >= 20,
        "required ex:name is never suppressed\n{data}"
    );
}

#[tokio::test]
async fn a_shape_scoped_property_rule_beats_an_unscoped_one() {
    let data = generate_with(
        r#"
version = 2
[prefixes]
ex = "http://example.org/"
[run]
seed = 1
scale = { entities = 20 }
[[property]]
match = { path = "ex:nick" }
fill = 0.0
[[property]]
match = { shape = "ex:AShape", path = "ex:nick" }
fill = 1.0
"#,
    )
    .await;
    let nick_subjects: Vec<&str> = data.lines().filter(|l| l.contains(&format!("<{EX}nick>"))).collect();
    assert!(!nick_subjects.is_empty(), "AShape keeps ex:nick\n{data}");
    let graph_b = instances_of(&data, &format!("{EX}B"));
    for b in graph_b {
        assert!(
            !data
                .lines()
                .any(|l| l.starts_with(&b) && l.contains(&format!("<{EX}nick>"))),
            "{b} (BShape) must not have ex:nick"
        );
    }
}

#[tokio::test]
async fn a_rule_cannot_suppress_a_required_property() {
    let data = generate_with(
        r#"
version = 2
[prefixes]
ex = "http://example.org/"
[run]
seed = 1
scale = { entities = 10 }
[[property]]
match = { path = "ex:name" }
fill = 0.0
"#,
    )
    .await;
    let verdict = validate_shex(
        &data,
        SCHEMA,
        "{FOCUS a <http://example.org/A>}@<http://example.org/AShape>",
    );
    assert!(verdict.conforms() && verdict.conformant > 0, "{}", verdict.report);
}

#[test]
fn every_shipped_example_config_parses() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut checked = 0;
    for entry in walk(&dir) {
        if entry.extension().is_some_and(|e| e == "toml") {
            GeneratorConfig::from_toml_file(&entry)
                .unwrap_or_else(|e| panic!("{} does not parse: {e}", entry.display()));
            checked += 1;
        }
    }
    assert!(checked > 0, "no example configs found in {}", dir.display());
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(walk(&path));
        } else {
            files.push(path);
        }
    }
    files
}
