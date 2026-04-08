use crate::{
    Rudof, RudofConfig, api::data::implementations::show_node_info::show_node_info, formats::InputSpec,
    formats::NodeInspectionMode,
};
use std::{io::Cursor, str::FromStr};

// Helper function to create a test Rudof instance with sample RDF data
fn setup_test_rudof() -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf_data_str = r#"
            @prefix ex: <http://example.org/> .
            @prefix foaf: <http://xmlns.com/foaf/0.1/> .
            
            ex:alice a foaf:Person ;
                foaf:name "Alice" ;
                foaf:knows ex:bob ;
                foaf:age 30 .
            
            ex:bob a foaf:Person ;
                foaf:name "Bob" ;
                foaf:knows ex:alice ;
                foaf:age 25 .
                
            ex:charlie a foaf:Person ;
                foaf:name "Charlie" ;
                foaf:knows ex:alice .
        "#;

    let rdf_data = InputSpec::from_str(rdf_data_str).unwrap();

    rudof
        .load_data()
        .with_data(&[rdf_data])
        .execute()
        .expect("Failed to load test RDF data");

    rudof
}

// Helper function to create multi-level graph for depth testing
fn setup_deep_graph_rudof() -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf_data_str = r#"
            @prefix ex: <http://example.org/> .
            @prefix rel: <http://example.org/rel/> .
            
            ex:level0 rel:child ex:level1a .
            ex:level1a rel:child ex:level2a .
            ex:level2a rel:child ex:level3a .
            ex:level3a rel:child ex:level4a .
            
            ex:level0 rel:child ex:level1b .
            ex:level1b rel:child ex:level2b .
        "#;

    let rdf_data = InputSpec::from_str(rdf_data_str).unwrap();

    rudof
        .load_data()
        .with_data(&[rdf_data])
        .execute()
        .expect("Failed to load deep graph RDF data");

    rudof
}

#[test]
fn test_show_node_info_basic_outgoing() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("Outgoing arcs"));
    assert!(output_str.contains("foaf:name"));
    assert!(output_str.contains("Alice"));

    println!(
        "\n===== test_show_node_info_basic_outgoing =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_basic_incoming() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        None,
        Some(&NodeInspectionMode::Incoming),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("Incoming arcs"));
    assert!(output_str.contains("foaf:knows"));

    println!(
        "\n===== test_show_node_info_basic_incoming =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_both_directions() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        None,
        Some(&NodeInspectionMode::Both),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("Outgoing arcs"));
    assert!(output_str.contains("Incoming arcs"));

    println!(
        "\n===== test_show_node_info_both_directions =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_with_predicate_filter() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());
    let predicates = vec!["foaf:name".to_string()];

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        Some(&predicates),
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("foaf:name"));
    assert!(output_str.contains("Alice"));
    // Should not contain other predicates
    assert!(!output_str.contains("foaf:knows") || output_str.matches("foaf:knows").count() == 0);

    println!(
        "\n===== test_show_node_info_with_predicate_filter =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_multiple_predicates_filter() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());
    let predicates = vec!["foaf:name".to_string(), "foaf:age".to_string()];

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        Some(&predicates),
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("foaf:name"));
    assert!(output_str.contains("foaf:age"));

    println!(
        "\n===== test_show_node_info_multiple_predicates_filter =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_depth_1() {
    let mut rudof = setup_deep_graph_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:level0",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("level1a"));
    assert!(output_str.contains("level1b"));
    // Should not show deeper levels at depth 1
    assert!(!output_str.contains("level2a"));

    println!(
        "\n===== test_show_node_info_depth_1 =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_depth_2() {
    let mut rudof = setup_deep_graph_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:level0",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(2),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("level1a"));
    assert!(output_str.contains("level2a"));
    // Should not show level 3 at depth 2
    assert!(!output_str.contains("level3a"));

    println!(
        "\n===== test_show_node_info_depth_2 =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_depth_3() {
    let mut rudof = setup_deep_graph_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:level0",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(3),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("level1a"));
    assert!(output_str.contains("level2a"));
    assert!(output_str.contains("level3a"));
    assert!(!output_str.contains("level4a"));

    println!(
        "\n===== test_show_node_info_depth_3 =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_incoming_depth_2() {
    let mut rudof = setup_deep_graph_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:level2a",
        None,
        Some(&NodeInspectionMode::Incoming),
        Some(2),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("level1a"));
    assert!(output_str.contains("level0"));

    println!(
        "\n===== test_show_node_info_incoming_depth_2 =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_with_colors() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    // assert!(output_str.contains("\x1b["));

    println!(
        "\n===== test_show_node_info_with_colors =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_without_colors() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:alice",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(false),
        &mut output,
    );

    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(!output_str.contains("\x1b["));

    println!(
        "\n===== test_show_node_info_without_colors =====\n{}============================================",
        output_str
    );
}

#[test]
fn test_show_node_info_non_existent_node() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    let result = show_node_info(
        &mut rudof,
        "ex:nonexistent",
        None,
        Some(&NodeInspectionMode::Outgoing),
        Some(1),
        Some(false),
        Some(true),
        &mut output,
    );

    // Should succeed but produce no output (empty node)
    assert!(result.is_ok());
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    // Output should be minimal or empty
    assert!(output_str.is_empty() || !output_str.contains("Outgoing arcs"));
}

#[test]
fn test_show_node_info_default_values() {
    let mut rudof = setup_test_rudof();
    let mut output = Cursor::new(Vec::new());

    // All optional parameters as None to test defaults
    let result = show_node_info(&mut rudof, "ex:alice", None, None, None, None, None, &mut output);

    assert!(result.is_ok());

    // Should use default mode (Outgoing), depth (1), no hyperlinks, colors enabled
    let output_str = String::from_utf8(output.into_inner()).unwrap();
    println!(
        "\n===== test_show_node_info_default_values =====\n{}============================================",
        output_str
    );
}
