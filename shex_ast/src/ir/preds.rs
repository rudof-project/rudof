use crate::Pred;

#[derive(Debug, Clone)]
pub struct Preds {
    pub values: Vec<Pred>,
}

impl Preds {
    pub fn new(values: Vec<Pred>) -> Self {
        Self { values }
    }

    pub fn preds(&self) -> &Vec<Pred> {
        &self.values
    }
}
