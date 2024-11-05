use shacl_validation::shacl_processor::ShaclValidationMode;

use crate::test;
use crate::TestSuite;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/node/";

#[test]
fn and_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "and-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn and_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "and-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn class_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "class-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn class_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "class-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn class_003() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "class-003");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn closed_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "closed-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn closed_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "closed-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn datatype_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn datatype_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn disjoint_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "disjoint-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn equals_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "equals-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn has_value_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "hasValue-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn in_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "in-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn language_in_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "languageIn-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn max_exclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxExclusive-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn max_inclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxInclusive-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn max_length_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxLength-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn min_exclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minExclusive-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn min_inclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minInclusive-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn min_inclusive_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minInclusive-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn min_inclusive_003() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minInclusive-003");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn min_length_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minLength-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn node_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "node-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn node_kind_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "nodeKind-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn not_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "not-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn not_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "not-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn or_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "or-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn pattern_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "pattern-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn pattern_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "pattern-002");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn qualified_001_data() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "qualified-001-data");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn qualified_001_shapes() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "qualified-001-shapes");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn qualified_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "qualified-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn xone_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "xone-001");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn xone_duplicate_data() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "xone-duplicate-data");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn xone_duplicate_shapes() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "xone-duplicate-shapes");
    test(path, ShaclValidationMode::Native, false)
}

#[test]
fn xone_duplicate() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "xone-duplicate");
    test(path, ShaclValidationMode::Native, false)
}
