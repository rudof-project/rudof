use rudof_rdf::rdf_core::{Rdf, term::Object};

use crate::{focus_nodes::FocusNodes, value_nodes::ValueNodes};

/// Abstraction over the possible itaration strategies when validating
pub trait IterationStrategy<S: Rdf> {
    type Item;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<S>,
    ) -> Box<dyn Iterator<Item = (&'a S::Term, &'a Self::Item)> + 'a>;

    fn to_value(&self, item: &Self::Item) -> Option<S::Term>;

    fn to_object(&self, item: &Self::Item) -> Option<Object> {
        match self.to_value(item) {
            None => None,
            Some(value) => S::term_as_object(&value).ok(),
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
        Box::new(value_nodes.iter())
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
        Box::new(
            value_nodes.iter().flat_map(|(focus_node, value_nodes)| {
                value_nodes.iter().map(move |value_node| (focus_node, value_node))
            }),
        )
    }

    fn to_value(&self, item: &Self::Item) -> Option<<S as Rdf>::Term> {
        Some(item.clone())
    }
}
