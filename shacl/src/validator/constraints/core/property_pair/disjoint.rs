use crate::ir::components::Disjoint;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, NativeValidator, SparqlValidator, validate_with_focus};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Triple;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Disjoint {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let check_fn = |f: &S::Term, vn: &S::Term| {
            let subject = S::term_as_subject(f).unwrap();
            let iri: S::IRI = self.iri().clone().into();
            let triples_to_compare = match store.triples_with_subject_predicate(&subject, &iri) {
                Ok(iter) => iter,
                Err(_) => return true,
            };

            for triple in triples_to_compare {
                let value1 = S::term_as_object(vn).unwrap();
                let value2 = S::term_as_object(triple.obj()).unwrap();

                if value1 == value2 {
                    return true;
                }
            }
            false
        };

        validate_with_focus(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check_fn,
            &format!("Disjoint failed. Property {}", self.iri()),
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Disjoint {
    fn validate_sparql(
        &self,
        _: &IRComponent,
        _: &IRShape,
        _: &S,
        _: &ValueNodes<S>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented {
            err: "Disjoint not implemented".to_string(),
        })
    }
}
