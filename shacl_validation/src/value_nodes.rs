use std::collections::HashMap;

use srdf::{Rdf, RDFNode};

use crate::focus_nodes::FocusNodes;

pub struct ValueNodes<S: Rdf>(HashMap<S::Term, FocusNodes<S>>);

impl<S: Rdf> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, FocusNodes<S>)>) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

pub trait IterationStrategy<S: Rdf> {
    type Item;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<S>,
    ) -> Box<dyn Iterator<Item = (&'a S::Term, &'a Self::Item)> + 'a>;

    fn to_value(&self, item: &Self::Item) -> Option<S::Term>;

    fn to_object(&self, item: &Self::Item) -> Option<RDFNode> {
        match self.to_value(item) {
            None => None,
            Some(value) => if let Ok(obj) = S::term_as_object(&value) {
                Some(obj)
            } else {
                None // TODO: Maybe handle the potential error
            }
        }
    }
}

pub struct FocusNodeIteration;

impl<S: Rdf> IterationStrategy<S> for FocusNodeIteration {
    type Item = FocusNodes<S>;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<S>,
    ) -> Box<dyn Iterator<Item = (&'a S::Term, &'a Self::Item)> + 'a> {
        Box::new(value_nodes.0.iter())
    }

    fn to_value(&self, _item: &Self::Item) -> Option<S::Term> {
        None
    }
}

pub struct ValueNodeIteration;

impl<S: Rdf> IterationStrategy<S> for ValueNodeIteration {
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

    fn to_value(&self, item: &Self::Item) -> Option<<S as Rdf>::Term> {
        Some(item.clone())
    }
}
