use crate::ir::node_expr::IRNodeExpr;
use rudof_rdf::rdf_core::term::Object;
use std::fmt::{Display, Formatter};

/// The scope variable each value node is bound to while the expression is evaluated.
pub const EXPR_VALUE_VAR: &str = "value";

/// sh:expression specifies the condition that the node expression has `true` as its only
/// output node for each value node.
///
/// https://www.w3.org/TR/shacl12-node-expr/#ExpressionConstraintComponent
#[derive(Debug, Clone)]
pub struct Expression {
    expr: IRNodeExpr,
    source: Object,
}

impl Expression {
    pub fn new(expr: IRNodeExpr, source: Object) -> Self {
        Self { expr, source }
    }

    pub fn expr(&self) -> &IRNodeExpr {
        &self.expr
    }

    /// The node the expression is written on, reported as `sh:sourceConstraint`
    pub fn source(&self) -> &Object {
        &self.source
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression({})", self.expr())
    }
}
