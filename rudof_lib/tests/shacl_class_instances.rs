//! SHACL instances of a class are the nodes with `rdf:type/rdfs:subClassOf*`
//! the class (<https://www.w3.org/TR/shacl/#dfn-shacl-instance>), at any
//! depth of `rdfs:subClassOf`. This holds for `sh:targetClass`, implicit class
//! targets and `sh:class`, with both validation modes.

use rudof_lib::{
    Rudof, RudofConfig,
    formats::{InputSpec, ShaclValidationMode},
};
use std::str::FromStr;

/// `:PhDStudent` is a subclass of `:Person` through two `rdfs:subClassOf`
/// steps. `:Person rdfs:subClassOf :PhDStudent` closes a cycle, which must not
/// make the lookups loop. `:bob` and `:carol` have no name.
const DATA: &str = r#"
prefix : <http://example.org/>
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
:Student    rdfs:subClassOf :Person .
:PhDStudent rdfs:subClassOf :Student .
:Person     rdfs:subClassOf :PhDStudent .
:alice a :Person ; :name "Alice" .
:bob   a :Student .
:carol a :PhDStudent .
:dave  a :Thing .
:course :student :carol, :dave, [ a :PhDStudent ; :name "Erin" ] .
"#;

const TARGET_CLASS: &str = r#"
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:property [ sh:path :name ; sh:minCount 1 ] .
"#;

const IMPLICIT_TARGET_CLASS: &str = r#"
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
:Person a rdfs:Class, sh:NodeShape ;
  sh:property [ sh:path :name ; sh:minCount 1 ] .
"#;

const CLASS: &str = r#"
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
:CourseShape a sh:NodeShape ;
  sh:targetNode :course ;
  sh:property [ sh:path :student ; sh:class :Person ] .
"#;

/// Validates `DATA` against `shapes` and returns the focus nodes and values
/// of the violations, sorted.
fn violations(shapes: &str, mode: ShaclValidationMode) -> Vec<(String, Option<String>)> {
    let config = RudofConfig::from_str("base_iri = \"http://example.org/\"").unwrap();
    let mut rudof = Rudof::new(config);
    rudof.load_data().with_data(&[InputSpec::str(DATA)]).execute().unwrap();
    rudof
        .load_shacl_shapes()
        .with_shacl_schema(&InputSpec::str(shapes))
        .execute()
        .unwrap();
    rudof
        .validate_shacl()
        .with_shacl_validation_mode(&mode)
        .execute()
        .unwrap();
    let report = rudof.shacl_validation_results().unwrap();
    let mut violations: Vec<_> = report
        .results()
        .iter()
        .map(|r| (r.focus_node().to_string(), r.value().map(|v| v.to_string())))
        .collect();
    violations.sort();
    violations
}

fn focus_nodes(shapes: &str, mode: ShaclValidationMode) -> Vec<String> {
    violations(shapes, mode).into_iter().map(|(node, _)| node).collect()
}

const MODES: [ShaclValidationMode; 2] = [ShaclValidationMode::Native, ShaclValidationMode::Sparql];

#[test]
fn target_class_includes_instances_of_subclasses_at_any_depth() {
    for mode in MODES {
        assert_eq!(
            focus_nodes(TARGET_CLASS, mode),
            ["http://example.org/bob", "http://example.org/carol"],
            "{mode:?}"
        );
    }
}

#[test]
fn implicit_target_class_includes_instances_of_subclasses_at_any_depth() {
    for mode in MODES {
        assert_eq!(
            focus_nodes(IMPLICIT_TARGET_CLASS, mode),
            ["http://example.org/bob", "http://example.org/carol"],
            "{mode:?}"
        );
    }
}

#[test]
fn class_accepts_instances_of_subclasses_at_any_depth() {
    for mode in MODES {
        // :carol and the blank node are instances of :PhDStudent; only :dave fails.
        assert_eq!(
            violations(CLASS, mode),
            [(
                "http://example.org/course".to_string(),
                Some("http://example.org/dave".to_string())
            )],
            "{mode:?}"
        );
    }
}
