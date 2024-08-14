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
use crate::validation_report::result::ValidationResult;

pub type FocusNode<S> = HashSet<<S as SRDFBasic>::Term>;
pub type ValueNode<S> = HashMap<<S as SRDFBasic>::Term, HashSet<<S as SRDFBasic>::Term>>;
pub type ValidateResult<S> = Result<Vec<ValidationResult<S>>, ValidateError>;

pub trait Validate<S: SRDFBasic> {
    fn validate(&self, executor: &dyn SHACLExecutor<S>) -> ValidateResult<S>;

    fn check_shape(
        &self,
        executor: &dyn SHACLExecutor<S>,
        focus_nodes: Option<&FocusNode<S>>,
    ) -> ValidateResult<S>;
}

impl<S: SRDFBasic> Validate<S> for NodeShape {
    fn validate(&self, executor: &dyn SHACLExecutor<S>) -> ValidateResult<S> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(Vec::new());
        }
        self.check_shape(executor, None)
    }

    fn check_shape(
        &self,
        executor: &dyn SHACLExecutor<S>,
        focus_nodes: Option<&FocusNode<S>>,
    ) -> ValidateResult<S> {
        let mut results = Vec::new(); // validation status of the current Shape

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
            results.extend(executor.evaluate(
                &Context::new(component, Shape::NodeShape(Box::new(self.clone()))),
                &value_nodes,
            )?);
        }

        // ... and the ones in the nested Property Shapes
        for shape in get_shapes_ref(self.property_shapes(), executor.schema())
            .into_iter()
            .flatten()
        {
            results.extend(match shape {
                Shape::NodeShape(_) => todo!(),
                Shape::PropertyShape(ps) => ps.check_shape(executor, Some(&focus_nodes))?,
            });
        }

        Ok(results)
    }
}

impl<S: SRDFBasic> Validate<S> for PropertyShape {
    fn validate(&self, executor: &dyn SHACLExecutor<S>) -> ValidateResult<S> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(Vec::new());
        }
        self.check_shape(executor, None)
    }

    fn check_shape(
        &self,
        executor: &dyn SHACLExecutor<S>,
        focus_nodes: Option<&FocusNode<S>>,
    ) -> ValidateResult<S> {
        let mut results = Vec::new(); // validation status of the current Shape

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
            results.extend(executor.evaluate(
                &Context::new(component, Shape::PropertyShape(self.clone())),
                &value_nodes,
            )?);
        }

        // ... and the ones in the nested Property Shapes
        for shape in get_shapes_ref(self.property_shapes(), executor.schema())
            .into_iter()
            .flatten()
        {
            results.extend(match shape {
                Shape::NodeShape(_) => todo!(),
                Shape::PropertyShape(ps) => ps.check_shape(executor, Some(&focus_nodes))?,
            });
        }

        Ok(results)
    }
}
