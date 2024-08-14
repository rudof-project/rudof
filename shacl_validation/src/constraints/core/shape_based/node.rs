use shacl_ast::shape::Shape;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::ConstraintResult;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::helper::shapes::get_shape_ref;
use crate::shape::Validate;
use crate::shape::ValueNode;
use crate::validation_report::result::ValidationResult;

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
pub(crate) struct Node {
    shape: RDFNode,
}

impl Node {
    pub fn new(shape: RDFNode) -> Self {
        Node { shape }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Node {
    fn evaluate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        let mut results = Vec::new();

        let shape = match get_shape_ref(&self.shape, executor.schema()) {
            Some(shape) => shape,
            None => return Err(ConstraintError::MissingShape),
        };

        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                let single_value_nodes = std::iter::once(value_node.to_owned()).collect();

                let inner_results = match shape {
                    Shape::NodeShape(shape) => {
                        shape.check_shape(executor, Some(&single_value_nodes))
                    }
                    Shape::PropertyShape(shape) => {
                        shape.check_shape(executor, Some(&single_value_nodes))
                    }
                };

                if inner_results.is_err() {
                    results.push(ValidationResult::new(focus_node, context, Some(value_node)));
                }

                if !inner_results.unwrap().is_empty() {
                    results.push(ValidationResult::new(focus_node, context, Some(value_node)));
                }
            }
        }

        Ok(results)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Node {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Node {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}
