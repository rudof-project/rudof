//! Tests of the rest of the API mirrored from the Python bindings: node
//! inspection, materialization, ShEx checks and resolvers, schema conversion
//! and comparison, DCTAP, rdf-config, service descriptions and property graph
//! schemas. They run natively and on `wasm`.

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use rudof_wasm::Session;

const DATA: &str = r#"
prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 ; :knows :bob .
:bob   :name "Bob"   ; :age "unknown" .
"#;

const SHEX_SCHEMA: &str = r#"
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer ; :knows @:Person * }
"#;

fn session_with_data() -> Session {
    let mut rudof = Session::new(None);
    rudof.read_data(DATA, None, None, None, None).unwrap();
    rudof
}

// Node inspection

#[test]
fn node_info() {
    let mut rudof = session_with_data();
    let info = rudof.node_info(":alice", None, None, Some(false), None).unwrap();
    assert!(info.contains("Alice"), "{info}");
    let info = rudof
        .node_info(":bob", None, Some("incoming"), Some(false), None)
        .unwrap();
    assert!(info.contains("alice"), "{info}");
}

#[test]
fn node_neighborhood() {
    let rudof = session_with_data();
    let neighborhood = rudof.node_neighborhood(":alice", None, None, None, None, None).unwrap();
    assert!(!neighborhood.truncated);
    assert_eq!(neighborhood.arcs.len(), 3, "{neighborhood:?}");
    let knows = neighborhood
        .arcs
        .iter()
        .find(|arc| arc.predicate == "http://example.org/knows")
        .unwrap();
    assert_eq!(knows.direction, "outgoing");
    assert_eq!(knows.depth, 1);
    assert_eq!(knows.neighbor, "http://example.org/bob");

    let limited = rudof
        .node_neighborhood(":alice", None, None, None, None, Some(1))
        .unwrap();
    assert_eq!(limited.arcs.len(), 1);
    assert!(limited.truncated);

    // Predicates are written as in the command line: prefixed names or <IRI>s.
    let names = [":name".to_string()];
    let only_names = rudof
        .node_neighborhood(":alice", Some(&names), None, None, None, None)
        .unwrap();
    assert_eq!(only_names.arcs.len(), 1, "{only_names:?}");
}

// Materialization with the ShEx Map extension

