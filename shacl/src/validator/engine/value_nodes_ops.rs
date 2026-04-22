use crate::error::ValidationError;
use crate::ir::{IRNodeShape, IRPropertyShape, IRShape};
use crate::validator::engine::Engine;
use crate::validator::nodes::{FocusNodes, ValueNodes};
use rudof_rdf::rdf_core::{NeighsRDF, Rdf};

pub(crate) trait ValueNodesOps<RDF: Rdf> {
    fn value_nodes(
        &self,
        store: &RDF,
        focus_nodes: &FocusNodes<RDF>,
        runner: &dyn Engine<RDF>,
    ) -> Result<ValueNodes<RDF>, ValidationError>;
}

impl<RDF: NeighsRDF> ValueNodesOps<RDF> for IRShape {
    fn value_nodes(
        &self,
        store: &RDF,
        focus_nodes: &FocusNodes<RDF>,
        runner: &dyn Engine<RDF>,
    ) -> Result<ValueNodes<RDF>, ValidationError> {
        match self {
            IRShape::NodeShape(ns) => ns.value_nodes(store, focus_nodes, runner),
            IRShape::PropertyShape(ps) => ps.value_nodes(store, focus_nodes, runner),
        }
    }
}

impl<RDF: NeighsRDF> ValueNodesOps<RDF> for IRNodeShape {
    fn value_nodes(
        &self,
        _: &RDF,
        focus_nodes: &FocusNodes<RDF>,
        _: &dyn Engine<RDF>,
    ) -> Result<ValueNodes<RDF>, ValidationError> {
        let value_nodes = focus_nodes.iter().map(|n| (n.clone(), FocusNodes::single(n.clone())));
        Ok(ValueNodes::from_iter(value_nodes))
    }
}

impl<RDF: NeighsRDF> ValueNodesOps<RDF> for IRPropertyShape {
    fn value_nodes(
        &self,
        store: &RDF,
        focus_nodes: &FocusNodes<RDF>,
        runner: &dyn Engine<RDF>,
    ) -> Result<ValueNodes<RDF>, ValidationError> {
        let value_nodes = focus_nodes.iter().filter_map(|n| {
            match runner.path(store, self, n) {
                Ok(ts) => Some((n.clone(), ts)),
                Err(_) => None, // TODO - Should we add a violation for this case?
            }
        });
        Ok(ValueNodes::from_iter(value_nodes))
    }
}
