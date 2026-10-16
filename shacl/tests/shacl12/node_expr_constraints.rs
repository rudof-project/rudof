#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/node-expr/constraints/",
        #[ignore = "the sh:expression constraint component is not implemented yet"]
        expression_001 => "expression-001",
        #[ignore = "the sh:nodeByExpression constraint component is not implemented yet"]
        node_by_expression_001 => "nodeByExpression-001",
    }
}
