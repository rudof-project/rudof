use std::collections::HashMap;

use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObjectRef;

use crate::focus_nodes::FocusNodes;

pub struct ValueNodes<R: Rdf>(HashMap<TObjectRef<R>, FocusNodes<R>>);

impl<R: Rdf> ValueNodes<R> {
    pub fn new(iter: impl Iterator<Item = (TObjectRef<R>, FocusNodes<R>)>) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

pub trait IterationStrategy<R: Rdf> {
    type Item;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<R>,
    ) -> Box<dyn Iterator<Item = (&'a TObjectRef<R>, &'a Self::Item)> + 'a>;
}

pub struct FocusNodeIteration;

impl<R: Rdf> IterationStrategy<R> for FocusNodeIteration {
    type Item = FocusNodes<R>;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<R>,
    ) -> Box<dyn Iterator<Item = (&'a TObjectRef<R>, &'a Self::Item)> + 'a> {
        Box::new(value_nodes.0.iter())
    }
}

pub struct ValueNodeIteration;

impl<R: Rdf> IterationStrategy<R> for ValueNodeIteration {
    type Item = TObjectRef<R>;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<R>,
    ) -> Box<dyn Iterator<Item = (&'a TObjectRef<R>, &'a Self::Item)> + 'a> {
        Box::new(value_nodes.0.iter().flat_map(|(focus_node, value_nodes)| {
            value_nodes
                .iter()
                .map(move |value_node| (focus_node, value_node))
        }))
    }
}
