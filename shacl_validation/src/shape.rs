use shacl_ast::compiled::node_shape::NodeShape;
use shacl_ast::compiled::property_shape::PropertyShape;
use shacl_ast::compiled::shape::Shape;
use srdf::SRDFBasic;

use crate::runner::ValidatorRunner;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

pub struct ShapeValidation<'a, S: SRDFBasic> {
    store: &'a S,
    runner: &'a dyn ValidatorRunner<S>,
    shape: &'a Shape<S>,
    focus_nodes: Targets<S>,
}

impl<'a, S: SRDFBasic> ShapeValidation<'a, S> {
    pub fn new(
        store: &'a S,
        runner: &'a dyn ValidatorRunner<S>,
        shape: &'a Shape<S>,
        targets: Option<&'a Targets<S>>,
    ) -> Self {
        let focus_nodes = match targets {
            Some(targets) => targets.to_owned(),
            None => shape.focus_nodes(store, runner),
        };

        ShapeValidation {
            store,
            runner,
            shape,
            focus_nodes,
        }
    }

    pub fn validate(&self) -> Result<ValidationResults<S>, ValidateError> {
        if *self.shape.is_deactivated() {
            // skipping because it is deactivated
            return Ok(ValidationResults::default());
        }

        let components = self.validate_components()?;
        let property_shapes = self.validate_property_shapes()?;
        let validation_results = components.into_iter().chain(property_shapes);

        Ok(ValidationResults::new(validation_results))
    }

    fn validate_components(&self) -> Result<ValidationResults<S>, ValidateError> {
        // 1. First we compute the ValueNodes; that is, the set of nodes that
        //    are going to be used during the validation stages. This set of
        //    nodes is obtained from the set of focus nodes
        let value_nodes = self
            .shape
            .value_nodes(self.store, &self.focus_nodes, self.runner);

        // let mut unique_components = HashSet::new(); TODO: check whether this works

        // let contexts: Vec<_> = self
        //     .shape
        //     .components()
        //     .iter()
        //     .filter_map(|component| {
        //         if unique_components.insert(component) {
        //             Some(EvaluationContext::new(component, self.shape))
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();

        let evaluated_components = self
            .shape
            .components()
            .into_iter()
            .flat_map(move |component| {
                self.runner
                    .evaluate(self.store, component, &value_nodes)
                    .unwrap_or_else(|_| ValidationResults::default())
            });

        Ok(ValidationResults::new(evaluated_components))
    }

    fn validate_property_shapes(&self) -> Result<ValidationResults<S>, ValidateError> {
        let evaluated_shapes = self
            .shape
            .property_shapes()
            .iter()
            .filter_map(|shape| {
                Some(ShapeValidation::new(
                    self.store,
                    self.runner,
                    shape,
                    Some(&self.focus_nodes),
                ))
            })
            .flat_map(|validation| {
                validation
                    .validate()
                    .ok()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            });

        Ok(ValidationResults::new(evaluated_shapes))
    }
}

pub trait FocusNodesOps<S: SRDFBasic> {
    fn focus_nodes(&self, store: &S, runner: &dyn ValidatorRunner<S>) -> Targets<S>;
}

impl<S: SRDFBasic> FocusNodesOps<S> for Shape<S> {
    fn focus_nodes(&self, store: &S, runner: &dyn ValidatorRunner<S>) -> Targets<S> {
        runner
            .focus_nodes(store, self, self.targets())
            .expect("Failed to retrieve focus nodes")
    }
}

pub trait ValueNodesOps<S: SRDFBasic> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &Targets<S>,
        runner: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S>;
}

impl<S: SRDFBasic> ValueNodesOps<S> for Shape<S> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &Targets<S>,
        runner: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S> {
        match self {
            Shape::NodeShape(ns) => ns.value_nodes(store, focus_nodes, runner),
            Shape::PropertyShape(ps) => ps.value_nodes(store, focus_nodes, runner),
        }
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for NodeShape<S> {
    fn value_nodes(
        &self,
        _: &S,
        focus_nodes: &Targets<S>,
        _: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S> {
        let value_nodes = focus_nodes.iter().map(|focus_node| {
            (
                focus_node.clone(),
                Targets::new(std::iter::once(focus_node.clone())),
            )
        });

        ValueNodes::new(value_nodes)
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for PropertyShape<S> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &Targets<S>,
        runner: &dyn ValidatorRunner<S>,
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
