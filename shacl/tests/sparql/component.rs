#[cfg(test)]
mod tests {
    use crate::common::TestSuiteError;
    use crate::test;
    use shacl::validator::ShaclValidationMode;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/sparql/component/";

    #[test]
    #[ignore]
    fn node_validator_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "nodeValidator-001");
        test(path.clone(), ShaclValidationMode::Native)?;
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    #[ignore]
    fn optional_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "optional-001");
        test(path.clone(), ShaclValidationMode::Native)?;
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    #[ignore]
    fn property_validator_select_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "propertyValidator-select-001");
        test(path.clone(), ShaclValidationMode::Native)?;
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    #[ignore]
    fn validator_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "validator-001");
        test(path.clone(), ShaclValidationMode::Native)?;
        test(path, ShaclValidationMode::Sparql)
    }
}
