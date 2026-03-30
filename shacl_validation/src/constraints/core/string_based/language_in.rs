use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::literal::Literal};
use shacl::ir::components::LanguageIn;
use shacl::ir::{IRComponent, IRSchema, IRShape};
use std::fmt::Debug;

use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for LanguageIn {
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
        let language_in = |value_node: &S::Term| {
            if let Ok(literal) = S::term_as_literal(value_node) {
                return match literal.lang() {
                    Some(lang) => !self.langs().contains(&lang),
                    None => true,
                };
            }
            true
        };

        let message = format!(
            "LanguageIn constraint not satisfied. Expected one of: {}",
            self.langs()
                .iter()
                .map(|l| l.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        );
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            language_in,
            &message,
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for LanguageIn {
    fn validate_native<'a>(
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for LanguageIn {
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
