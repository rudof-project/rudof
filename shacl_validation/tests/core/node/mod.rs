use shacl_validation::shacl_processor::ShaclValidationMode;

use crate::test;

const PATH: &str = "/tests/data-shapes/data-shapes-test-suite/tests/core/node/";

#[test]
fn and_001() {
    let path = format!("{}/{}.ttl", PATH, "and-001");
    test(path, ShaclValidationMode::Native, false);
}

#[test]
fn and_002() {}

#[test]
fn class_001() {}

#[test]
fn class_002() {}

#[test]
fn class_003() {}

#[test]
fn closed_001() {}

#[test]
fn closed_002() {}

#[test]
fn datatype_001() {}

#[test]
fn datatype_002() {}

#[test]
fn disjoint_001() {}

#[test]
fn equals_001() {}

#[test]
fn hasValue_001() {}

#[test]
fn in_001() {}

#[test]
fn languageIn_001() {}

#[test]
fn maxExclusive_001() {}

#[test]
fn maxInclusive_001() {}

#[test]
fn maxLength_001() {}

#[test]
fn minExclusive_001() {}

#[test]
fn minInclusive_001() {}

#[test]
fn minInclusive_002() {}

#[test]
fn minInclusive_003() {}

#[test]
fn minLength_001() {}

#[test]
fn node_001() {}

#[test]
fn nodeKind_001() {}

#[test]
fn not_001() {}

#[test]
fn not_002() {}

#[test]
fn or_001() {}

#[test]
fn pattern_001() {}

#[test]
fn pattern_002() {}

#[test]
fn qualified_001_data() {}

#[test]
fn qualified_001_shapes() {}

#[test]
fn qualified_001() {}

#[test]
fn xone_001() {}

#[test]
fn xone_duplicate_data() {}

#[test]
fn xone_duplicate_shapes() {}

#[test]
fn xone_duplicate() {}
