use std::collections::HashSet;

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
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

pub struct ShapeValidator<'a, S: SRDFBasic> {
    shape: &'a Shape,
    validation_context: &'a ValidationContext<'a, S>,
    focus_nodes: Targets<S>,
}

impl<'a, S: SRDFBasic + 'a> ShapeValidator<'a, S> {
    pub fn new(
        shape: &'a Shape,
        validation_context: &'a ValidationContext<S>,
        focus_nodes: Option<&'a Targets<S>>,
    ) -> Self {
        let focus_nodes = match focus_nodes {
            Some(focus) => focus.to_owned(),
            None => shape.focus_nodes(validation_context),
        };

        ShapeValidator {
            shape,
            validation_context,
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

        Ok(ValidationResults::new(
            components.into_iter().chain(property_shapes),
        ))
    }

    fn validate_components(&self) -> Result<ValidationResults<S>, ValidateError> {
        let value_nodes = self
            .shape
            .value_nodes(self.validation_context, &self.focus_nodes);

        let runner = self.validation_context.runner();
        let validation_context = self.validation_context;
        let mut unique_components = HashSet::new();

        // Mover la creación del contexto fuera del cierre
        let contexts: Vec<_> = self
            .shape
            .components()
            .iter()
            .filter_map(|component| {
                if unique_components.insert(component.clone()) {
                    Some(EvaluationContext::new(component, self.shape))
                } else {
                    None
                }
            })
            .collect();

        let evaluated_components = contexts.into_iter().flat_map(move |context| {
            runner
                .evaluate(validation_context, context, &value_nodes)
                .unwrap_or_else(|_| ValidationResults::default())
        });

        Ok(ValidationResults::new(evaluated_components))
    }

    fn validate_property_shapes(&self) -> Result<ValidationResults<S>, ValidateError> {
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
                        Some(&self.focus_nodes),
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

        Ok(ValidationResults::new(evaluated_shapes))
    }
}

pub trait FocusNodesOps<S: SRDFBasic> {
    fn focus_nodes(&self, validation_context: &ValidationContext<S>) -> Targets<S>;
}

impl<S: SRDFBasic> FocusNodesOps<S> for Shape {
    fn focus_nodes(&self, validation_context: &ValidationContext<S>) -> Targets<S> {
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
            Shape::NodeShape(ns) => ns.is_deactivated(),
            Shape::PropertyShape(ps) => ps.is_deactivated(),
        }
    }

    fn id(&self) -> &RDFNode {
        match self {
            Shape::NodeShape(ns) => ns.id(),
            Shape::PropertyShape(ps) => ps.id(),
        }
    }

    fn targets(&self) -> &Vec<Target> {
        match self {
            Shape::NodeShape(ns) => ns.targets(),
            Shape::PropertyShape(ps) => ps.targets(),
        }
    }

    fn components(&self) -> &Vec<Component> {
        match self {
            Shape::NodeShape(ns) => ns.components(),
            Shape::PropertyShape(ps) => ps.components(),
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
        _validation_context: &ValidationContext<S>,
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
        let value_nodes = focus_nodes.iter().filter_map(move |focus_node| {
            validation_context
                .runner()
                .path(validation_context.store(), self, focus_node)
                .ok()
                .map(|targets| (focus_node.clone(), targets))
        });

        ValueNodes::new(value_nodes)
    }
}
