#[cfg(test)]
mod tests {
    use crate::common::TestSuiteError;
    use crate::test;
    use shacl::validator::ShaclValidationMode;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/complex/";

    #[test]
    fn personexample() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "personexample");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn shacl_shacl_data_shapes() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shacl-shacl-data-shapes");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn shacl_shacl() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "shacl-shacl");
        test(path, ShaclValidationMode::Native)
    }
}
