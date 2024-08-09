use std::collections::HashSet;

use shacl_ast::node_shape::NodeShape;
use shacl_ast::property_shape::PropertyShape;
use srdf::SRDFBasic;

use crate::runner::ValidatorRunner;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub trait Validate<S: SRDFBasic> {
    fn validate(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ValidateError>;
}

impl<S: SRDFBasic> Validate<S> for NodeShape {
    fn validate(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(());
        }

        for component in self.components() {
            let value_nodes = runner.focus_nodes(store, self.targets())?;
            runner.evaluate(store, component, value_nodes, report)?;
        }

        Ok(())
    }
}

impl<S: SRDFBasic> Validate<S> for PropertyShape {
    fn validate(
        &self,
        store: &S,
        runner: &dyn ValidatorRunner<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(());
        }

        for component in self.components() {
            let focus_nodes = runner.focus_nodes(store, self.targets())?;
            let mut value_nodes = HashSet::new();
            for focus_node in focus_nodes {
                runner.path(store, self, focus_node, &mut value_nodes)?;
            }
            runner.evaluate(store, component, value_nodes, report)?;
        }

        Ok(())
    }
}
