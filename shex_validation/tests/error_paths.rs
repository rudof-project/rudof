//! A validation failure names, with a ShExPath, the shape expression or triple constraint of the
//! schema that failed, in its text and in the `path` and `paths` of its `appInfo`, and those paths
//! select it in the schema.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use serde_json::Value;
use shex_ast::ast::Schema;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::shexpath::ShExJDocument;
use shex_ast::{Node, ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};

const SCHEMA: &str = r#"
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:Person { :name xsd:string ; :age xsd:integer }
:Twice { :p xsd:integer ; :p xsd:string }
:Nested { :q { :s xsd:string } }
:Closed CLOSED { :p . }
:Either { :p xsd:string } OR { :q xsd:string }
"#;

const DATA: &str = r#"
prefix : <http://example.org/>

:old :name "Ann" ; :age "old" .
:noname :age 3 .
:once :p 1 .
:nested :q :inner .
:inner :s 3 .
:closed :p 1 ; :z 2 .
:neither :p 1 ; :q 2 .
"#;

fn compile(schema_src: &str, config: &ValidatorConfig) -> (Schema, SchemaIR) {
    let base = IriS::new_unchecked("http://example.org/");
    let ast = ShExParser::parse(schema_src, Some(base.clone()), &base).expect("parse schema");
    let mut map_state = MapState::default();
    let registry = SemanticActionsRegistry::default();
    registry.set_map_state(&mut map_state);
    let mut compiler = AST2IR::new(&ResolveMethod::default(), map_state);
    let mut compiled = SchemaIR::new(registry);
    compiler
        .compile(
            &ast,
            &base,
            &Some(base.clone()),
            &mut compiled,
            config.external_resolvers(),
        )
        .expect("compile to IR");
    (ast, compiled)
}

/// What the validation of `node@shape`, which must fail, says.
struct Failure {
    schema: Schema,
    reason: String,
    info: Vec<Value>,
}

impl Failure {
    /// The `path` of the only error.
    fn path(&self) -> &str {
        assert_eq!(self.info.len(), 1, "{:?}", self.info);
        self.info[0]["path"].as_str().expect("path")
    }

    /// The `paths` of the only error.
    fn paths(&self) -> Vec<&str> {
        assert_eq!(self.info.len(), 1, "{:?}", self.info);
        self.info[0]["paths"]
            .as_array()
            .expect("paths")
            .iter()
            .map(|path| path.as_str().expect("path"))
            .collect()
    }

    /// What `path` selects in the schema: the `type` of each item, and its `predicate` if it has
    /// one, as `type(predicate)`.
    fn selected(&self, path: &str) -> Vec<String> {
        let doc = ShExJDocument::new(&self.schema).expect("ShExJ document");
        doc.select(path)
            .unwrap_or_else(|e| panic!("{path}: {e}"))
            .iter()
            .map(|item| {
                let ty = item.value()["type"].as_str().unwrap_or("?").to_string();
                match item.value()["predicate"].as_str() {
                    Some(p) => format!("{ty}({})", p.replace("http://example.org/", ":")),
                    None => ty,
                }
            })
            .collect()
    }
}

