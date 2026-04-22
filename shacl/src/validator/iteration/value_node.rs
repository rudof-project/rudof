use crate::validator::iteration::IterationStrategy;
use crate::validator::nodes::ValueNodes;
use rudof_rdf::rdf_core::Rdf;

pub(crate) struct ValueNodeIteration;

impl<RDF: Rdf> IterationStrategy<RDF> for ValueNodeIteration {
    type Item = RDF::Term;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<RDF>,
    ) -> Box<dyn Iterator<Item = (&'a RDF::Term, &'a Self::Item)> + 'a> {
        Box::new(
            value_nodes.iter().flat_map(|(focus_node, value_nodes)| {
                value_nodes.iter().map(move |value_nodes| (focus_node, value_nodes))
            }),
        )
    }

    fn to_value(&self, item: &Self::Item) -> Option<RDF::Term> {
        Some(item.clone())
    }
}
