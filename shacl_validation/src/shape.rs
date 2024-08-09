use std::collections::HashSet;

use shacl_ast::node_shape::NodeShape;
use shacl_ast::property_shape::PropertyShape;
use shacl_ast::shape::Shape;
use shacl_ast::Schema;
use srdf::SRDFBasic;

use crate::helper::shapes::get_shapes_ref;
use crate::runner::ValidatorRunner;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub trait Validate<S: SRDFBasic> {
    fn validate(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        schema: &Schema,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError>;

    fn check_shape(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        schema: &Schema,
        value_nodes: Option<&HashSet<S::Term>>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError>;
}

impl<S: SRDFBasic> Validate<S> for NodeShape {
    fn validate(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        schema: &Schema,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(true);
        }
        self.check_shape(store, runner, schema, None, report)
    }

    fn check_shape(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        schema: &Schema,
        value_nodes: Option<&HashSet<S::Term>>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        let mut ans = true; // validation status of the current Shape
        let focus_nodes =
            runner.focus_nodes(store, &S::object_as_term(&self.id()), self.targets())?;
        let value_nodes = match value_nodes {
            Some(value_nodes) => value_nodes,
            None => &focus_nodes,
        };
        // we validate the components defined in the current Shape...
        for component in self.components() {
            ans = runner.evaluate(store, schema, component, value_nodes, report)?;
        }
        // ... and the ones in the nested Property Shapes
        for shape in get_shapes_ref(self.property_shapes(), schema)
            .into_iter()
            .flatten()
        {
            ans = match shape {
                Shape::NodeShape(_) => todo!(),
                Shape::PropertyShape(ps) => {
                    ps.check_shape(store, runner, schema, Some(value_nodes), report)?
                }
            }
        }
        Ok(ans)
    }
}

impl<S: SRDFBasic> Validate<S> for PropertyShape {
    fn validate(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        schema: &Schema,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(true);
        }
        self.check_shape(store, runner, schema, None, report)
    }

    fn check_shape(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        schema: &Schema,
        targets: Option<&HashSet<<S as SRDFBasic>::Term>>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        let mut ans = true; // validation status of the current Shape
        let mut value_nodes = HashSet::new();
        let focus_nodes =
            runner.focus_nodes(store, &S::object_as_term(self.id()), self.targets())?;
        for focus_node in match targets {
            Some(focus_nodes) => focus_nodes,
            None => &focus_nodes,
        } {
            runner.path(store, self, focus_node, &mut value_nodes)?;
        }
        // we validate the components defined in the current Shape...
        for component in self.components() {
            ans = runner.evaluate(store, schema, component, &value_nodes, report)?;
        }
        // ... and the ones in the nested Property Shapes
        for shape in get_shapes_ref(self.property_shapes(), schema)
            .into_iter()
            .flatten()
        {
            ans = match shape {
                Shape::NodeShape(_) => todo!(),
                Shape::PropertyShape(ps) => {
                    ps.check_shape(store, runner, schema, Some(&value_nodes), report)?
                }
            }
        }
        Ok(ans)
    }
}
