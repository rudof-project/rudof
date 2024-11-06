use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::Subsetting;

use crate::test;
use crate::TestSuite;

const PATH: &str = "tests/data-shapes/data-shapes-test-suite/tests/core/property/";

#[test]
fn and_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "and-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn class_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "class-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn datatype_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn datatype_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn datatype_003() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-003");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn datatype_ill_formed_data() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-ill-formed-data");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn datatype_ill_formed_shapes() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-ill-formed-shapes");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn datatype_ill_formed() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "datatype-ill-formed");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn disjoint_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "disjoint-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn equals_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "equals-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn has_value_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "hasValue-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn in_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "in-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn language_in_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "languageIn-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn less_than_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "lessThan-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn less_than_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "lessThan-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn less_than_or_equals_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "lessThanOrEquals-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn max_count_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxCount-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn max_count_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxCount-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn max_exclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxExclusive-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn max_inclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxInclusive-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn max_length_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "maxLength-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn min_count_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minCount-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn min_count_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minCount-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn min_exclusive_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minExclusive-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn min_exclusive_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minExclusive-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn min_length_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "minLength-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn node_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "node-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn node_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "node-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn node_kind_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "nodeKind-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn not_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "not-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn or_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "or-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn or_datatypes_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "or-datatypes-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn pattern_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "pattern-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn pattern_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "pattern-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn property_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "property-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn qualified_min_count_disjoint_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "qualifiedMinCountDisjoint-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn qualified_value_shape_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "qualifiedValueShape-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn qualified_value_shapes_disjoint_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "qualifiedValueShapesDisjoint-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn unique_lang_001() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "uniqueLang-001");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn unique_lang_002() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "uniqueLang-002");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn unique_lang_002_data() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "uniqueLang-002-data");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}

#[test]
fn unique_lang_002_shapes() -> Result<(), TestSuite> {
    let path = format!("{}/{}.ttl", PATH, "uniqueLang-002-shapes");
    test(path, ShaclValidationMode::Native, Subsetting::None)
}
