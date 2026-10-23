use prefixmap::PrefixMap;
use serde::Serialize;
use std::fmt::Display;

use crate::Pred;

#[derive(Debug, Clone, Serialize)]
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

    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        let preds_str = self
            .values
            .iter()
            .map(|pred| pred.show_qualified(prefixmap))
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}]", preds_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pred> {
        self.values.iter()
    }
}

impl Display for Preds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let preds_str = self
            .values
            .iter()
            .map(|pred| pred.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{preds_str}]")
    }
}
