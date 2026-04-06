use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use rudof_rdf::rdf_core::Rdf;
use crate::validation::focus_nodes::FocusNodes;

pub(crate) struct ValueNodes<RDF: Rdf> {
    map: HashMap<RDF::Term, FocusNodes<RDF>>
}

impl<RDF: Rdf> ValueNodes<RDF> {
    pub fn new(map: HashMap<RDF::Term, FocusNodes<RDF>>) -> Self {
        Self { map }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RDF::Term, &FocusNodes<RDF>)> {
        self.map.iter()
    }
}

impl<RDF: Rdf> FromIterator<(RDF::Term, FocusNodes<RDF>)> for ValueNodes<RDF> {
    fn from_iter<T: IntoIterator<Item=(RDF::Term, FocusNodes<RDF>)>>(iter: T) -> Self {
        Self { map: HashMap::from_iter(iter) }
    }
}

impl<RDF: Rdf> Display for ValueNodes<RDF> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValueNodes[{}]",
            self
                .map
                .iter()
                .map(|(node, vnodes)| format!("{} -> {}", node, vnodes))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}