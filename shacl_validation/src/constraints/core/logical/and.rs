use shacl_ast::shape::Shape;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::ConstraintResult;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::helper::shapes::get_shapes_ref;
use crate::shape::Validate;
use crate::shape::ValueNode;
use crate::validation_report::result::ValidationResult;

/// sh:and specifies the condition that each value node conforms to all provided
/// shapes. This is comparable to conjunction and the logical "and" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
pub(crate) struct And {
    shapes: Vec<RDFNode>,
}

impl And {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        And { shapes }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for And {
    fn evaluate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        let shapes = get_shapes_ref(&self.shapes, executor.schema());
        let mut results = Vec::new();

        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                let single_value_nodes = std::iter::once(value_node.to_owned()).collect();

                // Iterate through shapes and validate them
                let all_valid = shapes.iter().flatten().all(|shape| {
                    let result = match shape {
                        Shape::NodeShape(shape) => {
                            shape.check_shape(executor, Some(&single_value_nodes))
                        }
                        Shape::PropertyShape(shape) => {
                            shape.check_shape(executor, Some(&single_value_nodes))
                        }
                    };

                    match result {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                });

                if !all_valid {
                    results.push(ValidationResult::new(focus_node, context, Some(value_node)));
                }
            }
        }

        Ok(results)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for And {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for And {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}
