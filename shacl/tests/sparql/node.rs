#[cfg(all(test, feature = "sparql"))]
mod tests {
    use crate::common::TestSuiteError;
    use crate::test;
    use shacl::validator::ShaclValidationMode;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/sparql/node/";

    #[test]
    fn prefixes_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "prefixes-001");
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    fn sparql_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "sparql-001");
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    fn sparql_002() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "sparql-002");
        test(path, ShaclValidationMode::Sparql)
    }

    #[test]
    fn sparql_003() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "sparql-003");
        test(path, ShaclValidationMode::Sparql)
    }
}
