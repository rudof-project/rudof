use super::{
    annotation::Annotation,
    dependency_graph::{DependencyGraph, PosNeg},
    sem_act::SemAct,
};
use crate::{Expr, Pred, ShapeLabelIdx, ir::schema_ir::SchemaIR};
use itertools::Itertools;
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Shape {
    closed: bool,
    extra: Vec<Pred>,
    expr: Expr,
    sem_acts: Vec<SemAct>,
    annotations: Vec<Annotation>,
    preds: Vec<Pred>,
    extends: Vec<ShapeLabelIdx>,
    // references: HashMap<Pred, Vec<ShapeLabelIdx>>,
}

impl Shape {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        closed: bool,
        extra: Vec<Pred>,
        expr: Expr,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
        preds: Vec<Pred>,
        extends: Vec<ShapeLabelIdx>,
        // references: HashMap<Pred, Vec<ShapeLabelIdx>>,
    ) -> Self {
        Shape {
            closed,
            extra,
            expr,
            sem_acts,
            annotations,
            preds,
            extends,
        }
    }

    pub fn extends(&self) -> &Vec<ShapeLabelIdx> {
        &self.extends
    }

    pub fn preds(&self) -> Vec<Pred> {
        self.preds.clone()
    }

    pub fn references(&self) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        self.get_value_expr_references()
    }

    /// Regular Bag expression that corresponds to the triple expression of the shape
    /// Replace by expr which easier to remember?
    pub fn triple_expr(&self) -> &Expr {
        &self.expr
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Obtain the RBE tables that are affected by the current shape
    /// If there are no extends, it will only contain a value pointing None to the rbe_table
    /// It there are extends, each value of the HashMap will be a pair from the label of the extended shape to its rbe table
    pub fn get_triple_exprs(&self, schema: &SchemaIR) -> HashMap<Option<ShapeLabelIdx>, Vec<Expr>> {
        let main_triple_expr = self.expr.clone();
        let mut result = HashMap::new();
        result.insert(None, vec![main_triple_expr]);
        for e in &self.extends {
            match result.entry(Some(*e)) {
                Entry::Vacant(v) => {
                    let info = schema.find_shape_idx(e).unwrap();
                    let exprs = info.expr().get_triple_exprs(schema);
                    v.insert(exprs);
                }
                Entry::Occupied(_o) => {
                    // Ignore and don't insert anything here for diamond shapes...
                    // o.into_mut().extend(exprs);
                }
            }
        }
        result
    }

    pub fn show_short(&self) -> String {
        let closed = if self.closed { "CLOSED" } else { "" };
        let extra = if self.extra.is_empty() {
            "".to_string()
        } else {
            format!("EXTRA [{}]", self.extra.iter().join(" "))
        };
        format!("Shape {closed}{extra}")
    }

    pub fn add_edges(&self, source: ShapeLabelIdx, graph: &mut DependencyGraph, pos_neg: PosNeg) {
        for (_component, _pred, cond) in self.expr.components() {
            match cond {
                rbe::MatchCond::Single(_single_cond) => {}
                rbe::MatchCond::And(_match_conds) => {}
                rbe::MatchCond::Ref(r) => graph.add_edge(source, r, pos_neg),
            }
        }
    }

    fn get_value_expr_references(&self) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        let mut result: HashMap<Pred, Vec<ShapeLabelIdx>> = HashMap::new();
        for (_component, pred, cond) in self.expr.components() {
            match cond {
                rbe::MatchCond::Single(_single_cond) => {}
                rbe::MatchCond::And(_match_conds) => {}
                rbe::MatchCond::Ref(r) => {
                    result.entry(pred.clone()).or_default().push(r);
                }
            }
        }
        result
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extends = if self.extends.is_empty() {
            "".to_string()
        } else {
            format!(
                "EXTENDS [{}]",
                self.extends.iter().map(|e| e.to_string()).join(" ")
            )
        };
        let closed = if self.closed { "CLOSED" } else { "" };
        let extra = if self.extra.is_empty() {
            "".to_string()
        } else {
            format!("EXTRA [{}]", self.extra.iter().join(" "))
        };
        let preds = if self.preds.is_empty() {
            "".to_string()
        } else {
            self.preds.iter().join(",")
        };
        write!(f, "Shape {extends}{closed}{extra} ")?;
        write!(f, "Preds: {preds}")?;
        write!(f, ", TripleExpr: {}", self.expr)?;
        write!(
            f,
            ", References: [{}]",
            self.references()
                .iter()
                .map(|(p, ls)| format!("{}->{}", p, ls.iter().join(" ")))
                .join(", ")
        )?;
        Ok(())
    }
}
