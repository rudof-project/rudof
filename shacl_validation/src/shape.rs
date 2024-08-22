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
use crate::validate_error::ValidateError;
use crate::validation_report::result::LazyValidationIterator;
use crate::Targets;
use crate::ValueNodes;

pub struct ShapeValidator<'a, S: SRDFBasic> {
    shape: &'a Shape,
    validation_context: &'a ValidationContext<'a, S>,
    focus_nodes: &'a Targets<S>,
}

impl<'a, S: SRDFBasic + 'a> ShapeValidator<'a, S> {
    pub fn new(
        shape: &'a Shape,
        validation_context: &'a ValidationContext<S>,
        focus_nodes: Option<&'a Targets<S>>,
    ) -> Self {
        let focus_nodes = match focus_nodes {
            Some(focus) => focus,
            None => {
                let generated_focus_nodes = shape.focus_nodes(validation_context);
                generated_focus_nodes
            }
        };

        ShapeValidator {
            shape,
            validation_context,
            focus_nodes,
        }
    }

    pub fn validate(&self) -> Result<LazyValidationIterator<'_, S>, ValidateError> {
        if *self.shape.is_deactivated() {
            // skipping because it is deactivated
            return Ok(LazyValidationIterator::default());
        }

        let components = self.validate_components()?;
        let property_shapes = self.validate_property_shapes()?;

        Ok(LazyValidationIterator::new(
            components.chain(property_shapes),
        ))
    }

    fn validate_components(&self) -> Result<LazyValidationIterator<'_, S>, ValidateError> {
        let value_nodes = self
            .shape
            .value_nodes(self.validation_context, &self.focus_nodes);

        let contexts = self
            .shape
            .components()
            .iter()
            .map(|component| EvaluationContext::new(component, &self.shape));

        let evaluated_components = contexts.flat_map(move |context| {
            self.validation_context
                .runner()
                .evaluate(self.validation_context, context, &value_nodes)
                .unwrap_or_else(|_| LazyValidationIterator::default())
        });

        Ok(LazyValidationIterator::new(evaluated_components))
    }

    fn validate_property_shapes(&self) -> Result<LazyValidationIterator<'_, S>, ValidateError> {
        let shapes = get_shapes_ref(
            self.shape.property_shapes(),
            self.validation_context.schema(),
        );

        let evaluated_shapes = shapes
            .into_iter()
            .flatten()
            .filter_map(move |shape| {
                if let Shape::PropertyShape(_) = shape {
                    Some(ShapeValidator::new(
                        shape,
                        self.validation_context,
                        Some(self.focus_nodes),
                    ))
                } else {
                    None
                }
            })
            .flat_map(|context| {
                context
                    .validate()
                    .ok()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            });

        Ok(LazyValidationIterator::new(evaluated_shapes))
    }
}

pub trait FocusNodesOps<S: SRDFBasic> {
    fn focus_nodes(&self, validation_context: &ValidationContext<S>) -> &Targets<S>;
}

impl<S: SRDFBasic> FocusNodesOps<S> for Shape {
    fn focus_nodes(&self, validation_context: &ValidationContext<S>) -> &Targets<S> {
        validation_context
            .runner()
            .focus_nodes(
                validation_context.store(),
                &S::object_as_term(self.id()),
                self.targets(),
            )
            .expect("Failed to retrieve focus nodes")
    }
}

pub trait ShapeInfo {
    fn is_deactivated(&self) -> &bool;
    fn id(&self) -> &RDFNode;
    fn targets(&self) -> &Vec<Target>;
    fn components(&self) -> &Vec<Component>;
    fn property_shapes(&self) -> &Vec<RDFNode>;
}

impl ShapeInfo for Shape {
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

pub trait ValueNodesOps<S: SRDFBasic> {
    fn value_nodes(
        &self,
        validation_context: &ValidationContext<S>,
        focus_nodes: &Targets<S>,
    ) -> ValueNodes<S>;
}

impl<S: SRDFBasic> ValueNodesOps<S> for Shape {
    fn value_nodes(
        &self,
        validation_context: &ValidationContext<S>,
        focus_nodes: &Targets<S>,
    ) -> ValueNodes<S> {
        match self {
            Shape::NodeShape(ns) => ns.value_nodes(validation_context, focus_nodes),
            Shape::PropertyShape(ps) => ps.value_nodes(validation_context, focus_nodes),
        }
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for NodeShape {
    fn value_nodes(
        &self,
        validation_context: &ValidationContext<S>,
        focus_nodes: &Targets<S>,
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

impl<S: SRDFBasic> ValueNodesOps<S> for PropertyShape {
    fn value_nodes(
        &self,
        validation_context: &ValidationContext<S>,
        focus_nodes: &Targets<S>,
    ) -> ValueNodes<S> {
        let value_nodes =
            focus_nodes.iter().filter_map(move |focus_node| {
                match validation_context
                    .runner()
                    .path(validation_context.store(), self, &focus_node)
                    .ok()
                {
                    Some(targets) => Some((focus_node.clone(), targets)),
                    None => None,
                }
            });

        ValueNodes::new(value_nodes)
    }
}
