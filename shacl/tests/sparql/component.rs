#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/sparql/component/",
        #[ignore]
        node_validator_001 => "nodeValidator-001",
        #[ignore]
        optional_001 => "optional-001",
        #[ignore]
        property_validator_select_001 => "propertyValidator-select-001",
        #[ignore]
        validator_001 => "validator-001",
    }
}
