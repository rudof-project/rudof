//! Data generation: the output must be valid against the schema it came from,
//! checked with the bindings' own validators.

#![cfg(feature = "generate")]

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use rudof_wasm::{generate, validate_shacl, validate_shex};

const SHEX: &str = r#"
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer ? ; :worksFor @:Org }
:Org { :label xsd:string }
"#;

const SHACL: &str = r#"
@prefix : <http://example.org/> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
:PersonShape a sh:NodeShape ; sh:targetClass :Person ;
  sh:property [ sh:path :name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .
"#;

const PGSCHEMA: &str = r#"
CREATE NODE TYPE ( PersonType : Person { name: STRING }) ;
CREATE NODE TYPE ( OrgType : Org { label: STRING }) ;
CREATE EDGE TYPE (:@PersonType) -[WorksForType : worksFor]-> (:@OrgType)
"#;

#[test]
fn shex_data_is_valid_against_its_schema() {
    let generated = generate(SHEX, "shex", None, Some(30), Some(7)).unwrap();
    let report = validate_shex(&generated.data, SHEX, &generated.shapemap, None, None).unwrap();
    assert!(report.conforms && !report.entries.is_empty(), "{report:?}");
}

#[test]
fn shacl_data_is_valid_against_its_shapes() {
    let generated = generate(SHACL, "shacl", None, Some(10), Some(7)).unwrap();
    let report = validate_shacl(&generated.data, SHACL, None, None, None, None).unwrap();
    assert!(report.conforms, "{report:?}");
}

#[test]
fn a_pgschema_gives_a_property_graph_and_violations_give_a_report() {
    let config = "version = 2\n[violations]\nrate = 0.5\n";
    let generated = generate(PGSCHEMA, "pgschema", Some(config), Some(20), Some(7)).unwrap();
    assert!(generated.property_graph.is_some_and(|pg| !pg.is_empty()));
    assert!(generated.type_map.is_some_and(|m| !m.is_empty()));
    assert!(generated.labels.is_some_and(|l| !l.is_empty()));
}

#[test]
fn violations_fail_the_shex_validator_and_the_same_seed_repeats() {
    let config = "version = 2\n[violations]\nrate = 1.0\noperators = [\"drop-required\"]\n";
    let broken = generate(SHEX, "shex", Some(config), Some(10), Some(3)).unwrap();
    let report = validate_shex(&broken.data, SHEX, &broken.shapemap, None, None).unwrap();
    assert!(!report.conforms);
    let again = generate(SHEX, "shex", Some(config), Some(10), Some(3)).unwrap();
    assert_eq!(broken.labels, again.labels);
}

#[test]
fn an_unknown_schema_format_is_a_range_error() {
    let err = generate(SHEX, "owl", None, None, None).unwrap_err();
    assert_eq!(err.name(), "RangeError");
}
