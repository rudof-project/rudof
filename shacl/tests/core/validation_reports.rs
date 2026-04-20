
#[cfg(test)]
mod tests {
    use shacl::validator::ShaclValidationMode;
    use crate::common::TestSuiteError;
    use crate::test;

    const PATH: &'static str = "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/";

    #[test]
    #[ignore]
    fn shared() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shared");
        test(path, ShaclValidationMode::Native)
    }

}