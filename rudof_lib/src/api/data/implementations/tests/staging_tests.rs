use crate::{
    Result, Rudof, RudofConfig,
    formats::{InputSpec, ShaclFormat},
    types::Triple,
};
use std::str::FromStr;

const GRAPH: &str = r#"
        prefix : <http://example.org/>
        :alice :name "Alice" ;
               :age 23 ;
               :knows :bob .
        :bob :name "Bob" .
    "#;

/// The shapes and the instances of the same session live in one graph here, which is
/// what `load_shacl_shapes()` with no source reads as the shapes graph.
const SHAPES_AND_DATA: &str = r#"
        prefix :    <http://example.org/>
        prefix sh:  <http://www.w3.org/ns/shacl#>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :PersonShape a sh:NodeShape ;
            sh:targetClass :Person ;
            sh:property [ sh:path :name ; sh:datatype xsd:string ; sh:minCount 1 ] .

        :alice a :Person ; :name "Alice" .
    "#;

fn session() -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());
    let data = InputSpec::from_str(GRAPH).unwrap();
    rudof.load_data().with_data(&[data]).execute().unwrap();
    rudof
}

/// The triples of the loaded data, sorted so the assertions do not depend on the
/// iteration order of the graph.
fn all(rudof: &Rudof) -> Vec<String> {
    let mut triples: Vec<Triple> = rudof.triples().execute().unwrap().collect::<Result<_>>().unwrap();
    triples.sort_by_key(|triple| triple.to_string());
    triples.iter().map(Triple::to_string).collect()
}

fn count(rudof: &Rudof) -> usize {
    all(rudof).len()
}

#[test]
fn test_commit_keeps_and_rollback_drops_a_staged_add() {
    let mut rudof = session();

    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    assert_eq!(count(&rudof), 5);
    rudof.commit_staging().execute().unwrap();
    assert_eq!(count(&rudof), 5, "a committed add is kept");
    assert_eq!(rudof.staging_depth().execute(), 0);

    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":dave", ":name", "\"Dave\"").execute().unwrap();
    assert_eq!(count(&rudof), 6);
    rudof.rollback_staging().execute().unwrap();
    assert_eq!(count(&rudof), 5, "a rolled-back add is gone");
    assert!(
        rudof
            .triples()
            .with_subject(":dave")
            .execute()
            .unwrap()
            .next()
            .is_none(),
        "and it is the staged triple that is gone"
    );
    assert_eq!(rudof.staging_depth().execute(), 0);
}

#[test]
fn test_rollback_restores_a_staged_remove() {
    let mut rudof = session();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    rudof.remove_triple(":alice", ":name", "\"Alice\"").execute().unwrap();
    rudof.remove_triples(&[(":bob", ":name", "\"Bob\"")]).execute().unwrap();
    assert_eq!(count(&rudof), 2);

    rudof.rollback_staging().execute().unwrap();
    assert_eq!(all(&rudof), before, "every removed triple is back, and nothing else");
}

#[test]
fn test_rollback_undoes_what_actually_changed_and_only_that() {
    let mut rudof = session();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    // Adding a triple the graph already has is no change, so rolling back must not
    // remove it; removing one it does not have is no change either.
    rudof.add_triple(":bob", ":name", "\"Bob\"").execute().unwrap();
    rudof.remove_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    rudof.rollback_staging().execute().unwrap();

    assert_eq!(all(&rudof), before);
}

#[test]
fn test_rollback_of_a_batch_restores_the_graph_exactly() {
    let mut rudof = session();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    rudof
        .add_triples(&[
            (":carol", ":name", "\"Carol\""),
            (":carol", ":knows", ":alice"),
            (":bob", ":name", "\"Bob\""),
        ])
        .execute()
        .unwrap();
    rudof
        .remove_triples(&[(":alice", ":age", "23"), (":dave", ":name", "\"Dave\"")])
        .execute()
        .unwrap();
    assert_eq!(count(&rudof), 5);

    rudof.rollback_staging().execute().unwrap();
    assert_eq!(all(&rudof), before);
}

#[test]
fn test_nested_frames() {
    // An inner commit hands its changes to the outer frame, which still undoes them.
    let mut rudof = session();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    assert_eq!(rudof.begin_staging().execute().unwrap(), 2);
    rudof.add_triple(":dave", ":name", "\"Dave\"").execute().unwrap();
    rudof.commit_staging().execute().unwrap();
    assert_eq!(rudof.staging_depth().execute(), 1);
    assert_eq!(count(&rudof), 6);

    rudof.rollback_staging().execute().unwrap();
    assert_eq!(all(&rudof), before, "the outer rollback undoes the inner work too");

    // An inner rollback leaves only what the outer frame staged.
    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":dave", ":name", "\"Dave\"").execute().unwrap();
    rudof.rollback_staging().execute().unwrap();
    rudof.commit_staging().execute().unwrap();

    assert_eq!(rudof.staging_depth().execute(), 0);
    assert_eq!(count(&rudof), 5);
    assert_eq!(rudof.triples().with_subject(":carol").execute().unwrap().count(), 1);
    assert_eq!(rudof.triples().with_subject(":dave").execute().unwrap().count(), 0);
}

