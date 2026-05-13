#[cfg(all(test, feature = "sparql"))]
mod tests {
    use shacl::validator::ShaclValidationMode;
    use crate::common::TestSuiteError;
    use crate::test;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/sparql/property/";

    #[test]
    fn sparql_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "sparql-001");
        test(path, ShaclValidationMode::Sparql)
    }
}