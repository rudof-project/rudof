#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/misc/",
        deactivated_001 => "deactivated-001",
        deactivated_002 => "deactivated-002",
        #[ignore = "sh:deactivated in a reified triple annotation is not implemented"]
        deactivated_003 => "deactivated-003",
        message_001 => "message-001",
        message_002 => "message-002",
        severity_001 => "severity-001",
        severity_002 => "severity-002",
        #[ignore = "sh:severity in a reified triple annotation is not implemented"]
        severity_003 => "severity-003",
        severity_004 => "severity-004",
        severity_005 => "severity-005",
    }
}
