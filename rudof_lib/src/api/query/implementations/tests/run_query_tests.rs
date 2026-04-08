use crate::{
    Rudof, RudofConfig,
    api::query::implementations::load_query::load_query,
    api::query::implementations::run_query::run_query,
    api::query::implementations::serialize_query_results::serialize_query_results,
    formats::{InputSpec, ResultQueryFormat},
};
use std::str::FromStr;

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

    let rdf_data = InputSpec::from_str(rdf_data_str).unwrap();
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

    let query_input = InputSpec::from_str(
        r#"
prefix : <http://example.org/>
prefix schema: <http://schema.org/>

select ?person ?name ?status where {
  ?person schema:name ?name ;
          :status ?status .
}"#,
    )
    .unwrap();

    load_query(&mut rudof, &query_input, None).unwrap();
    run_query(&mut rudof, None).unwrap();

    let serialized = serialize_results_to_string(&rudof, Some(ResultQueryFormat::Internal));

    assert!(serialized.contains("Alice"));
    assert!(serialized.contains("Bob"));

    println!(
        "\n===== test_run_and_serialize_select_query =====\n{}\n============================================",
        serialized
    );
}
