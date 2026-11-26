//! ShEx and SHACL validation through the `Rudof` API with in-memory inputs.
//!
//! These tests run natively with `cargo test -p rudof_lib`, and on `wasm` with
//! `wasm-bindgen-test-runner` as the cargo runner, where plain `#[test]`s are
//! not executed.
//!
//! On `wasm` there is no current directory to derive a default base IRI from,
//! so the configuration sets `base_iri`, which applies to data, schemas and
//! shapemaps.

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use rudof_lib::{
    Rudof, RudofConfig,
    formats::{InputSpec, ShaclValidationMode},
};
use shex_ast::shapemap::ValidationStatus;
use std::str::FromStr;

const BASE: &str = "http://example.org/";

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
  sh:property [ sh:path :age ; sh:datatype xsd:integer ] ;
  sh:sparql [
    sh:message "Must be at least 18 years old" ;
    sh:select """
      PREFIX : <http://example.org/>
      SELECT $this WHERE { $this :age ?age . FILTER (isNumeric(?age) && ?age < 18) }
    """
  ] .
"#;

fn rudof_with_base() -> Rudof {
    let config = RudofConfig::from_str(&format!("base_iri = \"{BASE}\"")).unwrap();
    Rudof::new(config)
}

fn rudof_with_data(data: &str) -> Rudof {
    let mut rudof = rudof_with_base();
    rudof.load_data().with_data(&[InputSpec::str(data)]).execute().unwrap();
    rudof
}

fn shex_status(rudof: &Rudof, node: &str) -> ValidationStatus {
    rudof
        .shex_validation_results()
        .unwrap()
        .iter()
        .find(|(n, _, _)| n.to_string() == node)
        .unwrap_or_else(|| panic!("no ShEx result for {node}"))
        .2
        .clone()
}

#[test]
fn shex_validation() {
    let mut rudof = rudof_with_data(DATA);
    rudof.load_shex_schema(&InputSpec::str(SHEX_SCHEMA)).execute().unwrap();
    rudof
        .load_shapemap(&InputSpec::str(":alice@:Person, :bob@:Person"))
        .execute()
        .unwrap();
    rudof.validate_shex().execute().unwrap();

    assert!(shex_status(&rudof, "http://example.org/alice").is_conformant());
    assert!(shex_status(&rudof, "http://example.org/bob").is_non_conformant());

    let mut output = Vec::new();
    rudof.serialize_shex_validation_results(&mut output).execute().unwrap();
    assert!(!output.is_empty());
}

#[test]
fn shex_validation_with_query_selector() {
    let mut rudof = rudof_with_data(DATA);
    rudof.load_shex_schema(&InputSpec::str(SHEX_SCHEMA)).execute().unwrap();
    rudof
        .load_shapemap(&InputSpec::str("{FOCUS :name _}@:Person"))
        .execute()
        .unwrap();
    rudof.validate_shex().execute().unwrap();

    assert!(shex_status(&rudof, "http://example.org/alice").is_conformant());
    assert!(shex_status(&rudof, "http://example.org/bob").is_non_conformant());
}

#[test]
fn shacl_validation() {
    let data = format!(
        "{DATA}\n<http://example.org/carol> <http://example.org/name> \"Carol\" ; <http://example.org/age> 12 ."
    );
    for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
        let mut rudof = rudof_with_data(&data);
        rudof
            .load_shacl_shapes()
            .with_shacl_schema(&InputSpec::str(SHACL_SHAPES))
            .execute()
            .unwrap();
        rudof
            .validate_shacl()
            .with_shacl_validation_mode(&mode)
            .execute()
            .unwrap();

        let report = rudof.shacl_validation_results().unwrap();
        assert!(!report.conforms(), "{mode:?}");
        // :bob's age is not an integer; :carol is under 18 (sh:sparql).
        let mut focus_nodes: Vec<String> = report.results().iter().map(|r| r.focus_node().to_string()).collect();
        focus_nodes.sort();
        assert_eq!(
            focus_nodes,
            ["http://example.org/bob", "http://example.org/carol"],
            "{mode:?}"
        );

        let mut output = Vec::new();
        rudof.serialize_shacl_validation_results(&mut output).execute().unwrap();
        assert!(!output.is_empty());
    }
}

/// On wasm there is no current directory to derive a base IRI from.
#[cfg(target_family = "wasm")]
#[test]
fn shex_schema_requires_base_on_wasm() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let err = rudof
        .load_shex_schema(&InputSpec::str(SHEX_SCHEMA))
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("base IRI"), "{err}");
}

/// SPARQL endpoints are only available natively.
#[cfg(target_family = "wasm")]
#[test]
fn endpoints_are_rejected_on_wasm() {
    let mut rudof = rudof_with_base();
    let err = rudof
        .load_data()
        .with_endpoint("https://query.wikidata.org/sparql")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("not supported on wasm"), "{err}");
}
