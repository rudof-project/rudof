use super::{
    annotation::Annotation,
    dependency_graph::{DependencyGraph, PosNeg},
    sem_act::SemAct,
};
use crate::{Node, Pred, ShapeLabelIdx};
use itertools::Itertools;
use rbe::RbeTable;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Shape {
    closed: bool,
    extra: Vec<Pred>,
    rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>,
    sem_acts: Vec<SemAct>,
    annotations: Vec<Annotation>,
    preds: Vec<Pred>,
    references: HashMap<Pred, Vec<ShapeLabelIdx>>,
    display: String,
}

impl Shape {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        closed: bool,
        extra: Vec<Pred>,
        rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
        preds: Vec<Pred>,
        references: HashMap<Pred, Vec<ShapeLabelIdx>>,
        display: String,
    ) -> Self {
        Shape {
            closed,
            extra,
            rbe_table,
            sem_acts,
            annotations,
            preds,
            references,
            display,
        }
    }

    pub fn preds(&self) -> Vec<Pred> {
        self.preds.clone()
    }

    pub fn references(&self) -> &HashMap<Pred, Vec<ShapeLabelIdx>> {
        &self.references
    }

    pub fn rbe_table(&self) -> &RbeTable<Pred, Node, ShapeLabelIdx> {
        &self.rbe_table
    }

    pub fn is_closed(&self) -> bool {
        self.closed
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
        for (_component, _pred, cond) in self.rbe_table.components() {
            match cond {
                rbe::MatchCond::Single(_single_cond) => {}
                rbe::MatchCond::And(_match_conds) => {}
                rbe::MatchCond::Ref(r) => graph.add_edge(source, r, pos_neg),
            }
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        write!(f, "Shape {closed}{extra} ")?;
        writeln!(f, "Preds: {}", preds)?;
        writeln!(f, "{}", self.rbe_table)?;
        Ok(())
    }
}
