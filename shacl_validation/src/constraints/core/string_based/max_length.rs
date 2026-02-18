use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{
    NeighsRDF, SHACLPath,
    query::QueryRDF,
    term::{Iri as _, Term, literal::Literal as _},
};
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::MaxLength;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxLength {
    fn validate_native<'a>(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        _: &S,
        _engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_length = |value_node: &S::Term| {
            if value_node.is_blank_node() {
                true
            } else if value_node.is_iri() {
                let iri: S::IRI = match S::term_as_iri(value_node) {
                    Ok(iri) => iri,
                    Err(_) => todo!(),
                };
                iri.as_str().len() > self.max_length() as usize
            } else if value_node.is_literal() {
                let literal: S::Literal = match S::term_as_literal(value_node) {
                    Ok(literal) => literal,
                    Err(_) => todo!(),
                };
                literal.lexical_form().len() > self.max_length() as usize
            } else {
                todo!()
            }
        };

        let message = format!("MaxLength({}) not satisfied", self.max_length());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_length,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MaxLength {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_length_value = self.max_length();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                value_node, max_length_value
            }
        };

        let message = format!("MaxLength({}) not satisfied", self.max_length());
        validate_ask_with(component, shape, store, value_nodes, query, &message, maybe_path)
    }
}
