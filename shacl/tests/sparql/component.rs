#[cfg(test)]
mod tests {
    use shacl::validator::ShaclValidationMode;
    use crate::common::TestSuiteError;
    use crate::test;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/sparql/component/";

    #[test]
    fn node_validator_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "nodeValidator-001");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn optional_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "optional-001");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn property_validator_select_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "propertyValidator-select-001");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn validator_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "validator-001");
        test(path, ShaclValidationMode::Native)
    }
}