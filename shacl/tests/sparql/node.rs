#[cfg(test)]
mod tests {
    use shacl::validator::ShaclValidationMode;
    use crate::common::TestSuiteError;
    use crate::test;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/sparql/node/";

    #[test]
    fn prefixes_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn sparql_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn sparql_002() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn sparql_003() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "");
        test(path, ShaclValidationMode::Native)
    }
}