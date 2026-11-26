use crate::error::ValidationError;
use crate::ir::components::Class;
use crate::ir::{IRComponent, IRSchema, IRShape};
#[cfg(feature = "sparql")]
use crate::types::MessageMap;
use crate::validator::constraints::{NativeValidator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationOutcome;
#[cfg(feature = "sparql")]
use crate::validator::report::{Evidence, ValidationResult};
use rudof_rdf::rdf_core::term::Term;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashSet;
#[cfg(feature = "sparql")]
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
    ) -> Result<ValidationOutcome, ValidationError> {
        let term = S::object_as_term(self.class_rule());
        let class_fn = |vn: &S::Term| vn.is_literal() || !is_shacl_instance(store, vn, &term);

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
    ) -> Result<ValidationOutcome, ValidationError> {
        let class_sparql = match object_as_sparql(self.class_rule()) {
            Some(s) => s,
            None => return Ok(ValidationOutcome::new()),
        };
        let class_term: S::Term = self.class_rule().clone().into();
        let component_obj = Object::iri(component.into());
        let msg = format!("Class constraint not satisfied for class {}", self.class_rule());
        let mut outcome = ValidationOutcome::new();

        for (focus, vns) in value_nodes.iter() {
            let focus_obj = S::term_as_object(focus)?;
            for vn in vns.iter() {
                let conforms = if vn.is_literal() {
                    false
                } else if vn.is_blank_node() {
                    is_shacl_instance(store, vn, &class_term)
                } else {
                    let vn_sparql = term_as_sparql::<S>(vn).unwrap_or_default();
                    let query = formatdoc! {"
                        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                        ASK {{ {vn_sparql} rdf:type/rdfs:subClassOf* {class_sparql} }}
                    "};
                    store.query_ask(&query).map_err(ValidationError::ask_query_error::<S>)?
                };

                let value = S::term_as_object(vn).ok();
                if conforms {
                    let ev = Evidence::new(focus_obj.clone(), component_obj.clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(maybe_path.cloned())
                        .with_value(value);
                    outcome.push_evidence(ev);
                } else {
                    let vr = ValidationResult::new(focus_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_message(MessageMap::from(msg.as_str()))
                        .with_path(maybe_path.cloned())
                        .with_value(value);
                    outcome.push_violation(vr);
                }
            }
        }

        Ok(outcome)
    }
}

/// Whether `node` is a SHACL instance of `class`: whether it has
/// `rdf:type/rdfs:subClassOf*` `class`, at any depth of `rdfs:subClassOf`.
fn is_shacl_instance<S: NeighsRDF>(store: &S, node: &S::Term, class: &S::Term) -> bool {
    let rdf_type: S::IRI = RdfVocab::rdf_type().into();
    let subclass_of: S::IRI = RdfsVocab::rdfs_subclass_of_str().into();

    let mut pending: Vec<S::Term> = store
        .objects_for(node, &rdf_type)
        .unwrap_or_default()
        .into_iter()
        .collect();
    let mut seen: HashSet<S::Term> = pending.iter().cloned().collect();

    while let Some(current) = pending.pop() {
        if &current == class {
            return true;
        }
        for superclass in store.objects_for(&current, &subclass_of).unwrap_or_default() {
            if seen.insert(superclass.clone()) {
                pending.push(superclass);
            }
        }
    }
    false
}
