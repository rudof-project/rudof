//! Runs natively with `cargo test -p rudof_wasm`, and on `wasm` with
//! `wasm-bindgen-test-runner` as the cargo runner (see the crate README).

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use rudof_wasm::{QueryResults, Session, ShExValidationReport, validate_shacl, validate_shex};

const DATA: &str = r#"
prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 .
:bob   :name "Bob"   ; :age "unknown" .
"#;

const SHEX_SCHEMA: &str = r#"
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer }
"#;

const SHACL_SHAPES: &str = r#"
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:PersonShape a sh:NodeShape ;
  sh:targetSubjectsOf :name ;
  sh:property [ sh:path :age ; sh:datatype xsd:integer ] .
"#;

/// A SHACL-SPARQL constraint (`sh:sparql`), which needs a SPARQL engine over
/// the data even when the core constraints are validated natively.
const SHACL_SPARQL_SHAPES: &str = r#"
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
:AdultShape a sh:NodeShape ;
  sh:targetNode :alice, :carol ;
  sh:sparql [
    sh:message "Must be at least 18 years old" ;
    sh:select """
      PREFIX : <http://example.org/>
      SELECT $this WHERE { $this :age ?age . FILTER (?age < 18) }
    """
  ] .
"#;

const AGES: &str = r#"
prefix : <http://example.org/>
:alice :age 23 .
:carol :age 12 .
"#;

fn status_of(report: &ShExValidationReport, node: &str) -> String {
    report
        .entries
        .iter()
        .find(|e| e.node == node)
        .unwrap_or_else(|| panic!("no result for {node} in {report:?}"))
        .status
        .clone()
}

// One-shot validation

#[test]
fn shex_conformant_node() {
    let report = validate_shex(DATA, SHEX_SCHEMA, ":alice@:Person", None, None).unwrap();
    assert!(report.conforms);
    assert!(report.violations.is_empty());
    assert_eq!(status_of(&report, "http://example.org/alice"), "conformant");
}

#[test]
fn shex_mixed_nodes() {
    let report = validate_shex(DATA, SHEX_SCHEMA, ":alice@:Person, :bob@:Person", None, None).unwrap();
    assert!(!report.conforms);
    assert_eq!(report.entries.len(), 2);
    assert_eq!(status_of(&report, "http://example.org/alice"), "conformant");
    assert_eq!(status_of(&report, "http://example.org/bob"), "nonconformant");
    assert_eq!(report.violations.len(), 1);
    assert_eq!(report.violations[0].node, "http://example.org/bob");
}

#[test]
fn shex_with_base_and_ntriples() {
    let data = r#"<http://example.org/carol> <http://example.org/name> "Carol" .
<http://example.org/carol> <http://example.org/age> "30"^^<http://www.w3.org/2001/XMLSchema#integer> .
"#;
    let schema = r#"prefix xsd: <http://www.w3.org/2001/XMLSchema#>
<Person> { <name> xsd:string ; <age> xsd:integer }"#;
    let report = validate_shex(
        data,
        schema,
        "<carol>@<Person>",
        Some("ntriples"),
        Some("http://example.org/"),
    )
    .unwrap();
    assert!(report.conforms, "{report:?}");
}

#[test]
fn shex_query_node_selectors() {
    let shapemaps = [
        r#"SPARQL "PREFIX : <http://example.org/> SELECT ?p WHERE { ?p :name ?n }"@:Person"#,
        "{FOCUS :name _}@:Person",
    ];
    for shapemap in shapemaps {
        let report = validate_shex(DATA, SHEX_SCHEMA, shapemap, None, None).unwrap();
        assert_eq!(report.entries.len(), 2, "{shapemap}: {report:?}");
        assert_eq!(status_of(&report, "http://example.org/alice"), "conformant");
        assert_eq!(status_of(&report, "http://example.org/bob"), "nonconformant");
    }
}

#[test]
fn shacl_conformant_data() {
    let data = r#"prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 ."#;
    let report = validate_shacl(data, SHACL_SHAPES, None, None, None, None).unwrap();
    assert!(report.conforms);
    assert!(report.entries.is_empty());
}

