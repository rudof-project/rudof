use std::sync::Arc;

use shacl_ast::component::Component;
use shacl_ast::node_shape::NodeShape;
use shacl_ast::property_shape::PropertyShape;
use shacl_ast::shape::Shape;
use shacl_ast::target::Target;
use srdf::RDFNode;
use srdf::SRDFBasic;

use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::shapes::get_shapes_ref;
use crate::runner::ValidatorRunner;
use crate::targets::Targets;
use crate::validate_error::ValidateError;
use crate::validation_report::result::LazyValidationIterator;
use crate::value_nodes::ValueNodes;

pub struct ShapeValidator<S: SRDFBasic, R: ValidatorRunner<S>> {
    shape: Shape,
    validation_context: ValidationContext<S, R>,
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ShapeValidator<S, R> {
    pub fn new(shape: Shape, validation_context: ValidationContext<S, R>) -> Self {
        ShapeValidator {
            shape,
            validation_context,
        }
    }

    pub fn validate(
        &self,
        focus_nodes: Arc<Targets<S>>,
    ) -> Result<LazyValidationIterator<S>, ValidateError> {
        if *self.shape.is_deactivated() {
            // skipping because it is deactivated
            return Ok(LazyValidationIterator::default());
        }

        let focus_nodes = self
            .shape
            .focus_nodes(Arc::clone(&self.validation_context), focus_nodes);
        let value_nodes = self.shape.value_nodes(
            Arc::clone(&self.validation_context),
            Arc::clone(&focus_nodes),
        );

        let component_results = self.validate_components(value_nodes)?;
        let property_shapes_results = self.validate_property_shapes(Arc::clone(&focus_nodes))?;
        let results = LazyValidationIterator::new(component_results.chain(property_shapes_results));

        Ok(results)
    }

    fn validate_components(
        &self,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> Result<LazyValidationIterator<S>, ValidateError> {
        let contexts = <Shape as ValueNodesOps<S, R>>::components(&self.shape)
            .iter()
            .map(|component| EvaluationContext::new(component, &self.shape));

        let evaluated_components = contexts.flat_map(move |context| {
            self.validation_context
                .runner()
                .evaluate(
                    Arc::clone(&self.validation_context),
                    Arc::new(context),
                    Arc::clone(&value_nodes),
                )
                .unwrap_or_else(|_| LazyValidationIterator::default())
        });

        Ok(LazyValidationIterator::new(evaluated_components))
    }

    fn validate_property_shapes(
        &self,
        focus_nodes: Arc<Targets<S>>,
    ) -> Result<LazyValidationIterator<S>, ValidateError> {
        let shapes = get_shapes_ref(
            <Shape as ValueNodesOps<S, R>>::property_shapes(&self.shape),
            self.validation_context.schema(),
        );

        let contexts = shapes.iter().flatten().filter_map(|shape| {
            if let Shape::PropertyShape(_) = shape {
                Some(ShapeValidator::new(
                    Arc::new(shape),
                    Arc::clone(&self.validation_context),
                ))
            } else {
                None
            }
        });

        let evaluated_shapes = contexts
            .flat_map(move |context| {
                match context.validate(Arc::clone(&focus_nodes)) {
                    Ok(results) => Some(results),
                    Err(_) => None, // handle validation errors if necessary
                }
            })
            .flatten();

        Ok(LazyValidationIterator::new(evaluated_shapes))
    }
}

pub trait FocusNodesOps<S: SRDFBasic, R: ValidatorRunner<S>> {
    fn focus_nodes(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        focus_nodes: Arc<Targets<S>>,
    ) -> Arc<Targets<S>>;
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> FocusNodesOps<S, R> for Shape {
    fn focus_nodes(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        focus_nodes: Arc<Targets<S>>,
    ) -> Arc<Targets<S>> {
        if focus_nodes.peekable().peek().is_some() {
            focus_nodes
        } else {
            let result = validation_context
                .runner()
                .focus_nodes(
                    validation_context.store(),
                    &S::object_as_term(<Shape as ValueNodesOps<S, R>>::id(self)),
                    <Shape as ValueNodesOps<S, R>>::targets(self),
                )
                .expect("Failed to retrieve focus nodes");

            Arc::new(result)
        }
    }
}

pub trait ValueNodesOps<S: SRDFBasic, R: ValidatorRunner<S>> {
    fn value_nodes(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        focus_nodes: Arc<Targets<S>>,
    ) -> Arc<ValueNodes<S>>;

    fn is_deactivated(&self) -> &bool;
    fn id(&self) -> &RDFNode;
    fn targets(&self) -> &Vec<Target>;
    fn components(&self) -> &Vec<Component>;
    fn property_shapes(&self) -> &Vec<RDFNode>;
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ValueNodesOps<S, R> for Shape {
    fn value_nodes(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        focus_nodes: Arc<Targets<S>>,
    ) -> Arc<ValueNodes<S>> {
        match self {
            Shape::NodeShape(ns) => ns.value_nodes(validation_context, focus_nodes),
            Shape::PropertyShape(ps) => ps.value_nodes(validation_context, focus_nodes),
        }
    }

    fn is_deactivated(&self) -> &bool {
        match self {
            Shape::NodeShape(ref ns) => ns.is_deactivated(),
            Shape::PropertyShape(ref ps) => ps.is_deactivated(),
        }
    }

    fn id(&self) -> &RDFNode {
        match self {
            Shape::NodeShape(ref ns) => &ns.id(),
            Shape::PropertyShape(ref ps) => ps.id(),
        }
    }

    fn targets(&self) -> &Vec<Target> {
        match self {
            Shape::NodeShape(ref ns) => ns.targets(),
            Shape::PropertyShape(ref ps) => ps.targets(),
        }
    }

    fn components(&self) -> &Vec<Component> {
        match self {
            Shape::NodeShape(ref ns) => ns.components(),
            Shape::PropertyShape(ref ps) => ps.components(),
        }
    }

    fn property_shapes(&self) -> &Vec<RDFNode> {
        match self {
            Shape::NodeShape(ns) => ns.property_shapes(),
            Shape::PropertyShape(ps) => ps.property_shapes(),
        }
    }
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ValueNodesOps<S, R> for NodeShape {
    fn value_nodes(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        focus_nodes: Arc<Targets<S>>,
    ) -> Arc<ValueNodes<S>> {
        let value_nodes = focus_nodes.map(|focus_node| {
            (
                focus_node.clone(),
                Targets::new(std::iter::once(focus_node.clone())),
            )
        });

        Arc::new(ValueNodes::new(value_nodes))
    }

    fn is_deactivated(&self) -> &bool {
        self.is_deactivated()
    }

    fn id(&self) -> &RDFNode {
        self.id()
    }

    fn targets(&self) -> &Vec<Target> {
        self.targets()
    }

    fn components(&self) -> &Vec<Component> {
        self.components()
    }

    fn property_shapes(&self) -> &Vec<RDFNode> {
        self.property_shapes()
    }
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ValueNodesOps<S, R> for PropertyShape {
    fn value_nodes(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        focus_nodes: Arc<Targets<S>>,
    ) -> Arc<ValueNodes<S>> {
        let value_nodes = focus_nodes.filter_map(move |focus_node| {
            match validation_context
                .runner()
                .path(validation_context.store(), self, &focus_node)
                .ok()
            {
                Some(targets) => Some((focus_node, targets)),
                None => None,
            }
        });

        Arc::new(ValueNodes::new(value_nodes))
    }

    fn is_deactivated(&self) -> &bool {
        self.is_deactivated()
    }

    fn id(&self) -> &RDFNode {
        self.id()
    }

    fn targets(&self) -> &Vec<Target> {
        self.targets()
    }

    fn components(&self) -> &Vec<Component> {
        self.components()
    }

    fn property_shapes(&self) -> &Vec<RDFNode> {
        self.property_shapes()
    }
}
