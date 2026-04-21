use std::arch::x86_64::_store_mask8;
use std::collections::HashMap;
use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::literal::Literal;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::components::UniqueLang;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validator::engine::{Engine, SparqlEngine};
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for UniqueLang {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        // If unique_lang is not activated, just return without any check
        if !self.unique_lang() { return Ok(Default::default()) }

        let mut validation_results = Vec::new();
        let component = Object::iri(component.into());

        // Collect langs
        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            let mut langs_map: HashMap<String, Vec<S::Term>> = HashMap::new();
            for node in nodes.iter() {
                if let Ok(lit) = S::term_as_literal(node) {
                    if let Some(lang) = lit.lang() {
                        langs_map.entry(lang.to_string()).or_default().push(node.clone());
                    }
                }
            }

            for (k, v) in langs_map {
                if v.len() > 1 {
                    // If there are multiple nodes with the same language, report a violation
                    let msg = format!("Unique lang failed for lang {k} with values: {}", v.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
                    let vr = ValidationResult::new(fnode_obj.clone(), component.clone(), shape.severity())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_source(Some(shape.id().clone()));
                    validation_results.push(vr);
                }
            }
        }

        Ok(validation_results)
    }
}
