use crate::ir::node_expr::IRNodeExpr;
use std::fmt::{Display, Formatter};

/// sh:nodeByExpression specifies the condition that each value node conforms to the node
/// shapes a node expression produces for it.
///
/// https://www.w3.org/TR/shacl12-node-expr/#NodeByExpressionConstraintComponent
#[derive(Debug, Clone)]
pub struct NodeByExpression {
    expr: IRNodeExpr,
}

impl NodeByExpression {
    pub fn new(expr: IRNodeExpr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &IRNodeExpr {
        &self.expr
    }
}

impl Display for NodeByExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeByExpression({})", self.expr())
    }
}
