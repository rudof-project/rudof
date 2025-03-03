use shacl_ast::compiled::node_shape::CompiledNodeShape;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Rdf;

use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

/// Validate RDF data using SHACL
pub trait Validate<R: Rdf> {
    fn validate<E: Engine<R>>(
        &self,
        store: &R,
        targets: Option<&FocusNodes<R>>, // TODO: improve this naming convention
    ) -> Result<Vec<ValidationResult>, ValidateError>;
}

impl<R: Rdf> Validate<R> for CompiledShape<R> {
    fn validate<E: Engine<R>>(
        &self,
        store: &R,
        targets: Option<&FocusNodes<R>>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        // 0. skipping if it is deactivated
        if *self.is_deactivated() {
            return Ok(Vec::default());
        }

        // 1.
        let focus_nodes = match targets {
            Some(targets) => targets.clone(), // TODO: remove the clone
            None => self.focus_nodes::<E>(store),
        };

        // 2. Second we compute the ValueNodes; that is, the set of nodes that
        //    are going to be used during the validation stages. This set of
        //    nodes is obtained from the set of focus nodes
        let value_nodes = self.value_nodes::<E>(store, &focus_nodes);

        // 3.
        let component_validation_results = self
            .components()
            .iter()
            .flat_map(move |component| E::evaluate(store, self, component, &value_nodes));

        // 4. After validating the constraints that are defined in the current
        //    Shape, it is important to also perform the validation over those
        //    nested PropertyShapes. The thing is that the validation needs to
        //    occur over the focus_nodes that have been computed for the current
        //    shape
        let property_shapes_validation_results = self
            .property_shapes()
            .iter()
            .flat_map(|shape| shape.validate::<E>(store, Some(&focus_nodes)));

        // 5.
        let validation_results = component_validation_results
            .chain(property_shapes_validation_results)
            .flatten()
            .collect(); // TODO: is it necessary to collect?

        Ok(validation_results)
    }
}

pub trait FocusNodesOps<R: Rdf> {
    fn focus_nodes<E: Engine<R>>(&self, store: &R) -> FocusNodes<R>;
}

impl<R: Rdf> FocusNodesOps<R> for CompiledShape<R> {
    fn focus_nodes<E: Engine<R>>(&self, store: &R) -> FocusNodes<R> {
        // TODO: remove the expect
        E::focus_nodes(store, self.targets()).expect("Failed to retrieve focus nodes")
    }
}

pub trait ValueNodesOps<R: Rdf> {
    fn value_nodes<E: Engine<R>>(&self, store: &R, focus_nodes: &FocusNodes<R>) -> ValueNodes<R>;
}

impl<R: Rdf> ValueNodesOps<R> for CompiledShape<R> {
    fn value_nodes<E: Engine<R>>(&self, store: &R, focus_nodes: &FocusNodes<R>) -> ValueNodes<R> {
        match self {
            CompiledShape::NodeShape(ns) => ns.value_nodes::<E>(store, focus_nodes),
            CompiledShape::PropertyShape(ps) => ps.value_nodes::<E>(store, focus_nodes),
        }
    }
}

impl<R: Rdf> ValueNodesOps<R> for CompiledNodeShape<R> {
    fn value_nodes<E: Engine<R>>(&self, _: &R, focus_nodes: &FocusNodes<R>) -> ValueNodes<R> {
        let value_nodes = focus_nodes.iter().map(|focus_node| {
            let single_focus_node = FocusNodes::new(std::iter::once(focus_node.clone()));
            (focus_node.clone(), single_focus_node)
        });

        ValueNodes::new(value_nodes)
    }
}

impl<R: Rdf> ValueNodesOps<R> for CompiledPropertyShape<R> {
    fn value_nodes<E: Engine<R>>(&self, store: &R, focus_nodes: &FocusNodes<R>) -> ValueNodes<R> {
        let value_nodes = focus_nodes.iter().filter_map(|focus_node| {
            E::path(store, self, focus_node)
                .ok()
                .map(|targets| (focus_node.clone(), targets))
        });
        ValueNodes::new(value_nodes)
    }
}
