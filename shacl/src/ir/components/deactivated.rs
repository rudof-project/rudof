use std::fmt::{Display, Formatter};
use crate::ir::node_expr::IRNodeExpr;

#[derive(Debug, Clone)]
pub struct Deactivated {
    is_deactivated: IRNodeExpr,
}

impl Deactivated {
    pub fn new(is_deactivated: IRNodeExpr) -> Self {
        Self { is_deactivated }
    }

    pub fn is_deactivated(&self) -> &IRNodeExpr {
        &self.is_deactivated
    }
}

impl Display for Deactivated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deactivated: {}", self.is_deactivated)
    }
}