#[test]
fn test_staging_depth_and_closing_more_frames_than_are_open() {
    let mut rudof = session();

    assert_eq!(rudof.staging_depth().execute(), 0);
    assert_eq!(rudof.begin_staging().execute().unwrap(), 1);
    assert_eq!(rudof.begin_staging().execute().unwrap(), 2);
    assert_eq!(rudof.staging_depth().execute(), 2);
    rudof.commit_staging().execute().unwrap();
    assert_eq!(rudof.staging_depth().execute(), 1);
    rudof.rollback_staging().execute().unwrap();
    assert_eq!(rudof.staging_depth().execute(), 0);

    for err in [
        rudof.commit_staging().execute().unwrap_err(),
        rudof.rollback_staging().execute().unwrap_err(),
    ] {
        assert!(err.to_string().contains("No staging frame is open"), "{err}");
    }
}

#[test]
fn test_rollback_of_a_load_data_inside_a_frame() {
    let mut rudof = session();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    // Merging a document in cannot be undone triple by triple, so the frame falls
    // back to the snapshot it took before it — which still has to undo the staged
    // triple that came before.
    let more = InputSpec::from_str(
        r#"
        prefix : <http://example.org/>
        :dave :name "Dave" .
        "#,
    )
    .unwrap();
    rudof.load_data().with_data(&[more]).execute().unwrap();
    assert_eq!(count(&rudof), 6);

    rudof.rollback_staging().execute().unwrap();
    assert_eq!(all(&rudof), before);
}

#[test]
fn test_rollback_of_a_reset_data_inside_a_frame() {
    let mut rudof = session();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    rudof.remove_triple(":bob", ":name", "\"Bob\"").execute().unwrap();
    rudof.reset_data().execute();
    assert!(rudof.data_stats().is_none(), "the data is gone");

    rudof.rollback_staging().execute().unwrap();
    assert_eq!(all(&rudof), before, "and the frame brings all of it back");
}

#[test]
fn test_begin_staging_needs_mutable_rdf_data() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let err = rudof.begin_staging().execute().unwrap_err();
    assert!(err.to_string().contains("No RDF data loaded"), "{err}");
    assert_eq!(rudof.staging_depth().execute(), 0);
}

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
#[test]
fn test_begin_staging_is_refused_on_endpoint_data() {
    let mut rudof = Rudof::new(RudofConfig::default());
    // No request is made: the frame is refused before anything would be sent.
    rudof
        .load_data()
        .with_endpoint("http://example.org/sparql")
        .execute()
        .unwrap();

    let err = rudof.begin_staging().execute().unwrap_err();
    assert!(err.to_string().contains("cannot be modified"), "{err}");
    assert_eq!(rudof.staging_depth().execute(), 0);
}

#[test]
fn test_rollback_clears_the_results_of_the_graph_it_discards() {
    let mut rudof = session();
    let query = InputSpec::str(
        r#"
        prefix : <http://example.org/>
        SELECT ?name WHERE { ?who :name ?name }
        "#,
    );

    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    rudof.load_sparql_query(&query).execute().unwrap();
    rudof.run_query().execute().unwrap();
    assert!(rudof.query_results().is_some());
    assert!(rudof.has_sparql_query());

    rudof.rollback_staging().execute().unwrap();
    assert!(
        rudof.query_results().is_none(),
        "the answers describe a graph that no longer exists"
    );
    assert!(!rudof.has_sparql_query(), "and so does the query that was asked");

    // The same query against the restored graph answers for the restored graph: the
    // backend's SPARQL store cache is not left stale by the rollback.
    rudof.load_sparql_query(&query).execute().unwrap();
    rudof.run_query().execute().unwrap();
    let results = format!("{:?}", rudof.query_results().unwrap());
    assert!(results.contains("Alice"), "{results}");
    assert!(!results.contains("Carol"), "{results}");
}

#[test]
fn test_commit_keeps_the_results_of_the_graph_it_keeps() {
    let mut rudof = session();
    let query = InputSpec::str("SELECT ?s WHERE { ?s ?p ?o }");

    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    rudof.load_sparql_query(&query).execute().unwrap();
    rudof.run_query().execute().unwrap();

    rudof.commit_staging().execute().unwrap();
    assert!(rudof.query_results().is_some());
    assert!(rudof.has_sparql_query());
}

#[test]
fn test_a_rollback_leaves_the_loaded_shapes_alone() {
    let mut rudof = session();
    let shapes = InputSpec::str(
        r#"
        @prefix :    <http://example.org/> .
        @prefix sh:  <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        :PersonShape a sh:NodeShape ;
            sh:targetClass :Person ;
            sh:property [ sh:path :name ; sh:datatype xsd:string ; sh:minCount 1 ] .
        "#,
    );

    rudof.begin_staging().execute().unwrap();
    // A schema loaded inside the frame is session state, not staged state.
    rudof
        .load_shacl_shapes()
        .with_shacl_schema(&shapes)
        .with_shacl_schema_format(&ShaclFormat::Turtle)
        .execute()
        .unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    rudof.validate_shacl().execute().unwrap();
    assert!(rudof.shacl_validation_results().is_some());

    rudof.rollback_staging().execute().unwrap();
    assert!(rudof.shacl_shapes().is_some(), "the schema is still loaded");
    assert!(
        rudof.shacl_validation_results().is_none(),
        "only the report of the discarded graph is cleared"
    );

    // And it is still usable: validating again needs no reload.
    rudof.validate_shacl().execute().unwrap();
    assert!(rudof.shacl_validation_results().is_some());
}