#[test]
fn shacl_violation() {
    for mode in ["native", "sparql"] {
        let report = validate_shacl(DATA, SHACL_SHAPES, None, None, None, Some(mode)).unwrap();
        assert!(!report.conforms, "{mode}");
        assert_eq!(report.entries.len(), 1, "{mode}: {report:?}");
        let entry = &report.entries[0];
        assert_eq!(entry.focus_node, "http://example.org/bob");
        assert_eq!(entry.path.as_deref(), Some("http://example.org/age"));
        assert!(entry.severity.contains("Violation"), "{entry:?}");
        assert_eq!(report.violations, report.entries);
    }
}

#[test]
fn shacl_sparql_constraint() {
    for mode in ["native", "sparql"] {
        let report = validate_shacl(AGES, SHACL_SPARQL_SHAPES, None, None, None, Some(mode)).unwrap();
        assert!(!report.conforms, "{mode}");
        assert_eq!(report.entries.len(), 1, "{mode}: {report:?}");
        let entry = &report.entries[0];
        assert_eq!(entry.focus_node, "http://example.org/carol");
        assert_eq!(entry.messages[0].text, "Must be at least 18 years old");
        assert_eq!(entry.messages[0].lang, None);
    }
}

// Errors

#[test]
fn errors_have_categories() {
    let err = validate_shex(DATA, "this is not ShEx", ":alice@:Person", None, None).unwrap_err();
    assert_eq!(err.name(), "ShExError", "{err}");

    let err = validate_shacl(DATA, SHACL_SHAPES, Some("no-such-format"), None, None, None).unwrap_err();
    assert_eq!(err.name(), "RangeError", "{err}");
    assert!(err.message().contains("no-such-format"), "{err}");

    let err = validate_shacl(DATA, SHACL_SHAPES, None, None, None, Some("magic")).unwrap_err();
    assert_eq!(err.name(), "RangeError", "{err}");

    let err = validate_shex("this is not RDF", SHEX_SCHEMA, ":alice@:Person", None, None).unwrap_err();
    assert_eq!(err.name(), "DataError", "{err}");
}

// Sessions

#[test]
fn session_shex_workflow() {
    let mut rudof = Session::new(None);
    rudof.read_data(DATA, None, None, None, None).unwrap();
    rudof.read_shex(SHEX_SCHEMA, None, None, None).unwrap();
    rudof
        .read_shapemap(":alice@:Person, :bob@:Person", None, None, None)
        .unwrap();

    let report = rudof.validate_shex().unwrap();
    assert!(!report.conforms);
    assert_eq!(report.entries.len(), 2);

    let schema = rudof.serialize_current_shex(Some("shexj"), None).unwrap();
    assert!(schema.contains("\"type\""), "{schema}");
    let shapemap = rudof.serialize_shapemap(None).unwrap();
    assert!(shapemap.contains("alice"), "{shapemap}");
    let results = rudof.serialize_shex_validation_results(Some("compact"), None).unwrap();
    assert!(results.contains("bob"), "{results}");
    for format in ["details", "json", "csv"] {
        let results = rudof.serialize_shex_validation_results(Some(format), None).unwrap();
        assert!(results.contains("bob"), "{format}: {results}");
    }
    // RDF formats are not implemented for ShEx results: an error, not a panic
    // (which would abort the wasm module).
    let err = rudof
        .serialize_shex_validation_results(Some("turtle"), None)
        .unwrap_err();
    assert!(err.to_string().contains("turtle"), "{err}");

    // Validating again with another ShapeMap reuses the data and schema.
    rudof.reset_shapemap();
    rudof.read_shapemap(":alice@:Person", None, None, None).unwrap();
    assert!(rudof.validate_shex().unwrap().conforms);
}

#[test]
fn session_shacl_workflow() {
    let mut rudof = Session::new(None);
    rudof.read_data(DATA, None, None, None, None).unwrap();
    rudof.read_shacl(Some(SHACL_SHAPES), None, None, None).unwrap();
    assert!(!rudof.validate_shacl(None).unwrap().conforms);

    let shapes = rudof.serialize_shacl(Some("ntriples")).unwrap();
    assert!(shapes.contains("http://www.w3.org/ns/shacl#datatype"), "{shapes}");
    let results = rudof.serialize_shacl_validation_results(Some("turtle"), None).unwrap();
    assert!(results.contains("ValidationReport"), "{results}");
    let results = rudof.serialize_shacl_validation_results(Some("minimal"), None).unwrap();
    assert!(results.contains("1 violations"), "{results}");
    // JSON is not implemented for SHACL results: an error, not a panic.
    let err = rudof
        .serialize_shacl_validation_results(Some("json"), None)
        .unwrap_err();
    assert!(err.to_string().contains("json"), "{err}");

    // Fixing the data makes it conform.
    rudof.reset_data();
    rudof
        .read_data(
            r#"<http://example.org/bob> <http://example.org/name> "Bob" ."#,
            Some("ntriples"),
            None,
            None,
            None,
        )
        .unwrap();
    assert!(rudof.validate_shacl(Some("sparql")).unwrap().conforms);
}

