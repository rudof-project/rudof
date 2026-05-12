#[cfg(test)]
mod tests {
    use shacl::validator::ShaclValidationMode;
    use crate::common::TestSuiteError;
    use crate::test;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/sparql/pre-binding/";

    #[test]
    #[ignore]
    fn pre_binding_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-001");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn pre_binding_002() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-002");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn pre_binding_003() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-003");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn pre_binding_004() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-004");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn pre_binding_005() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-005");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn pre_binding_006() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-006");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn pre_binding_007() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "pre-binding-007");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn shapes_graph_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "shapesGraph-001");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn unsupported_sparql_001() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "unsupported-sparql-001");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn unsupported_sparql_002() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "unsupported-sparql-002");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn unsupported_sparql_003() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "unsupported-sparql-003");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn unsupported_sparql_004() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "unsupported-sparql-004");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn unsupported_sparql_005() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "unsupported-sparql-005");
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    #[ignore]
    fn unsupported_sparql_006() -> Result<(), TestSuiteError> {
        let path = format!("{}{}.ttl", PATH, "unsupported-sparql-006");
        test(path, ShaclValidationMode::Native)
    }
}