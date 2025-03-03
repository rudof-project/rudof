use shacl_validation::engine::native::NativeEngine;

use crate::test;
use crate::TestSuiteError;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/targets/";

#[test]
fn multiple_targets_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "multipleTargets-001");
    test::<NativeEngine>(path)
}

#[test]
fn target_class_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "targetClass-001");
    test::<NativeEngine>(path)
}

#[test]
fn target_class_implicit_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "targetClassImplicit-001");
    test::<NativeEngine>(path)
}

#[test]
fn target_node_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "targetNode-001");
    test::<NativeEngine>(path)
}

#[test]
fn target_objects_of_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "targetObjectsOf-001");
    test::<NativeEngine>(path)
}

#[test]
fn target_subjects_of_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "targetSubjectsOf-001");
    test::<NativeEngine>(path)
}

#[test]
fn target_subjects_of_002() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "targetSubjectsOf-002");
    test::<NativeEngine>(path)
}
