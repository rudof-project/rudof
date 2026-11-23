use rbe::Component;
use rudof_iri::IriS;
use serde::{Deserialize, Serialize};

use crate::ShapeLabelIdx;
use crate::ir::{shape_expr::ShapeExpr, shape_label::ShapeLabel, source_idx::SourceIdx};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShapeExprInfo {
    label: Option<ShapeLabel>,
    expr: ShapeExpr,
    source: SourceIdx,
    #[serde(default)]
    location: Option<ShapeExprLocation>,
    /// The labels of the triple constraints of this shape that have one, `$label`, by the
    /// component each was compiled as.
    #[serde(default)]
    triple_expr_labels: Vec<(Component, IriS)>,
}

/// Where an anonymous shape expression is written in the schema, relative to the shape
/// expression or triple constraint that contains it. A labelled shape expression is found by its
/// label and has none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeExprLocation {
    /// The operand at `index` of the `ShapeAnd` or `ShapeOr` at `parent`.
    Operand { parent: ShapeLabelIdx, index: usize },
    /// The shape expression negated by the `ShapeNot` at `parent`.
    Negated { parent: ShapeLabelIdx },
    /// The value expression of the triple constraint compiled as `component` of the shape at
    /// `shape`.
    ValueExpr { shape: ShapeLabelIdx, component: Component },
}

impl ShapeExprInfo {
    pub fn new(label: Option<ShapeLabel>, expr: ShapeExpr, source: SourceIdx) -> Self {
        ShapeExprInfo {
            label,
            expr,
            source,
            location: None,
            triple_expr_labels: Vec::new(),
        }
    }

    /// Records that the triple constraint compiled as `component` is labelled `label`.
    pub fn add_triple_expr_label(&mut self, component: Component, label: IriS) {
        self.triple_expr_labels.push((component, label));
    }

    /// The label of the triple constraint compiled as `component`, if it has one.
    pub fn triple_expr_label(&self, component: &Component) -> Option<&IriS> {
        self.triple_expr_labels
            .iter()
            .find(|(c, _)| c == component)
            .map(|(_, label)| label)
    }

    pub fn set_expr(&mut self, expr: ShapeExpr) {
        self.expr = expr;
    }

    pub fn set_location(&mut self, location: ShapeExprLocation) {
        self.location = Some(location);
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

    pub fn location(&self) -> Option<&ShapeExprLocation> {
        self.location.as_ref()
    }
}
