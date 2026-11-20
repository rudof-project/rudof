#[cfg(test)]
mod tests {
    suite_tests! {
        shacl_testsuite::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/",
        shared => "shared",
    }
}
