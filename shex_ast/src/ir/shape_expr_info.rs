use crate::ir::{shape_expr::ShapeExpr, shape_label::ShapeLabel, source_idx::SourceIdx};

#[derive(Debug, Default, Clone)]
pub struct ShapeExprInfo {
    label: Option<ShapeLabel>,
    expr: ShapeExpr,
    source: SourceIdx,
}

impl ShapeExprInfo {
    pub fn new(label: Option<ShapeLabel>, expr: ShapeExpr, source: SourceIdx) -> Self {
        ShapeExprInfo {
            label,
            expr,
            source,
        }
    }

    pub fn set_expr(&mut self, expr: ShapeExpr) {
        self.expr = expr;
    }

    pub fn label(&self) -> Option<&ShapeLabel> {
        self.label.as_ref()
    }

    pub fn expr(&self) -> &ShapeExpr {
        &self.expr
    }

    pub fn source_idx(&self) -> &SourceIdx {
        &self.source
    }
}
