use shacl_ast::Schema;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::context::Context;
use crate::shape::ValueNode;
use crate::{
    constraints::{DefaultConstraintComponent, SparqlConstraintComponent},
    runner::{
        default_runner::DefaultValidatorRunner, query_runner::QueryValidatorRunner, ValidatorRunner,
    },
    validate_error::ValidateError,
    validation_report::report::ValidationReport,
};

pub trait SHACLExecutor<S: SRDFBasic> {
    fn store(&self) -> &S;
    fn schema(&self) -> &Schema;
    fn runner(&self) -> &dyn ValidatorRunner<S>;

    fn evaluate(
        &self,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError>;
}

pub struct DefaultExecutor<'a, S: SRDF> {
    store: &'a S,
    schema: Schema,
    runner: DefaultValidatorRunner,
}

impl<'a, S: SRDF> DefaultExecutor<'a, S> {
    pub(crate) fn new(store: &'a S, schema: Schema) -> Self {
        Self {
            store,
            schema,
            runner: DefaultValidatorRunner,
        }
    }
}

impl<'a, S: SRDF + 'static> SHACLExecutor<S> for DefaultExecutor<'a, S> {
    fn store(&self) -> &S {
        self.store
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn runner(&self) -> &dyn ValidatorRunner<S> {
        &self.runner
    }

    fn evaluate(
        &self,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        let component: Box<dyn DefaultConstraintComponent<S>> = context.component().into();
        Ok(component.evaluate_default(self, context, value_nodes, report)?)
    }
}

pub struct QueryExecutor<'a, S: QuerySRDF> {
    store: &'a S,
    schema: Schema,
    runner: QueryValidatorRunner,
}

impl<'a, S: QuerySRDF> QueryExecutor<'a, S> {
    pub(crate) fn new(store: &'a S, schema: Schema) -> Self {
        Self {
            store,
            schema,
            runner: QueryValidatorRunner,
        }
    }
}

impl<'a, S: QuerySRDF + 'static> SHACLExecutor<S> for QueryExecutor<'a, S> {
    fn store(&self) -> &S {
        self.store
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn runner(&self) -> &dyn ValidatorRunner<S> {
        &self.runner
    }

    fn evaluate(
        &self,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ValidateError> {
        let component: Box<dyn SparqlConstraintComponent<S>> = context.component().into();
        Ok(component.evaluate_sparql(self, context, value_nodes, report)?)
    }
}
