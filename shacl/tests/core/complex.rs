#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/core/complex/",
        personexample => "personexample",
        shacl_shacl_data_shapes => "shacl-shacl-data-shapes",
        shacl_shacl => "shacl-shacl",
    }
}
