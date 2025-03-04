use shacl_validation::engine::native::NativeEngine;

use crate::test;
use crate::TestSuiteError;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/validation-reports/";

#[test]
fn shared_data() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "shared-data");
    test::<NativeEngine>(path)
}

#[test]
fn shared_shapes() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "shared-shapes");
    test::<NativeEngine>(path)
}

#[test]
fn shared() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "shared");
    test::<NativeEngine>(path)
}
