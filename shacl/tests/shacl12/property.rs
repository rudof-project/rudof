#[cfg(test)]
mod tests {
    use crate::common::TestSuiteError;
    use crate::test;
    use shacl::validator::ShaclValidationMode;

    const PATH: &str = "tests/data-shapes/shacl12-test-suite/tests/core/property/";

    #[test]
    fn subset_of_001() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "subsetOf-001");
        test(path.clone(), ShaclValidationMode::Native)?;
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    fn subset_of_002() -> Result<(), TestSuiteError> {
        let path = format!("{}/{}.ttl", PATH, "subsetOf-002");
        test(path.clone(), ShaclValidationMode::Native)?;
        test(path, ShaclValidationMode::Sparql)
    }
}
