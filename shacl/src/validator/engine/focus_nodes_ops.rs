use crate::ir::IRShape;
use crate::validator::engine::Engine;
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf};

pub(crate) trait FocusNodesOps<RDF: Rdf> {
    fn focus_nodes(&self, store: &RDF, runner: &dyn Engine<RDF>) -> FocusNodes<RDF>;
}

impl<RDF: NeighsRDF> FocusNodesOps<RDF> for IRShape {
    fn focus_nodes(&self, store: &RDF, runner: &dyn Engine<RDF>) -> FocusNodes<RDF> {
        runner
            .focus_nodes(store, self.targets())
            .expect("Failed to retrieve focus nodes")
    }
}
