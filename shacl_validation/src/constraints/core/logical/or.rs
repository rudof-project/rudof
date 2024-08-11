use shacl_ast::shape::Shape;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::helper::shapes::get_shapes_ref;
use crate::shape::Validate;
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
pub(crate) struct Or {
    shapes: Vec<RDFNode>,
}

impl Or {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        Or { shapes }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Or {
    fn evaluate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let shapes = get_shapes_ref(&self.shapes, executor.schema());
        let mut is_valid = true;

        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                let single_value_nodes = std::iter::once(value_node.to_owned()).collect();

                let any_valid = shapes.iter().flatten().any(|shape| {
                    let result = match shape {
                        Shape::NodeShape(shape) => shape.check_shape(
                            executor,
                            Some(&single_value_nodes),
                            &mut ValidationReport::default(),
                        ),
                        Shape::PropertyShape(shape) => shape.check_shape(
                            executor,
                            Some(&single_value_nodes),
                            &mut ValidationReport::default(),
                        ),
                    };
                    result.unwrap_or(false)
                });

                if !any_valid {
                    is_valid = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                }
            }
        }

        Ok(is_valid)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Or {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(executor, context, value_nodes, report)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Or {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(executor, context, value_nodes, report)
    }
}
