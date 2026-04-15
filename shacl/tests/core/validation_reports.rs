
#[cfg(test)]
mod tests {
    use shacl::validation::ShaclValidationMode;
    use crate::common::TestSuiteError;
    use crate::test;

    const PATH: &'static str = "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/";

    #[test]
    fn shared_data() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shared-data");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn shared_shapes() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shared-shapes");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn shared() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shared");
        test(path, ShaclValidationMode::Native)
    }

}