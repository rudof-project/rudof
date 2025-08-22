use shacl_validation::shacl_processor::ShaclValidationMode;
// use shacl_validation::Subsetting;

use crate::TestSuiteError;
use crate::test;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/path/";

#[test]
fn path_alternative_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-alternative-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_complex_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-complex-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_complex_002_data() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-complex-002-data");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_complex_002_shapes() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-complex-002-shapes");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_complex_002() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-complex-002");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_inverse_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-inverse-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_one_or_more_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-oneOrMore-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_sequence_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-sequence-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_sequence_002() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-sequence-002");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_sequence_duplicate_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-sequence-duplicate-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_strange_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-strange-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_strange_002() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-strange-002");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_unused_001_data() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-unused-001-data");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_unused_001_shapes() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-unused-001-shapes");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_unused_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-unused-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_zero_or_more_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-zeroOrMore-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}

#[test]
fn path_zero_or_one_001() -> Result<(), TestSuiteError> {
    let path = format!("{}/{}.ttl", PATH, "path-zeroOrOne-001");
    // test(path, ShaclValidationMode::Native, Subsetting::None)
    test(path, ShaclValidationMode::Native)
}
