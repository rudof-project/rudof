// use shacl_validation::Subsetting;

#[cfg(test)]
mod tests {

    use crate::TestSuiteError;
    use crate::test;
    use shacl_validation::shacl_processor::ShaclValidationMode;
    use tracing_test::traced_test;

    const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/misc/";

    #[traced_test]
    #[test]
    fn deactivated_001() -> Result<(), Box<TestSuiteError>> {
        println!("Running deactivated_001 test");

        let path = format!("{}/{}.ttl", PATH, "deactivated-001");
        // test(path, ShaclValidationMode::Native, Subsetting::None)
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn deactivated_002() -> Result<(), Box<TestSuiteError>> {
        let path = format!("{}/{}.ttl", PATH, "deactivated-002");
        // test(path, ShaclValidationMode::Native, Subsetting::None)
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn message_001() -> Result<(), Box<TestSuiteError>> {
        let path = format!("{}/{}.ttl", PATH, "message-001");
        // test(path, ShaclValidationMode::Native, Subsetting::None)
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn severity_001() -> Result<(), Box<TestSuiteError>> {
        let path = format!("{}/{}.ttl", PATH, "severity-001");
        // test(path, ShaclValidationMode::Native, Subsetting::None)
        test(path, ShaclValidationMode::Native)
    }

    #[test]
    fn severity_002() -> Result<(), Box<TestSuiteError>> {
        let path = format!("{}/{}.ttl", PATH, "severity-002");
        // test(path, ShaclValidationMode::Native, Subsetting::None)
        test(path, ShaclValidationMode::Native)
    }
}
