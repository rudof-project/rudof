use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::LanguageIn;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Sparql;
use srdf::Rdf;
use srdf::Query;
use std::fmt::Debug;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: Rdf + Debug> Validator<S> for LanguageIn<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let language_in = |value_node: &S::Term| {
            if let Some(lang) = S::term_as_literal(value_node) {
                if self.langs().contains(&lang) {
                    return false;
                }
            }
            true
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            language_in,
        )
    }
}

impl<S: Query + Debug + 'static> NativeValidator<S> for LanguageIn<S> {
    fn validate_native<'a>(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, NativeEngine, value_nodes)
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for LanguageIn<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, SparqlEngine, value_nodes)
    }
}
