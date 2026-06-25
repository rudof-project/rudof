use crate::error::ValidationError;
use crate::ir::components::Disjoint;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{BasicSparqlValidator, NativeValidator, validate_with_focus};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use indoc::formatdoc;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Triple};
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
    ) -> Result<Vec<ValidationResult>, ValidationError> {
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
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for Disjoint {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;

            for vn in nodes.iter() {
                let query = formatdoc! {"
                    ASK {{ {} <{}> {} }}
                ", fnode, self.iri(), vn};

                let ask = store
                    .query_ask(&query)
                    .map_err(ValidationError::ask_query_error::<S>)?;

                if ask {
                    let value = S::term_as_object(vn).ok();
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(maybe_path.cloned())
                        .with_value(value);
                    results.push(vr);
                }
            }
        }

        Ok(results)
    }
}
