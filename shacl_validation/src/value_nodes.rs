use std::collections::HashMap;

use srdf::SRDFBasic;

use crate::focus_nodes::FocusNodes;

pub struct ValueNodes<S: SRDFBasic>(HashMap<S::Term, FocusNodes<S>>);

impl<S: SRDFBasic> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, FocusNodes<S>)>) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

pub trait IterationStrategy<S: SRDFBasic> {
    type Item;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<S>,
    ) -> Box<dyn Iterator<Item = (&'a S::Term, &'a Self::Item)> + 'a>;
}

pub struct FocusNodeIteration;

impl<S: SRDFBasic> IterationStrategy<S> for FocusNodeIteration {
    type Item = FocusNodes<S>;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<S>,
    ) -> Box<dyn Iterator<Item = (&'a S::Term, &'a Self::Item)> + 'a> {
        Box::new(value_nodes.0.iter())
    }
}

pub struct ValueNodeIteration;

impl<S: SRDFBasic> IterationStrategy<S> for ValueNodeIteration {
    type Item = S::Term;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<S>,
    ) -> Box<dyn Iterator<Item = (&'a S::Term, &'a Self::Item)> + 'a> {
        Box::new(value_nodes.0.iter().flat_map(|(focus_node, value_nodes)| {
            value_nodes
                .iter()
                .map(move |value_node| (focus_node, value_node))
        }))
    }
}
