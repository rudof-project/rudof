use crate::validator::iteration::IterationStrategy;
use crate::validator::nodes::{FocusNodes, ValueNodes};
use rudof_rdf::rdf_core::Rdf;

pub(crate) struct FocusNodeIteration;

impl<RDF: Rdf> IterationStrategy<RDF> for FocusNodeIteration {
    type Item = FocusNodes<RDF>;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<RDF>,
    ) -> Box<dyn Iterator<Item = (&'a RDF::Term, &'a Self::Item)> + 'a> {
        Box::new(value_nodes.iter())
    }

    fn to_value(&self, _: &Self::Item) -> Option<RDF::Term> {
        None
    }
}
