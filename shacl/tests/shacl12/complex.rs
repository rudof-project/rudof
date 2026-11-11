#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/complex/",
        personexample => "personexample",
        shacl_shacl => "shacl-shacl",
    }
}
