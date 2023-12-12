use iri_s::IriS;
use rbe::RbeTable;

use crate::{Pred, Node, ShapeLabelIdx};

use super::{sem_act::SemAct, annotation::Annotation};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Shape {
    closed: bool,
    extra: Vec<IriS>,
    rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>,
    sem_acts: Vec<SemAct>,
    annotations: Vec<Annotation>,
    preds: Vec<IriS>
}

impl Shape {

    pub fn new(
        closed: bool, 
        extra: Vec<IriS>, 
        rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>, 
        sem_acts: Vec<SemAct>, 
        annotations: Vec<Annotation>, 
        preds: Vec<IriS>) -> Self {
       Shape {
         closed,
         extra,
         rbe_table,
         sem_acts,
         annotations,
         preds
       }
    }

    pub fn preds(&self) -> Vec<IriS> {
        self.preds.clone()
    }

    pub fn rbe_table(&self) -> &RbeTable<Pred, Node, ShapeLabelIdx> {
        &self.rbe_table
    }
}