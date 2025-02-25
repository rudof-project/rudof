use shacl_validation::shacl_processor::ShaclValidationMode;
// use shacl_validation::Subsetting;

use crate::test;
use crate::TestSuiteError;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/misc/";

#[test]
fn deactivated_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "deactivated-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn deactivated_002() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "deactivated-002");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn message_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "message-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn severity_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "severity-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn severity_002() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "severity-002");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}
