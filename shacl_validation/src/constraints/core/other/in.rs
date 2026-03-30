use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::{NativeValidator, Validator};
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF};
use shacl::ir::components::In;
use shacl::ir::{IRComponent, IRSchema, IRShape};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for In {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let check = |value_node: &S::Term| {
            let values: Vec<_> = self.values().iter().map(|node| S::object_as_term(node)).collect();
            !values.contains(value_node)
        };
        let message = format!("In constraint not satisfied. Expected one of: {:?}", self.values());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check,
            &message,
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for In {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            engine,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for In {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}
