#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/property/",
        subset_of_001 => "subsetOf-001",
        subset_of_002 => "subsetOf-002",
    }
}
