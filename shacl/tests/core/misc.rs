#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/core/misc/",
        deactivated_001 => "deactivated-001",
        deactivated_002 => "deactivated-002",
        message_001 => "message-001",
        severity_001 => "severity-001",
        severity_002 => "severity-002",
    }
}
