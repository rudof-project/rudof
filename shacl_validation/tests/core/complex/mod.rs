use shacl_validation::shacl_processor::ShaclValidationMode;

use crate::test;

const PATH: &str = "/tests/data-shapes/data-shapes-test-suite/tests/core/complex/";

#[test]
fn personexample() {
    let path = format!("{}/{}.ttl", PATH, "personexample");
    test(path, ShaclValidationMode::Native, false);
}

#[test]
fn shacl_shacl_data_shapes() {}

#[test]
fn shacl_shacl() {}
