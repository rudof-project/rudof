use crate::focus_nodes::FocusNodes;
use srdf::Rdf;
use std::collections::HashMap;

pub struct ValueNodes<S: Rdf>(HashMap<S::Term, FocusNodes<S>>);

impl<S: Rdf> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, FocusNodes<S>)>) -> Self {
        Self(iter.collect())
    }

    fn iter<'a, I: IterationStrategy>(
        &'a self,
    ) -> impl Iterator<Item = (&'a S::Term, &'a I::Item)> + 'a {
        I::iterate(self)
    }
}

pub trait IterationStrategy {
    type Item;

    fn iterate<'a, R: Rdf>(
        value_nodes: &'a ValueNodes<R>,
    ) -> impl Iterator<Item = (&'a R::Term, &'a Self::Item)> + 'a;
}

pub struct FocusNodeIteration;

impl IterationStrategy for FocusNodeIteration {
    type Item = FocusNodes<R>;

    fn iterate<'a, R: Rdf>(
        value_nodes: &'a ValueNodes<R>,
    ) -> impl Iterator<Item = (&'a R::Term, &'a Self::Item)> + 'a {
        value_nodes.0.iter()
    }
}

pub struct ValueNodeIteration;

impl IterationStrategy for ValueNodeIteration {
    type Item = R::Term;

    fn iterate<'a, R: Rdf>(
        value_nodes: &'a ValueNodes<R>,
    ) -> impl Iterator<Item = (&'a R::Term, &'a Self::Item)> + 'a {
        value_nodes.0.iter().flat_map(|(focus_node, focus_nodes)| {
            focus_nodes
                .iter()
                .map(move |value_node| (focus_node, value_node))
        })
    }
}
