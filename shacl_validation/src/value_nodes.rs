use std::collections::HashMap;

use srdf::SRDFBasic;

use crate::focus_nodes::FocusNodes;

pub struct ValueNodes<S: SRDFBasic>(HashMap<S::Term, FocusNodes<S>>);

impl<S: SRDFBasic> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, FocusNodes<S>)>) -> Self {
        Self(HashMap::from_iter(iter))
    }

    pub fn iter_value_nodes(&self) -> impl Iterator<Item = (&S::Term, &S::Term)> {
        self.0.iter().flat_map(|(focus_node, value_nodes)| {
            value_nodes
                .iter()
                .map(move |value_node| (focus_node, value_node))
        })
    }

    pub fn iter_focus_nodes(&self) -> impl Iterator<Item = (&S::Term, &FocusNodes<S>)> {
        self.0.iter()
    }
}
