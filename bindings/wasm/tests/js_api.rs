//! Tests of the JavaScript layer: the values it returns to JavaScript and the
//! errors it throws. Only on `wasm`, with `wasm-bindgen-test-runner`.
#![cfg(target_family = "wasm")]

use rudof_wasm::js::{JsRudof, JsRudofConfig, validate_shacl, validate_shex};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test as test;

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

fn get(value: &JsValue, key: &str) -> JsValue {
    js_sys::Reflect::get(value, &JsValue::from_str(key)).unwrap()
}

fn array(value: &JsValue) -> js_sys::Array {
    value.clone().dyn_into::<js_sys::Array>().unwrap()
}

fn error_name(error: &JsValue) -> String {
    error.clone().dyn_into::<js_sys::Error>().unwrap().name().into()
}

#[test]
fn reports_are_plain_objects() {
    let report = validate_shex(DATA, SHEX_SCHEMA, ":alice@:Person, :bob@:Person", None, None).unwrap();
    assert_eq!(get(&report, "conforms"), JsValue::FALSE);
    assert_eq!(array(&get(&report, "entries")).length(), 2);
    let violation = array(&get(&report, "violations")).get(0);
    assert_eq!(get(&violation, "node"), "http://example.org/bob");
    assert_eq!(get(&violation, "status"), "nonconformant");

    let report = validate_shacl(DATA, SHACL_SHAPES, None, None, None, Some("sparql".into())).unwrap();
    assert_eq!(get(&report, "conforms"), JsValue::FALSE);
    let entry = array(&get(&report, "entries")).get(0);
    assert_eq!(get(&entry, "focusNode"), "http://example.org/bob");
    assert_eq!(get(&entry, "path"), "http://example.org/age");
    // Missing values are `null`, not `undefined`.
    assert!(get(&entry, "sourceShape").is_null() || get(&entry, "sourceShape").is_string());
}

#[test]
fn errors_are_thrown_with_their_category_as_name() {
    let err = validate_shex(DATA, "not ShEx", ":alice@:Person", None, None).unwrap_err();
    assert_eq!(error_name(&err), "ShExError");
    let err = validate_shacl(DATA, SHACL_SHAPES, Some("nope".into()), None, None, None).unwrap_err();
    assert_eq!(error_name(&err), "RangeError");
    let err = JsRudofConfig::from_toml("this is = not = toml").err().unwrap();
    assert_eq!(error_name(&err), "ConfigError");
}

#[test]
fn rudof_class_workflow() {
    let config = JsRudofConfig::from_toml(r#"base_iri = "http://example.org/""#).unwrap();
    let mut rudof = JsRudof::new(Some(config));
    rudof.read_data(DATA, None, None, None, None).unwrap();
    rudof.read_shex(SHEX_SCHEMA, None, None, None).unwrap();
    rudof.read_shapemap("<alice>@:Person", None, None, None).unwrap();
    let report = rudof.validate_shex().unwrap();
    assert_eq!(get(&report, "conforms"), JsValue::TRUE);

    rudof.read_shacl(Some(SHACL_SHAPES.into()), None, None, None).unwrap();
    let report = rudof.validate_shacl(None).unwrap();
    assert_eq!(array(&get(&report, "entries")).length(), 1);

    rudof
        .read_query("PREFIX : <http://example.org/> ASK { :alice :age 23 }", None)
        .unwrap();
    let results = rudof.run_query().unwrap();
    assert_eq!(get(&results, "kind"), "ask");
    assert_eq!(get(&results, "boolean"), JsValue::TRUE);

    rudof.add_prefix("ex", "http://example.org/").unwrap();
    let prefixes = array(&rudof.prefixes().unwrap());
    assert!(prefixes.iter().any(|pair| array(&pair).get(0) == "ex"));
    assert!(!rudof.get_version().is_empty());
}