#[test]
fn test_shapes_read_out_of_the_data_survive_a_rollback_of_that_data() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let data = InputSpec::from_str(SHAPES_AND_DATA).unwrap();
    rudof.load_data().with_data(&[data]).execute().unwrap();
    let before = all(&rudof);

    rudof.begin_staging().execute().unwrap();
    // A second shape, staged as data: with the shapes in the data graph, the shape
    // triples are data like any other.
    rudof
        .add_triples(&[
            (
                ":DogShape",
                "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>",
                "sh:NodeShape",
            ),
            (":DogShape", "sh:targetClass", ":Dog"),
        ])
        .execute()
        .unwrap();
    // Reading the shapes graph out of the loaded data compiles a schema of its own.
    rudof.load_shacl_shapes().execute().unwrap();
    rudof.rollback_staging().execute().unwrap();

    assert_eq!(all(&rudof), before, "the staged shape triples are gone from the data");
    assert!(
        rudof.shacl_shapes().is_some(),
        "but the compiled schema is session state and stays"
    );

    // Re-reading the shapes graph after the rollback yields the shapes of the
    // restored data, which is how a session goes back to them.
    rudof.load_shacl_shapes().execute().unwrap();
    rudof.validate_shacl().execute().unwrap();
    assert!(rudof.shacl_validation_results().is_some());
}

#[test]
fn test_the_guard_rolls_back_when_it_is_dropped() {
    let mut rudof = session();
    let before = all(&rudof);

    {
        let mut tx = rudof.staging().execute().unwrap();
        tx.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
        assert_eq!(tx.staging_depth().execute(), 1);
        // Dropped without a commit.
    }
    assert_eq!(rudof.staging_depth().execute(), 0);
    assert_eq!(all(&rudof), before);

    // Leaving the frame through an error rolls it back the same way.
    fn stage_then_fail(rudof: &mut Rudof) -> Result<()> {
        let mut tx = rudof.staging().execute()?;
        tx.add_triple(":dave", ":name", "\"Dave\"").execute()?;
        tx.add_triple("\"not a subject\"", ":name", "\"Dave\"").execute()?;
        tx.commit()
    }
    assert!(stage_then_fail(&mut rudof).is_err());
    assert_eq!(rudof.staging_depth().execute(), 0);
    assert_eq!(all(&rudof), before);
}

#[test]
fn test_the_guard_commits_and_rolls_back_on_demand() {
    let mut rudof = session();

    let mut tx = rudof.staging().execute().unwrap();
    tx.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    tx.commit().unwrap();
    assert_eq!(count(&rudof), 5);
    assert_eq!(rudof.staging_depth().execute(), 0);

    let mut tx = rudof.staging().execute().unwrap();
    tx.add_triple(":dave", ":name", "\"Dave\"").execute().unwrap();
    tx.rollback().unwrap();
    assert_eq!(count(&rudof), 5);
    assert_eq!(rudof.staging_depth().execute(), 0);
}

#[test]
fn test_the_guard_closes_the_frames_opened_inside_it() {
    let mut rudof = session();
    let before = all(&rudof);

    {
        let mut tx = rudof.staging().execute().unwrap();
        tx.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
        // Opened by hand through the deref and left open: the guard closes it too.
        tx.begin_staging().execute().unwrap();
        tx.add_triple(":dave", ":name", "\"Dave\"").execute().unwrap();
    }
    assert_eq!(rudof.staging_depth().execute(), 0);
    assert_eq!(all(&rudof), before);

    // And a frame already closed by hand leaves the guard with nothing to do.
    let mut tx = rudof.staging().execute().unwrap();
    tx.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    tx.commit_staging().execute().unwrap();
    assert_eq!(tx.staging_depth().execute(), 0);
    drop(tx);
    assert_eq!(rudof.staging_depth().execute(), 0);
    assert_eq!(count(&rudof), 5, "the committed change is not rolled back twice");
}

#[test]
fn test_reset_all_inside_a_frame_abandons_the_staging() {
    let mut rudof = session();

    rudof.begin_staging().execute().unwrap();
    rudof.begin_staging().execute().unwrap();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    rudof.reset_all().execute();

    assert_eq!(
        rudof.staging_depth().execute(),
        0,
        "there is nothing left to roll back to"
    );
    assert!(rudof.data_stats().is_none());
    let err = rudof.rollback_staging().execute().unwrap_err();
    assert!(err.to_string().contains("No staging frame is open"), "{err}");
}
