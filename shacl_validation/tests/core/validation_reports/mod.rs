use shacl_validation::shacl_processor::ShaclValidationMode;
// use shacl_validation::Subsetting;

use crate::TestSuiteError;
use crate::test;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/";

#[test]
fn shared_data() -> Result<(), Box<TestSuiteError>> {
    let path = format!("{}/{}.ttl", PATH, "shared-data");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn shared_shapes() -> Result<(), Box<TestSuiteError>> {
    let path = format!("{}/{}.ttl", PATH, "shared-shapes");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn shared() -> Result<(), Box<TestSuiteError>> {
    let path = format!("{}/{}.ttl", PATH, "shared");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}
