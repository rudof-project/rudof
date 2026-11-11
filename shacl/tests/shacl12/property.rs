#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/property/",
        and_001 => "and-001",
        class_001 => "class-001",
        #[ignore = "a list of classes as the value of sh:class is not implemented"]
        class_002 => "class-002",
        datatype_001 => "datatype-001",
        datatype_002 => "datatype-002",
        datatype_003 => "datatype-003",
        #[ignore = "a list of datatypes as the value of sh:datatype is not implemented"]
        datatype_004 => "datatype-004",
        datatype_ill_formed => "datatype-ill-formed",
        disjoint_001 => "disjoint-001",
        #[ignore = "a property path as the value of sh:disjoint is not implemented"]
        disjoint_002 => "disjoint-002",
        equals_001 => "equals-001",
        #[ignore = "a property path as the value of sh:equals is not implemented"]
        equals_002 => "equals-002",
        has_value_001 => "hasValue-001",
        in_001 => "in-001",
        language_in_001 => "languageIn-001",
        less_than_001 => "lessThan-001",
        less_than_002 => "lessThan-002",
        #[ignore = "a property path as the value of sh:lessThan is not implemented"]
        less_than_003 => "lessThan-003",
        less_than_or_equals_001 => "lessThanOrEquals-001",
        #[ignore = "a property path as the value of sh:lessThanOrEquals is not implemented"]
        less_than_or_equals_002 => "lessThanOrEquals-002",
        max_count_001 => "maxCount-001",
        max_count_002 => "maxCount-002",
        max_exclusive_001 => "maxExclusive-001",
        max_inclusive_001 => "maxInclusive-001",
        max_length_001 => "maxLength-001",
        #[ignore = "sh:maxListLength is not implemented"]
        max_list_length_001 => "maxListLength-001",
        #[ignore = "sh:memberShape is not implemented"]
        member_shape_001 => "memberShape-001",
        min_count_001 => "minCount-001",
        min_count_002 => "minCount-002",
        min_exclusive_001 => "minExclusive-001",
        min_exclusive_002 => "minExclusive-002",
        min_length_001 => "minLength-001",
        #[ignore = "sh:minListLength is not implemented"]
        min_list_length_001 => "minListLength-001",
        node_001 => "node-001",
        node_002 => "node-002",
        node_kind_001 => "nodeKind-001",
        not_001 => "not-001",
        or_001 => "or-001",
        or_datatypes_001 => "or-datatypes-001",
        pattern_001 => "pattern-001",
        pattern_002 => "pattern-002",
        property_001 => "property-001",
        qualified_min_count_disjoint_001 => "qualifiedMinCountDisjoint-001",
        qualified_value_shape_001 => "qualifiedValueShape-001",
        qualified_value_shapes_disjoint_001 => "qualifiedValueShapesDisjoint-001",
        #[ignore = "sh:reifierShape reports the reifier shape's own violations, not one sh:ReifierShapeConstraintComponent result"]
        reifier_shape_001 => "reifierShape-001",
        #[ignore = "sh:reifierShape reports the reifier shape's own violations, not one sh:ReifierShapeConstraintComponent result"]
        reifier_shape_002 => "reifierShape-002",
        #[ignore = "sh:singleLine is not implemented"]
        single_line_001 => "singleLine-001",
        #[ignore = "sh:someValue is not implemented"]
        some_value_001 => "someValue-001",
        subset_of_001 => "subsetOf-001",
        subset_of_002 => "subsetOf-002",
        unique_lang_001 => "uniqueLang-001",
        unique_lang_002 => "uniqueLang-002",
        #[ignore = "sh:uniqueLang reports a result per language of the rdf:dirLangString values, instead of one"]
        unique_lang_003 => "uniqueLang-003",
        #[ignore = "sh:uniqueMembers is not implemented"]
        unique_members_001 => "uniqueMembers-001",
    }
}
