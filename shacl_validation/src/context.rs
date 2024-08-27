use shacl_ast::component::Component;
use shacl_ast::shape::Shape;
use shacl_ast::Schema;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::store::Store;

pub struct ValidationContext<'a, S: SRDFBasic> {
    store: &'a dyn Store<S>,
    schema: &'a Schema,
    runner: &'a dyn ValidatorRunner<S>,
}

impl<'a, S: SRDF + 'static> ValidationContext<'a, S> {
    pub(crate) fn new_default(store: &'a dyn Store<S>, schema: &'a Schema) -> Self {
        Self {
            store,
            schema,
            runner: &DefaultValidatorRunner,
        }
    }
}

impl<'a, S: QuerySRDF + 'static> ValidationContext<'a, S> {
    pub(crate) fn new_sparql(store: &'a dyn Store<S>, schema: &'a Schema) -> Self {
        Self {
            store,
            schema,
            runner: &QueryValidatorRunner,
        }
    }
}

impl<'a, S: SRDFBasic> ValidationContext<'a, S> {
    pub(crate) fn store(&self) -> &S {
        self.store.store()
    }

    pub(crate) fn schema(&self) -> &Schema {
        self.schema
    }

    pub(crate) fn runner(&self) -> &dyn ValidatorRunner<S> {
        self.runner
    }
}

pub struct EvaluationContext<'a> {
    component: &'a Component,
    shape: &'a Shape,
}

impl<'a> EvaluationContext<'a> {
    pub fn new(component: &'a Component, shape: &'a Shape) -> Self {
        Self { component, shape }
    }

    pub fn component(&self) -> &Component {
        self.component
    }

    pub(crate) fn shape<S: SRDFBasic>(&self) -> S::Term {
        match self.shape {
            Shape::NodeShape(ns) => S::object_as_term(&ns.id()),
            Shape::PropertyShape(ps) => S::object_as_term(ps.id()),
        }
    }

    pub fn source_constraint_component<S: SRDFBasic>(&self) -> S::Term {
        S::iri_s2term(&self.component.to_owned().into())
    }

    pub(crate) fn result_severity<S: SRDFBasic>(&self) -> Option<S::Term> {
        let severity = match self.shape {
            Shape::NodeShape(ns) => ns.severity(),
            Shape::PropertyShape(ps) => ps.severity(),
        };
        severity.map(|severity| S::iri_s2term(&severity.to_owned().into()))
    }
}
