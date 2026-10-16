#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/core/path/",
        path_alternative_001 => "path-alternative-001",
        path_complex_001 => "path-complex-001",
        path_complex_002_data => "path-complex-002-data",
        path_complex_002_shapes => "path-complex-002-shapes",
        path_complex_002 => "path-complex-002",
        path_inverse_001 => "path-inverse-001",
        path_one_or_more_001 => "path-oneOrMore-001",
        path_sequence_001 => "path-sequence-001",
        path_sequence_002 => "path-sequence-002",
        path_sequence_duplicate_001 => "path-sequence-duplicate-001",
        path_strange_001 => "path-strange-001",
        path_strange_002 => "path-strange-002",
        path_unused_001_data => "path-unused-001-data",
        path_unused_001_shapes => "path-unused-001-shapes",
        path_unused_001 => "path-unused-001",
        path_zero_or_more_001 => "path-zeroOrMore-001",
        path_zero_or_one_001 => "path-zeroOrOne-001",
    }
}
