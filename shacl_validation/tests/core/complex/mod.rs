use shacl_validation::engine::native::NativeEngine;

use crate::test;
use crate::TestSuiteError;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/complex/";

#[test]
fn personexample() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "personexample");test::<NativeEngine>(path)
}

#[test]
fn shacl_shacl_data_shapes() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "shacl-shacl-data-shapes");test::<NativeEngine>(path)
}

#[test]
fn shacl_shacl() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "shacl-shacl");test::<NativeEngine>(path)
}
