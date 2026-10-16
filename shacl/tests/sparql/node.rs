#[cfg(all(test, feature = "sparql"))]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/sparql/node/",
        prefixes_001 => "prefixes-001",
        sparql_001 => "sparql-001",
        sparql_002 => "sparql-002",
        sparql_003 => "sparql-003",
    }
}