#[test]
fn materialize_after_validation() {
    let schema = r#"
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix Map: <http://shex.io/extensions/Map/>
:Person { :name xsd:string %Map:{ <http://example.org/vars/name> %} }
"#;
    let mut rudof = session_with_data();
    rudof.read_shex(schema, None, None, None).unwrap();
    rudof.read_shapemap(":alice@:Person", None, None, None).unwrap();
    assert!(rudof.validate_shex().unwrap().conforms);
    let graph = rudof.materialize(Some("ntriples"), None).unwrap();
    assert!(graph.contains("\"Alice\""), "{graph}");
    assert!(graph.contains("<http://example.org/name>"), "{graph}");
}

// ShEx checks and external resolvers

#[test]
fn check_shex() {
    let rudof = Session::new(None);
    let check = rudof.check_shex(SHEX_SCHEMA, None, None).unwrap();
    assert!(check.valid, "{check:?}");
    // A malformed schema is reported as invalid or as a ShEx error.
    match rudof.check_shex("prefix : <http://example.org/> :S { :p @:Missing }", None, None) {
        Ok(check) => assert!(!check.valid, "{check:?}"),
        Err(e) => assert_eq!(e.name(), "ShExError", "{e}"),
    }
}

#[test]
fn external_resolvers() {
    let resolvers = Session::list_external_resolvers();
    assert!(!resolvers.is_empty());
    assert!(
        resolvers
            .iter()
            .all(|r| !r.name.is_empty() && !r.spec_syntax.is_empty())
    );
    let mut rudof = Session::new(None);
    assert!(rudof.add_external_resolver("no-such-resolver:foo").is_err());
    rudof.clear_external_resolvers();
}

// Schema conversion and comparison

#[cfg(feature = "conversion")]
#[test]
fn convert_schemas() {
    let mut rudof = Session::new(None);
    let uml = rudof
        .convert_schemas(SHEX_SCHEMA, "shex", "uml", "shexc", "uml", None, None, None)
        .unwrap();
    assert!(uml.contains("@startuml"), "{uml}");

    let shacl_shapes = r#"
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:PersonShape a sh:NodeShape ;
  sh:property [ sh:path :name ; sh:datatype xsd:string ] .
"#;
    let shex = rudof
        .convert_schemas(shacl_shapes, "shacl", "shex", "turtle", "shexc", None, None, None)
        .unwrap();
    assert!(shex.contains("PersonShape"), "{shex}");

    let sparql = rudof
        .convert_schemas(SHEX_SCHEMA, "shex", "sparql", "shexc", "internal", None, None, None)
        .unwrap();
    assert!(sparql.to_uppercase().contains("SELECT"), "{sparql}");

    let err = rudof
        .convert_schemas(SHEX_SCHEMA, "shex", "nothing", "shexc", "turtle", None, None, None)
        .unwrap_err();
    assert_eq!(err.name(), "RangeError", "{err}");
}

#[cfg(feature = "comparison")]
#[test]
fn compare_schemas() {
    let other = r#"
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :email xsd:string }
"#;
    let mut rudof = Session::new(None);
    let comparison = rudof
        .compare_schemas(
            SHEX_SCHEMA,
            other,
            "shex",
            "shex",
            "shexc",
            "shexc",
            None,
            None,
            Some("http://example.org/Person"),
            Some("http://example.org/Person"),
            None,
        )
        .unwrap();
    assert!(comparison.contains("email"), "{comparison}");
}

// DCTAP, rdf-config and service descriptions

#[cfg(feature = "dctap")]
#[test]
fn dctap() {
    let tap = "shapeId,propertyId,mandatory,valueDatatype\nPerson,name,true,xsd:string\n";
    let mut rudof = Session::new(None);
    rudof.read_dctap(tap, Some("csv")).unwrap();
    let serialized = rudof.serialize_dctap(None).unwrap();
    assert!(serialized.contains("name"), "{serialized}");
    #[cfg(feature = "conversion")]
    {
        let shex = rudof
            .convert_schemas(tap, "dctap", "shex", "csv", "shexc", None, None, None)
            .unwrap();
        assert!(shex.contains("name"), "{shex}");
    }
    rudof.reset_dctap();
    assert!(rudof.serialize_dctap(None).is_err());
}

#[cfg(feature = "rdf-config")]
#[test]
fn rdf_config() {
    let config = r#"
- Person ex:person1:
  - a: ex:Person
  - rdfs:label:
    - name: "Alice"
"#;
    let mut rudof = Session::new(None);
    rudof.read_rdf_config(config, None).unwrap();
    let serialized = rudof.serialize_rdf_config(Some("yaml")).unwrap();
    assert!(serialized.contains("Person"), "{serialized}");
    rudof.reset_rdf_config();
    assert!(rudof.serialize_rdf_config(None).is_err());
}

#[test]
fn service_description() {
    let service = r#"
@prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
<http://example.org/sparql> a sd:Service ;
    sd:endpoint <http://example.org/sparql> ;
    sd:supportedLanguage sd:SPARQL11Query .
"#;
    let mut rudof = Session::new(None);
    rudof.read_service_description(service, None, None, None).unwrap();
    let serialized = rudof.serialize_service_description(None).unwrap();
    assert!(serialized.contains("example.org/sparql"), "{serialized}");
    rudof.reset_service_description();
    assert!(rudof.serialize_service_description(None).is_err());
}

// Property graph schemas

#[cfg(feature = "pgschema")]
#[test]
fn pgschema_validation() {
    let mut rudof = Session::new(None);
    rudof
        .read_data(
            r#"
(n1 {"Student"}["name": "Alice", "age": 23])
(n2 {"Student"}["name": "Bob", "age": 12])
"#,
            Some("pg"),
            None,
            None,
            None,
        )
        .unwrap();
    rudof
        .read_pgschema(
            r#"
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
"#,
            None,
        )
        .unwrap();
    let schema = rudof.serialize_pgschema(None).unwrap();
    assert!(schema.contains("Node Types"), "{schema}");
    rudof
        .read_typemap("n1: AdultStudentType, n2: AdultStudentType")
        .unwrap();

    let report = rudof.validate_pgschema().unwrap();
    assert!(!report.conforms, "{report:?}");
    assert_eq!(report.entries.len(), 2, "{report:?}");
    assert_eq!(report.violations.len(), 1, "{report:?}");
    assert_eq!(report.violations[0].node_id, "n2");
    assert_eq!(report.violations[0].type_name, "AdultStudentType");
    let json = rudof.serialize_pgschema_validation_results(Some("json")).unwrap();
    assert!(json.contains("n2"), "{json}");
    let csv = rudof.serialize_pgschema_validation_results(Some("csv")).unwrap();
    assert!(csv.contains("n2"), "{csv}");
    // No terminal colors on wasm (see `session_shacl_workflow`)
    if cfg!(target_family = "wasm") {
        assert!(!csv.contains('\x1b'), "{csv:?}");
    }

    rudof.reset_pgschema_validation();
    assert!(rudof.serialize_pgschema_validation_results(None).is_err());
}

// Resets

#[test]
fn reset_validation_results() {
    let mut rudof = session_with_data();
    rudof.read_shex(SHEX_SCHEMA, None, None, None).unwrap();
    rudof.read_shapemap(":alice@:Person", None, None, None).unwrap();
    rudof.validate_shex().unwrap();
    rudof.reset_validation_results();
    assert!(rudof.serialize_shex_validation_results(None, None).is_err());
    assert!(rudof.validate_shex().is_err());
}

// Operations that need the network, which is not available on wasm

#[cfg(target_family = "wasm")]
#[test]
fn network_operations_on_wasm() {
    let mut rudof = Session::new(None);
    let err = rudof.dereference("http://example.org/alice", None, None).unwrap_err();
    assert_eq!(err.name(), "DataError", "{err}");
    rudof.read_data(DATA, None, None, None, None).unwrap();
    assert!(rudof.list_endpoints().unwrap().is_empty());
}
