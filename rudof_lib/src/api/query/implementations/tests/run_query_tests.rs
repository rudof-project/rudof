use crate::{
    Rudof, RudofConfig,
    api::query::implementations::load_sparql_query::load_sparql_query,
    api::query::implementations::run_query::run_query,
    api::query::implementations::serialize_query_results::serialize_query_results,
    formats::{InputSpec, ResultQueryFormat},
    types::{Bindings, Object, QueryResult},
};
use rudof_iri::IriS;

/// Helper: load RDF data into Rudof instance
fn setup_test_rudof_with_data() -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf_data_str = r#"
prefix : <http://example.org/>
prefix schema: <http://schema.org/>

:a schema:name  "Alice" ;
   :status      :Active ;
   schema:knows :a, :b  .

:b schema:name  "Bob"    ;
   :status      :Waiting ;
   schema:knows :c       .

:c schema:name  "Carol"  .

:d schema:name  23      .  # Should fail

:e schema:name  "Emily" ;  # Should fail
   schema:knows :d      .
    "#;

    let rdf_data = InputSpec::str(rdf_data_str);
    rudof
        .load_data()
        .with_data(&[rdf_data])
        .execute()
        .expect("Failed to load test RDF data");

    rudof
}

/// Helper: serialize query results to string
fn serialize_results_to_string(rudof: &Rudof, format: Option<ResultQueryFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_query_results(rudof, format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_run_and_serialize_select_query() {
    let mut rudof = setup_test_rudof_with_data();

    let query_input = InputSpec::str(
        r#"
prefix : <http://example.org/>
prefix schema: <http://schema.org/>

select ?person ?name ?status where {
  ?person schema:name ?name ;
          :status ?status .
}"#,
    );

    load_sparql_query(&mut rudof, &query_input, None).unwrap();
    run_query(&mut rudof, None).unwrap();

    let serialized = serialize_results_to_string(&rudof, Some(ResultQueryFormat::Internal));

    assert!(serialized.contains("Alice"));
    assert!(serialized.contains("Bob"));

    println!(
        "\n===== test_run_and_serialize_select_query =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_select_bindings_are_qualified_as_objects() {
    let mut rudof = setup_test_rudof_with_data();

    let query_input = InputSpec::str(
        r#"
prefix schema: <http://schema.org/>

select ?person ?name where {
  ?person schema:name ?name .
}"#,
    );

    load_sparql_query(&mut rudof, &query_input, None).unwrap();
    run_query(&mut rudof, None).unwrap();

    let Some(QueryResult::Select(solutions)) = rudof.query_results() else {
        panic!("a SELECT query should leave SELECT solutions behind");
    };
    let bindings = Bindings::try_from(solutions).unwrap();

    assert_eq!(bindings.variables(), ["person", "name"]);
    assert_eq!(bindings.count(), 5);

    let alice = bindings
        .rows()
        .iter()
        .find(|row| row.get("person") == Some(&Object::iri(IriS::new_unchecked("http://example.org/a"))))
        .expect("the data binds ?person to :a");
    assert_eq!(
        alice.get("name"),
        Some(&Object::str("Alice")),
        "a plain literal keeps its lexical form"
    );
}
