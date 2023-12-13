use crate::Pred;

#[derive(Debug, Clone)]
pub struct Preds {
    values: Vec<Pred>,
}

impl Preds {
    pub fn new(values: Vec<Pred>) -> Preds {
        Preds { values }
    }
}
