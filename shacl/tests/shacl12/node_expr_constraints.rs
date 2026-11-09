#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/node-expr/constraints/",
        expression_001 => "expression-001",
        node_by_expression_001 => "nodeByExpression-001",
    }
}
