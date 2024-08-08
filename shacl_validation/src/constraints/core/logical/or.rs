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
        store: &S,
        schema: &Schema,
        runner: &dyn ValidatorRunner<S>,
        _: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let shapes = get_shapes_ref(&self.shapes, schema);
        let ans = shapes
            .into_iter()
            .filter_map(|shape| shape)
            .any(|shape| match shape {
                Shape::NodeShape(shape) => shape
                    .validate(store, runner, schema, &mut ValidationReport::default())
                    .unwrap_or(false),
                Shape::PropertyShape(shape) => shape
                    .validate(store, runner, schema, &mut ValidationReport::default())
                    .unwrap_or(false),
            });
        if !ans {
            report.make_validation_result(None);
        }
        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Or {
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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Or {
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
