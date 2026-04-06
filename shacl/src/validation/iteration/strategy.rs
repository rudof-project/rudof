use rudof_rdf::rdf_core::Rdf;
use rudof_rdf::rdf_core::term::Object;
use crate::validation::value_nodes::ValueNodes;

/// Abstraction over the possible iteration strategies when validating
pub(crate) trait IterationStrategy<RDF: Rdf> {
    type Item;

    fn iterate<'a>(
        &'a self,
        value_nodes: &'a ValueNodes<RDF>
    ) -> Box<dyn Iterator<Item = (&'a RDF::Term, &'a Self::Item)> + 'a>;

    fn to_value(&self, item: &Self::Item) -> Option<RDF::Term>;

    fn to_object(&self, item: &Self::Item) -> Option<Object> {
        match self.to_value(item) {
            None => None,
            Some(value) => RDF::term_as_object(&value).ok(),
        }
    }
}