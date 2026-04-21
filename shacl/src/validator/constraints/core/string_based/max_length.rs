use std::fmt::Debug;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, RDFError, Rdf, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Iri, Term};
use rudof_rdf::rdf_core::term::literal::Literal;
use crate::ir::components::MaxLength;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{validate_ask_with, validate_with, ConstraintError, NativeValidator, SparqlValidator};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxLength {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_length_fn = |vn: &S::Term| {
            if vn.is_blank_node() {
                true
            } else if vn.is_iri() {
                let iri: S::IRI = match S::term_as_iri(vn) {
                    Ok(iri) => iri,
                    Err(_) => todo!(),
                };
                iri.as_str().len() > self.max_length() as usize
            } else if vn.is_literal() {
                let lit: S::Literal = match S::term_as_literal(vn) {
                    Ok(lit) => lit,
                    Err(_) => todo!(),
                };
                lit.lexical_form().len() > self.max_length() as usize
            } else { todo!() }
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_length_fn,
            &format!("MaxLength({}) not satisfied", self.max_length()),
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MaxLength {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn = |vn: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                vn, self.max_length()
            }
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("MaxLength({}) not satisfied", self.max_length()),
            maybe_path
        )
    }
}