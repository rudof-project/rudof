use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::MinLength;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::Iri as _;
use srdf::Literal as _;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use srdf::Term;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MinLength {
    fn validate_native<'a>(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_length = |value_node: &S::Term| {
            if value_node.is_blank_node() {
                true
            } else if value_node.is_iri() {
                let iri: S::IRI = match S::term_as_iri(value_node) {
                    Ok(iri) => iri,
                    Err(_) => todo!(),
                };
                iri.as_str().len() < self.min_length() as usize
            } else if value_node.is_literal() {
                let literal: S::Literal = match S::term_as_literal(value_node) {
                    Ok(literal) => literal,
                    Err(_) => todo!(),
                };
                literal.lexical_form().len() < self.min_length() as usize
            } else {
                true
            }
        };

        let message = format!("MinLength({}) not satisfied", self.min_length());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            min_length,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MinLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_length_value = self.min_length();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                value_node, min_length_value
            }
        };

        let message = format!("MinLength({min_length_value}) not satisfied");
        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query,
            &message,
            maybe_path,
        )
    }
}
