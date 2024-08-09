use std::collections::HashSet;

use shacl_ast::shape::Shape;
use shacl_ast::Schema;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::helper::shapes::get_shapes_ref;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::shape::Validate;
use crate::validation_report::report::ValidationReport;

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
        store: &S,
        schema: &Schema,
        runner: &dyn ValidatorRunner<S>,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let shapes = get_shapes_ref(&self.shapes, schema);
        let mut is_valid = true;

        for value_node in value_nodes {
            let targets: HashSet<_> = std::iter::once(value_node.clone()).collect();
            let all_valid = shapes.iter().flatten().all(|shape| {
                let result = match shape {
                    Shape::NodeShape(shape) => shape.check_shape(
                        store,
                        runner,
                        schema,
                        Some(&targets),
                        &mut ValidationReport::default(),
                    ),
                    Shape::PropertyShape(shape) => shape.check_shape(
                        store,
                        runner,
                        schema,
                        Some(&targets),
                        &mut ValidationReport::default(),
                    ),
                };
                result.unwrap_or(false)
            });
            if !all_valid {
                is_valid = false;
                report.make_validation_result(Some(value_node));
            }
        }

        Ok(is_valid)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for And {
    fn evaluate_default(
        &self,
        store: &S,
        schema: &Schema,
        runner: &DefaultValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(store, schema, runner, value_nodes, report)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for And {
    fn evaluate_sparql(
        &self,
        store: &S,
        schema: &Schema,
        runner: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(store, schema, runner, value_nodes, report)
    }
}
