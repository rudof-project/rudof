use crate::error::ValidationError;
use crate::ir::components::Class;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{NativeValidator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Term;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

#[cfg(feature = "sparql")]
use crate::validator::constraints::{BasicSparqlValidator, object_as_sparql, term_as_sparql};
#[cfg(feature = "sparql")]
use indoc::formatdoc;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::term::Object;

impl<S: NeighsRDF + 'static> NativeValidator<S> for Class {
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
        let class_fn = |vn: &S::Term| {
            if vn.is_literal() {
                return true;
            }
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
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for Class {
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
        let class_sparql = match object_as_sparql(self.class_rule()) {
            Some(s) => s,
            None => return Ok(Vec::new()),
        };
        let class_term: S::Term = self.class_rule().clone().into();
        let component_obj = Object::iri(component.into());
        let msg = format!("Class constraint not satisfied for class {}", self.class_rule());
        let mut results = Vec::new();

        for (focus, vns) in value_nodes.iter() {
            let focus_obj = S::term_as_object(focus)?;
            for vn in vns.iter() {
                let conforms = if vn.is_literal() {
                    false
                } else if vn.is_blank_node() {
                    blank_is_instance::<S>(store, vn, &class_term)?
                } else {
                    let vn_sparql = term_as_sparql::<S>(vn).unwrap_or_default();
                    let query = formatdoc! {"
                        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                        ASK {{ {vn_sparql} rdf:type/rdfs:subClassOf* {class_sparql} }}
                    "};
                    store.query_ask(&query).map_err(ValidationError::ask_query_error::<S>)?
                };

                if !conforms {
                    let value = S::term_as_object(vn).ok();
                    let vr = ValidationResult::new(focus_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_message(MessageMap::from(msg.as_str()))
                        .with_path(maybe_path.cloned())
                        .with_value(value);
                    results.push(vr);
                }
            }
        }

        Ok(results)
    }
}

#[cfg(feature = "sparql")]
fn blank_is_instance<S: NeighsRDF>(store: &S, vn: &S::Term, class_term: &S::Term) -> Result<bool, ValidationError> {
    let types = store.objects_for(vn, &RdfVocab::rdf_type().into()).unwrap_or_default();
    Ok(types.iter().any(|ctype| {
        ctype == class_term
            || store
                .objects_for(ctype, &RdfsVocab::rdfs_subclass_of_str().into())
                .unwrap_or_default()
                .contains(class_term)
    }))
}
