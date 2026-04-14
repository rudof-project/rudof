use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::literal::{Lang, Literal};
use sparql_service::sd_available_graphs;
use crate::ir::components::LanguageIn;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::iteration::ValueNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::validate_with;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for LanguageIn {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |vn| {
                if let Ok(lit) = S::term_as_literal(vn) {
                    return match lit.lang() {
                        None => true,
                        Some(lang) => !self.langs().contains(&lang),
                    }
                }
                true
            },
            &format!("LanguageIn constraint not satisfied. Expected one of {}", self.langs().iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", ")),
            maybe_path,
        )
    }
}