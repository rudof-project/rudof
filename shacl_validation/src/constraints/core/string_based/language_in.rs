use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::LanguageIn;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::Literal;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::lang::Lang;
use std::fmt::Debug;

use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::engine::Engine;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: Rdf + Debug> Validator<S> for LanguageIn {
    fn validate(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let language_in = |value_node: &S::Term| {
            if let Ok(literal) = S::term_as_literal(value_node) {
                return match literal.lang() {
                    Some(lang) => !self.langs().contains(&Lang::new_unchecked(lang)),
                    None => true,
                };
            }
            true
        };

        let message = format!(
            "LanguageIn constraint not satisfied. Expected one of: {:?}",
            self.langs()
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
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for LanguageIn {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}
