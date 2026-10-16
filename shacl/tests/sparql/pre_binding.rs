#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/sparql/pre-binding/",
        #[ignore]
        pre_binding_001 => "pre-binding-001",
        #[ignore]
        pre_binding_002 => "pre-binding-002",
        #[ignore]
        pre_binding_003 => "pre-binding-003",
        #[ignore]
        pre_binding_004 => "pre-binding-004",
        #[ignore]
        pre_binding_005 => "pre-binding-005",
        #[ignore]
        pre_binding_006 => "pre-binding-006",
        #[ignore]
        pre_binding_007 => "pre-binding-007",
        #[ignore]
        shapes_graph_001 => "shapesGraph-001",
        #[ignore]
        unsupported_sparql_001 => "unsupported-sparql-001",
        #[ignore]
        unsupported_sparql_002 => "unsupported-sparql-002",
        #[ignore]
        unsupported_sparql_003 => "unsupported-sparql-003",
        #[ignore]
        unsupported_sparql_004 => "unsupported-sparql-004",
        #[ignore]
        unsupported_sparql_005 => "unsupported-sparql-005",
        #[ignore]
        unsupported_sparql_006 => "unsupported-sparql-006",
    }
}
