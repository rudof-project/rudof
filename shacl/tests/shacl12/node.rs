#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/node/",
        and_001 => "and-001",
        and_002 => "and-002",
        class_001 => "class-001",
        class_002 => "class-002",
        class_003 => "class-003",
        closed_001 => "closed-001",
        closed_002 => "closed-002",
        #[ignore = "sh:closed sh:ByTypes is not implemented"]
        closed_003 => "closed-003",
        #[ignore = "sh:closed sh:ByTypes is not implemented"]
        closed_004 => "closed-004",
        datatype_001 => "datatype-001",
        datatype_002 => "datatype-002",
        #[ignore = "a list of datatypes as the value of sh:datatype is not implemented"]
        datatype_003 => "datatype-003",
        disjoint_001 => "disjoint-001",
        equals_001 => "equals-001",
        has_value_001 => "hasValue-001",
        in_001 => "in-001",
        language_in_001 => "languageIn-001",
        max_exclusive_001 => "maxExclusive-001",
        max_inclusive_001 => "maxInclusive-001",
        max_length_001 => "maxLength-001",
        #[ignore = "sh:maxListLength is not implemented"]
        max_list_length_001 => "maxListLength-001",
        #[ignore = "sh:memberShape is not implemented"]
        member_shape_001 => "memberShape-001",
        min_exclusive_001 => "minExclusive-001",
        min_inclusive_001 => "minInclusive-001",
        min_inclusive_002 => "minInclusive-002",
        min_inclusive_003 => "minInclusive-003",
        min_length_001 => "minLength-001",
        #[ignore = "sh:minListLength is not implemented"]
        min_list_length_001 => "minListLength-001",
        node_001 => "node-001",
        node_by_expression_001 => "nodeByExpression-001",
        node_kind_001 => "nodeKind-001",
        #[ignore = "a list of node kinds as the value of sh:nodeKind is not implemented"]
        node_kind_002 => "nodeKind-002",
        not_001 => "not-001",
        not_002 => "not-002",
        or_001 => "or-001",
        pattern_001 => "pattern-001",
        pattern_002 => "pattern-002",
        qualified_001 => "qualified-001",
        #[ignore = "sh:rootClass is not implemented"]
        root_class_001 => "rootClass-001",
        #[ignore = "sh:uniqueMembers is not implemented"]
        unique_members_001 => "uniqueMembers-001",
        #[ignore = "sh:uniqueValuesFor is not implemented"]
        unique_values_for_001 => "uniqueValuesFor-001",
        #[ignore = "sh:uniqueValuesFor is not implemented"]
        unique_values_for_002 => "uniqueValuesFor-002",
        #[ignore = "sh:uniqueValuesFor is not implemented"]
        unique_values_for_003 => "uniqueValuesFor-003",
        // Conforms because the values are missing, which an unimplemented sh:uniqueValuesFor also does
        unique_values_for_004 => "uniqueValuesFor-004",
        xone_001 => "xone-001",
        xone_duplicate => "xone-duplicate",
    }
}