#[test]
fn session_shacl_shapes_from_data() {
    let mut rudof = Session::new(None);
    rudof
        .read_data(&format!("{DATA}\n{SHACL_SHAPES}"), None, None, None, None)
        .unwrap();
    rudof.read_shacl(None, None, None, None).unwrap();
    let report = rudof.validate_shacl(None).unwrap();
    assert_eq!(report.entries.len(), 1, "{report:?}");
}

#[test]
fn session_merges_data() {
    let mut rudof = Session::new(None);
    rudof
        .read_data(
            r#"<http://example.org/a> <http://example.org/p> "1" ."#,
            Some("ntriples"),
            None,
            None,
            None,
        )
        .unwrap();
    rudof
        .read_data(
            r#"<http://example.org/b> <http://example.org/p> "2" ."#,
            Some("ntriples"),
            None,
            None,
            Some(true),
        )
        .unwrap();
    let data = rudof.serialize_data(Some("ntriples")).unwrap();
    assert!(
        data.contains("http://example.org/a") && data.contains("http://example.org/b"),
        "{data}"
    );
}

#[test]
fn session_sparql_queries() {
    let mut rudof = Session::new(None);
    rudof.read_data(DATA, None, None, None, None).unwrap();

    rudof
        .read_query(
            "PREFIX : <http://example.org/> SELECT ?name WHERE { ?p :name ?name } ORDER BY ?name",
            None,
        )
        .unwrap();
    match rudof.run_query().unwrap() {
        QueryResults::Select { variables, rows } => {
            assert_eq!(variables.len(), 1, "{variables:?}");
            assert!(variables[0].contains("name"), "{variables:?}");
            assert_eq!(rows.len(), 2, "{rows:?}");
            assert!(rows[0][0].as_deref().unwrap().contains("Alice"), "{rows:?}");
        },
        other => panic!("expected SELECT results, got {other:?}"),
    }
    assert!(!rudof.serialize_query_results(None).unwrap().is_empty());

    rudof.reset_query();
    rudof
        .read_query("PREFIX : <http://example.org/> ASK { :bob :age ?a }", None)
        .unwrap();
    assert_eq!(rudof.run_query().unwrap(), QueryResults::Ask { boolean: true });

    rudof.reset_query();
    rudof
        .read_query(
            "PREFIX : <http://example.org/> CONSTRUCT { ?p a :Person } WHERE { ?p :name ?n }",
            None,
        )
        .unwrap();
    match rudof.run_query().unwrap() {
        QueryResults::Graph { graph } => assert!(graph.contains("Person"), "{graph}"),
        other => panic!("expected a graph, got {other:?}"),
    }
}

#[test]
fn session_prefixes() {
    let mut rudof = Session::new(None);
    rudof.add_prefix("ex", "http://example.org/").unwrap();
    assert!(
        rudof
            .prefixes()
            .contains(&("ex".to_string(), "http://example.org/".to_string()))
    );
    rudof.rename_prefix("ex", "example").unwrap();
    assert!(rudof.prefixes().iter().any(|(alias, _)| alias == "example"));
    rudof.remove_prefix("example").unwrap();
    assert!(!rudof.prefixes().iter().any(|(alias, _)| alias == "example"));
}

#[test]
fn session_errors_without_loaded_state() {
    let mut rudof = Session::new(None);
    rudof.read_data(DATA, None, None, None, None).unwrap();
    let err = rudof.validate_shex().unwrap_err();
    assert_eq!(err.name(), "ShExError", "{err}");
    rudof.reset_all();
    assert!(rudof.validate_shacl(None).is_err());
}

#[test]
fn session_version() {
    assert!(!Session::new(None).version().is_empty());
}
