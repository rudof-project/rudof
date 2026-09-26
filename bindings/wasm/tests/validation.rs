//! Runs natively with `cargo test -p rudof_wasm`, and on `wasm` with
//! `wasm-bindgen-test-runner` as the cargo runner (see the crate README).

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use rudof_wasm::{validate_shacl, validate_shex};
use serde_json::Value;

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

fn parse(json: &str) -> Value {
    serde_json::from_str(json).unwrap()
}

fn status_of(results: &Value, node: &str) -> String {
    results["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["node"] == node)
        .unwrap_or_else(|| panic!("no result for {node} in {results}"))["status"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn shex_conformant_node() {
    let results = parse(&validate_shex(DATA, SHEX_SCHEMA, ":alice@:Person", None, None).unwrap());
    assert_eq!(results["conforms"], true);
    assert_eq!(status_of(&results, "http://example.org/alice"), "conformant");
}

#[test]
fn shex_mixed_nodes() {
    let results = parse(&validate_shex(DATA, SHEX_SCHEMA, ":alice@:Person, :bob@:Person", None, None).unwrap());
    assert_eq!(results["conforms"], false);
    assert_eq!(status_of(&results, "http://example.org/alice"), "conformant");
    assert_eq!(status_of(&results, "http://example.org/bob"), "nonconformant");
}

#[test]
fn shex_with_base_and_ntriples() {
    let data = r#"<http://example.org/carol> <http://example.org/name> "Carol" .
<http://example.org/carol> <http://example.org/age> "30"^^<http://www.w3.org/2001/XMLSchema#integer> .
"#;
    let schema = r#"prefix xsd: <http://www.w3.org/2001/XMLSchema#>
<Person> { <name> xsd:string ; <age> xsd:integer }"#;
    let results = parse(
        &validate_shex(
            data,
            schema,
            "<carol>@<Person>",
            Some("ntriples"),
            Some("http://example.org/"),
        )
        .unwrap(),
    );
    assert_eq!(results["conforms"], true);
}

#[test]
fn shex_reports_schema_syntax_errors() {
    let err = validate_shex(DATA, "this is not ShEx", ":alice@:Person", None, None).unwrap_err();
    assert!(err.contains("ShEx schema"), "{err}");
}

#[test]
fn shacl_conformant_data() {
    let data = r#"prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 ."#;
    let report = parse(&validate_shacl(data, SHACL_SHAPES, None, None, None, None).unwrap());
    assert_eq!(report["conforms"], true);
    assert_eq!(report["results"].as_array().unwrap().len(), 0);
}

#[test]
fn shacl_violation() {
    let report = parse(&validate_shacl(DATA, SHACL_SHAPES, None, None, None, None).unwrap());
    assert_eq!(report["conforms"], false);
    let results = report["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0]["focusNode"].as_str().unwrap().contains("bob"), "{report}");
    assert!(
        results[0]["severity"].as_str().unwrap().contains("Violation"),
        "{report}"
    );
    assert_eq!(results[0]["path"], "http://example.org/age");
    assert!(
        !results[0]["messages"][0]["text"].as_str().unwrap().starts_with('"'),
        "{report}"
    );
}

#[test]
fn rejects_unknown_data_format() {
    let err = validate_shacl(DATA, SHACL_SHAPES, Some("no-such-format"), None, None, None).unwrap_err();
    assert!(err.contains("Unsupported RDF format"), "{err}");
}

#[test]
fn shacl_sparql_mode() {
    let report = parse(&validate_shacl(DATA, SHACL_SHAPES, None, None, None, Some("sparql")).unwrap());
    assert_eq!(report["conforms"], false);
    let results = report["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0]["focusNode"].as_str().unwrap().contains("bob"), "{report}");
}

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

#[test]
fn shacl_sparql_constraint() {
    for mode in ["native", "sparql"] {
        let report = parse(&validate_shacl(AGES, SHACL_SPARQL_SHAPES, None, None, None, Some(mode)).unwrap());
        assert_eq!(report["conforms"], false, "{mode}: {report}");
        let results = report["results"].as_array().unwrap();
        assert_eq!(results.len(), 1, "{mode}: {report}");
        assert_eq!(results[0]["focusNode"], "http://example.org/carol", "{mode}: {report}");
    }
}

#[test]
fn rejects_unknown_shacl_mode() {
    let err = validate_shacl(DATA, SHACL_SHAPES, None, None, None, Some("magic")).unwrap_err();
    assert!(err.contains("Invalid SHACL validation mode"), "{err}");
}

#[test]
fn shex_query_node_selectors() {
    let shapemaps = [
        r#"SPARQL "PREFIX : <http://example.org/> SELECT ?p WHERE { ?p :name ?n }"@:Person"#,
        "{FOCUS :name _}@:Person",
    ];
    for shapemap in shapemaps {
        let results = parse(&validate_shex(DATA, SHEX_SCHEMA, shapemap, None, None).unwrap());
        assert_eq!(results["results"].as_array().unwrap().len(), 2, "{shapemap}: {results}");
        assert_eq!(status_of(&results, "http://example.org/alice"), "conformant");
        assert_eq!(status_of(&results, "http://example.org/bob"), "nonconformant");
    }
}
