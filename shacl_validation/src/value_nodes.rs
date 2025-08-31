use std::collections::HashMap;

use srdf::Rdf;

use crate::focus_nodes::FocusNodes;

pub struct ValueNodes<S: Rdf> {
    map: HashMap<S::Term, FocusNodes<S>>,
}

impl<S: Rdf> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, FocusNodes<S>)>) -> Self {
        Self {
            map: HashMap::from_iter(iter),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&S::Term, &FocusNodes<S>)> {
        self.map.iter()
    }
}
