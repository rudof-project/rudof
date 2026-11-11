#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/validation-reports/",
        conformance_disallows_001 => "conformance-disallows-001",
        conformance_disallows_002 => "conformance-disallows-002",
        shared => "shared",
    }
}