fn failure(node: &str, shape: &str) -> Failure {
    let config = ValidatorConfig::default();
    let (schema, compiled) = compile(SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse(&format!("http://example.org/{node}"), None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked(&format!("http://example.org/{shape}")));
    let result = validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    let status = result.get_info(&node, &shape).expect("status");
    assert!(status.is_non_conformant(), "{node}@{shape} conforms");
    let info = status.app_info()["info"].as_array().expect("info entries").clone();
    Failure {
        schema,
        reason: status.reason(),
        info,
    }
}

#[test]
fn a_failed_condition_names_its_triple_constraint() {
    let failure = failure("old", "Person");
    assert_eq!(
        failure.reason,
        ":old does not conform to :Person: property :age: Datatype error: found xsd:string, \
         expected xsd:integer, lexical form \"old\" (at @:Person~:age)"
    );
    assert_eq!(failure.path(), "@:Person~:age");
    assert_eq!(failure.paths(), ["@:Person", "@:Person~:age"]);
    assert_eq!(failure.selected(failure.path()), ["TripleConstraint(:age)"]);
}

#[test]
fn a_failed_cardinality_names_its_triple_constraint() {
    let failure = failure("noname", "Person");
    assert_eq!(
        failure.reason,
        ":noname does not conform to :Person: property :name: expected exactly 1 value, found 0 \
         (at @:Person~:name)"
    );
    assert_eq!(failure.path(), "@:Person~:name");
    assert_eq!(failure.selected(failure.path()), ["TripleConstraint(:name)"]);
}

#[test]
fn constraints_on_the_same_predicate_are_told_apart() {
    let failure = failure("once", "Twice");
    assert_eq!(
        failure.reason,
        ":once does not conform to :Twice: property :p: expected exactly 1 value, found 0 \
         (2nd :p constraint, at @:Twice~:p/self::[1])"
    );
    assert_eq!(failure.path(), "@:Twice~:p/self::[1]");
    // the second constraint on :p, `:p xsd:string`
    let doc = ShExJDocument::new(&failure.schema).expect("ShExJ document");
    let selected = doc.select(failure.path()).expect("select");
    assert_eq!(selected.len(), 1);
    assert_eq!(
        selected[0].value()["valueExpr"]["datatype"],
        "http://www.w3.org/2001/XMLSchema#string"
    );
}

#[test]
fn a_nested_shape_is_named_by_where_it_is() {
    let failure = failure("nested", "Nested");
    // The anonymous shape is named by its path, the chain of references to it is one line, and
    // the datatypes of the nested error are written with the prefixes of the schema.
    assert_eq!(
        failure.reason,
        ":nested does not conform to :Nested\n\
         └── property :q value :inner does not conform to @:Nested~:q/valueExpr\n    \
         └── property :s: Datatype error: found xsd:integer, expected xsd:string, \
         lexical form \"3\"^^xsd:integer (at @:Nested~:q/valueExpr~:s)"
    );
    assert_eq!(failure.path(), "@:Nested~:q/valueExpr~:s");
    assert_eq!(
        failure.paths(),
        ["@:Nested", "@:Nested~:q/valueExpr", "@:Nested~:q/valueExpr~:s"]
    );
    assert_eq!(failure.selected("@:Nested~:q/valueExpr"), ["Shape"]);
    assert_eq!(failure.selected(failure.path()), ["TripleConstraint(:s)"]);
}

#[test]
fn a_closed_shape_names_its_shape() {
    let failure = failure("closed", "Closed");
    assert_eq!(failure.reason, "Closed shape :Closed does not allow :z (allowed: :p)");
    assert_eq!(failure.path(), "@:Closed");
    assert_eq!(failure.selected(failure.path()), ["Shape"]);
}

#[test]
fn every_path_of_a_failure_selects_one_item() {
    for (node, shape) in [
        ("old", "Person"),
        ("noname", "Person"),
        ("once", "Twice"),
        ("nested", "Nested"),
        ("closed", "Closed"),
        ("neither", "Either"),
    ] {
        let failure = failure(node, shape);
        for entry in &failure.info {
            for path in entry["paths"].as_array().expect("paths") {
                let path = path.as_str().expect("path");
                assert_eq!(failure.selected(path).len(), 1, "{node}@{shape}: {path}");
            }
        }
    }
}

#[test]
fn the_branches_of_an_or_are_named_by_where_they_are() {
    let failure = failure("neither", "Either");
    assert_eq!(
        failure.reason,
        ":neither does not conform to any branch of the OR\n\
         ├── branch 0 (@:Either/shapeExprs/*[0])\n\
         │   └── property :p: Datatype error: found xsd:integer, expected xsd:string, \
         lexical form \"1\"^^xsd:integer (at @:Either/shapeExprs/*[0]~:p)\n\
         └── branch 1 (@:Either/shapeExprs/*[1])\n    \
         └── property :q: Datatype error: found xsd:integer, expected xsd:string, \
         lexical form \"2\"^^xsd:integer (at @:Either/shapeExprs/*[1]~:q)"
    );
    assert!(
        failure.paths().contains(&"@:Either/shapeExprs/*[0]~:p"),
        "{:?}",
        failure.paths()
    );
    assert!(
        failure.paths().contains(&"@:Either/shapeExprs/*[1]~:q"),
        "{:?}",
        failure.paths()
    );
}
