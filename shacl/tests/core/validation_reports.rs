#[cfg(test)]
mod tests {
    use crate::common::TestSuiteError;
    use crate::test;
    use shacl::validator::ShaclValidationMode;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/";

    #[test]
    fn shared() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shared");
        test(path, ShaclValidationMode::Native)
    }
}
