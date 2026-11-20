#![cfg(not(target_family = "wasm"))]

use shacl_testsuite::manifest::ManifestWalker;
use shacl_testsuite::runner::{TestOutcome, run_outcome};
use std::path::Path;

fn fixture_suite() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/earl"))
}

#[test]
fn the_walker_finds_both_approved_entries_with_urn_test_iris() {
    let entries = ManifestWalker::new(fixture_suite())
        .validate_entries()
        .expect("the fixture suite should walk");

    let mut iris: Vec<_> = entries.iter().map(|e| e.entry.clone()).collect();
    iris.sort();

    assert_eq!(
        iris,
        vec![
            "urn:x-shacl-test:/fixture-fail".to_string(),
            "urn:x-shacl-test:/fixture-pass".to_string(),
        ]
    );
}

#[test]
fn running_each_entry_yields_the_expected_outcome() {
    let entries = ManifestWalker::new(fixture_suite())
        .validate_entries()
        .expect("the fixture suite should walk");

    for entry in &entries {
        let outcome = run_outcome(&entry.path);
        match entry.entry.as_str() {
            "urn:x-shacl-test:/fixture-pass" => {
                assert!(matches!(outcome, TestOutcome::Passed), "fixture-pass should pass");
            },
            "urn:x-shacl-test:/fixture-fail" => match outcome {
                TestOutcome::Failed(message) => assert!(!message.is_empty()),
                TestOutcome::Passed => panic!("fixture-fail should fail"),
            },
            other => panic!("unexpected entry: {other}"),
        }
    }
}
