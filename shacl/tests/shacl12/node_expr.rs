#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::node_expr_runner,
        "tests/data-shapes/shacl12-test-suite/tests/node-expr/shnex/",
        concat => "concat",
        constant => "constant",
        count => "count",
        distinct => "distinct",
        empty => "empty",
        exists => "exists",
        flat_map => "flatMap",
        if_expr => "if",
        instances_of => "instancesOf",
        intersection => "intersection",
        limit => "limit",
        list => "list",
        max => "max",
        min => "min",
        nodes_matching => "nodesMatching",
        offset => "offset",
        order_by => "orderBy",
        path_values => "pathValues",
        remove => "remove",
        sum => "sum",
        var => "var",

        // These three take a shape. The AST holds an inline `ASTShape`, so the
        // node expression parser would have to call the shape parser, which calls
        // the component parsers, which call the node expression parser — a cycle
        // the combinator types cannot express. The suite always references the
        // shape by IRI, so the fix is to hold a shape reference and resolve it
        // through the pending-shapes worklist, as `sh:node` does.
        #[ignore = "filter shape expressions are not parsed yet (shape reference needed)"]
        filter_shape => "filterShape",
        #[ignore = "find first expressions are not parsed yet (shape reference needed)"]
        find_first => "findFirst",
        #[ignore = "match all expressions are not parsed yet (shape reference needed)"]
        match_all => "matchAll",
    }
}
