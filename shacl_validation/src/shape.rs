use std::collections::HashMap;
use std::collections::HashSet;

use shacl_ast::node_shape::NodeShape;
use shacl_ast::property_shape::PropertyShape;
use shacl_ast::shape::Shape;
use srdf::SRDFBasic;

use crate::context::Context;
use crate::executor::SHACLExecutor;
use crate::helper::shapes::get_shapes_ref;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub type FocusNode<S> = HashSet<<S as SRDFBasic>::Term>;
pub type ValueNode<S> = HashMap<<S as SRDFBasic>::Term, HashSet<<S as SRDFBasic>::Term>>;

pub trait Validate<S: SRDFBasic> {
    fn validate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError>;

    fn check_shape(
        &self,
        executor: &dyn SHACLExecutor<S>,
        focus_nodes: Option<&FocusNode<S>>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError>;
}

impl<S: SRDFBasic> Validate<S> for NodeShape {
    fn validate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(true);
        }
        self.check_shape(executor, None, report)
    }

    fn check_shape(
        &self,
        executor: &dyn SHACLExecutor<S>,
        focus_nodes: Option<&FocusNode<S>>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        let mut ans = true; // validation status of the current Shape
        let focus_nodes = match focus_nodes {
            Some(focus_nodes) => focus_nodes.to_owned(),
            None => executor.runner().focus_nodes(
                executor.store(),
                &S::object_as_term(&self.id()),
                self.targets(),
            )?,
        };
        let mut value_nodes = ValueNode::<S>::new();
        for focus_node in &focus_nodes {
            value_nodes.insert(
                focus_node.to_owned(),
                vec![focus_node.to_owned()].into_iter().collect(),
            );
        }
        // we validate the components defined in the current Shape...
        for component in self.components() {
            ans = executor.evaluate(
                &Context::new(component, Shape::NodeShape(Box::new(self.clone()))),
                &value_nodes,
                report,
            )?;
        }
        // ... and the ones in the nested Property Shapes
        for shape in get_shapes_ref(self.property_shapes(), executor.schema())
            .into_iter()
            .flatten()
        {
            ans = match shape {
                Shape::NodeShape(_) => todo!(),
                Shape::PropertyShape(ps) => ps.check_shape(executor, Some(&focus_nodes), report)?,
            }
        }
        Ok(ans)
    }
}

impl<S: SRDFBasic> Validate<S> for PropertyShape {
    fn validate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(true);
        }
        self.check_shape(executor, None, report)
    }

    fn check_shape(
        &self,
        executor: &dyn SHACLExecutor<S>,
        focus_nodes: Option<&FocusNode<S>>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        let mut ans = true; // validation status of the current Shape
        let focus_nodes = match focus_nodes {
            Some(focus_nodes) => focus_nodes.to_owned(),
            None => executor.runner().focus_nodes(
                executor.store(),
                &S::object_as_term(self.id()),
                self.targets(),
            )?,
        };
        let mut value_nodes = ValueNode::<S>::new();
        for focus_node in &focus_nodes {
            executor
                .runner()
                .path(executor.store(), self, focus_node, &mut value_nodes)?;
        }
        // we validate the components defined in the current Shape...
        for component in self.components() {
            ans = executor.evaluate(
                &Context::new(component, Shape::PropertyShape(self.clone())),
                &value_nodes,
                report,
            )?;
        }
        // ... and the ones in the nested Property Shapes
        for shape in get_shapes_ref(self.property_shapes(), executor.schema())
            .into_iter()
            .flatten()
        {
            ans = match shape {
                Shape::NodeShape(_) => todo!(),
                Shape::PropertyShape(ps) => ps.check_shape(executor, Some(&focus_nodes), report)?,
            }
        }
        Ok(ans)
    }
}
