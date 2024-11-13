use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::Subsetting;

use crate::test;
use crate::TestSuite;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/complex/";

#[test]
fn personexample() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "personexample");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn shacl_shacl_data_shapes() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "shacl-shacl-data-shapes");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn shacl_shacl() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "shacl-shacl");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}
