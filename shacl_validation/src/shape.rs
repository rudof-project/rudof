use shacl_ast::compiled::node_shape::CompiledNodeShape;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::SRDFBasic;

use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResults;
use crate::value_nodes::ValueNodes;

// TODO: can this ShapeValidation thing be transformed into a trait that is
//       implemented for the CompiledShape struct. The thing is that focus_nodes
//       may now be a parameter?

pub trait Validate<S: SRDFBasic> {
    fn validate(
        &self,
        store: &S,
        runner: &dyn Engine<S>,
        targets: Option<&FocusNodes<S>>, // TODO: can this be put inside of the CompiledShape?
    ) -> Result<ValidationResults<S>, ValidateError>;
}

impl<S: SRDFBasic> Validate<S> for CompiledShape<S> {
    fn validate(
        &self,
        store: &S,
        runner: &dyn Engine<S>,
        targets: Option<&FocusNodes<S>>,
    ) -> Result<ValidationResults<S>, ValidateError> {
        // 0.
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(ValidationResults::default());
        }

        // 1.
        let focus_nodes = match targets {
            Some(targets) => targets.to_owned(),
            None => self.focus_nodes(store, runner),
        };

        // 2. Second we compute the ValueNodes; that is, the set of nodes that
        //    are going to be used during the validation stages. This set of
        //    nodes is obtained from the set of focus nodes
        let value_nodes = self.value_nodes(store, &focus_nodes, runner);

        // 3.
        let component_validation_results = self.components().iter().flat_map(move |component| {
            runner
                .evaluate(store, component, &value_nodes)
                .unwrap_or_else(|_| ValidationResults::default())
        });

        // 4.
        let property_shapes_validation_results = self.property_shapes().iter().flat_map(|shape| {
            shape
                .validate(store, runner, Some(&focus_nodes))
                .ok()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        });

        // 5.
        let validation_results =
            component_validation_results.chain(property_shapes_validation_results);

        Ok(ValidationResults::new(validation_results))
    }
}

pub trait FocusNodesOps<S: SRDFBasic> {
    fn focus_nodes(&self, store: &S, runner: &dyn Engine<S>) -> FocusNodes<S>;
}

impl<S: SRDFBasic> FocusNodesOps<S> for CompiledShape<S> {
    fn focus_nodes(&self, store: &S, runner: &dyn Engine<S>) -> FocusNodes<S> {
        runner
            .focus_nodes(store, self, self.targets())
            .expect("Failed to retrieve focus nodes") // TODO: expect?
    }
}

pub trait ValueNodesOps<S: SRDFBasic> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> ValueNodes<S>;
}

impl<S: SRDFBasic> ValueNodesOps<S> for CompiledShape<S> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> ValueNodes<S> {
        match self {
            CompiledShape::NodeShape(ns) => ns.value_nodes(store, focus_nodes, runner),
            CompiledShape::PropertyShape(ps) => ps.value_nodes(store, focus_nodes, runner),
        }
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for CompiledNodeShape<S> {
    fn value_nodes(&self, _: &S, focus_nodes: &FocusNodes<S>, _: &dyn Engine<S>) -> ValueNodes<S> {
        let value_nodes = focus_nodes.iter().map(|focus_node| {
            (
                focus_node.clone(),
                FocusNodes::new(std::iter::once(focus_node.clone())),
            )
        });

        ValueNodes::new(value_nodes)
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for CompiledPropertyShape<S> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> ValueNodes<S> {
        let value_nodes = focus_nodes.iter().filter_map(move |focus_node| {
            runner
                .path(store, self, focus_node)
                .ok()
                .map(|targets| (focus_node.clone(), targets))
        });

        ValueNodes::new(value_nodes)
    }
}
