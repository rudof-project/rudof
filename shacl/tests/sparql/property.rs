#[cfg(all(test, feature = "sparql"))]
mod tests {
    suite_tests! {
        shacl_testsuite::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/sparql/property/",
        sparql_001 => "sparql-001",
    }
}
