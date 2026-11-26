use crate::error::ValidationError;
use crate::ir::components::UniqueLang;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::Validator;
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::Literal;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashMap;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for UniqueLang {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        // If unique_lang is not activated, just return without any check
        if !self.unique_lang() {
            return Ok(Default::default());
        }

        let mut outcome = ValidationOutcome::new();
        let component = Object::iri(component.into());

        // Collect langs
        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            let mut langs_map: HashMap<String, Vec<S::Term>> = HashMap::new();
            for node in nodes.iter() {
                if let Ok(lit) = S::term_as_literal(node)
                    && let Some(lang) = lit.lang()
                {
                    langs_map.entry(lang.to_string()).or_default().push(node.clone());
                }
            }

            let mut had_duplicate = false;
            for (k, v) in langs_map {
                if v.len() > 1 {
                    // If there are multiple nodes with the same language, report a violation
                    had_duplicate = true;
                    let msg = format!(
                        "Unique lang failed for lang {k} with values: {}",
                        v.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component.clone(), shape.severity().clone())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_source(Some(shape.id().clone()));
                    outcome.push_violation(vr);
                }
            }

            if !had_duplicate {
                let ev = Evidence::new(fnode_obj, component.clone())
                    .with_path(maybe_path.cloned())
                    .with_source(Some(shape.id().clone()));
                outcome.push_evidence(ev);
            }
        }

        Ok(outcome)
    }
}
