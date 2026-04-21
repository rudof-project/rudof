use std::fmt::Debug;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Term;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use crate::ir::components::Class;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{validate_ask_with, validate_with, ConstraintError, NativeValidator, SparqlValidator};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + 'static> NativeValidator<S> for Class {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let class_fn = |vn: &S::Term| {
            if vn.is_literal() { return true; }
            let term = S::object_as_term(self.class_rule());

            !store
                .objects_for(vn, &RdfVocab::rdf_type().into())
                .unwrap_or_default()
                .iter()
                .any(|ctype| {
                    ctype == &term
                    || store
                        .objects_for(ctype, &RdfsVocab::rdfs_subclass_of_str().into())
                        .unwrap_or_default()
                        .contains(&term)
                })
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            class_fn,
            &format!("Class constraint not satisfied for class {}", self.class_rule()),
            maybe_path
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Class {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn = |vn: &S::Term| {
            formatdoc! {"
                PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
            ", vn, self.class_rule()
            }
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("Class constraint not satisfied for class {}", self.class_rule()),
            maybe_path
        )
    }
}