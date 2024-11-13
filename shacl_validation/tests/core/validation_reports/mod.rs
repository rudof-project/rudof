use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::Subsetting;

use crate::test;
use crate::TestSuite;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/";

#[test]
fn shared_data() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "shared-data");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn shared_shapes() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "shared-shapes");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn shared() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "shared");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}
