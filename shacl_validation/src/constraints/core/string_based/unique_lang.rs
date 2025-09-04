use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::UniqueLang;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::Literal;
use srdf::NeighsRDF;
use srdf::Object;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::debug;

impl<S: NeighsRDF + Debug> Validator<S> for UniqueLang {
    fn validate(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        // If unique_lang is not activated, just return without any check
        if !self.unique_lang() {
            return Ok(Default::default());
        }
        let mut validation_results = Vec::new();
        // Collect langs
        // println!("Value nodes: {}", value_nodes);
        for (_focus_node, focus_nodes) in value_nodes.iter() {
            let mut langs_map: HashMap<String, Vec<S::Term>> = HashMap::new();
            for node in focus_nodes.iter() {
                if let Ok(lit) = S::term_as_literal(&node) {
                    // println!("Literal: {:?}", lit);
                    if let Some(lang) = lit.lang() {
                        // println!("Lang: {:?}", lang);
                        langs_map
                            .entry(lang.to_string())
                            .or_default()
                            .push(node.clone());
                    }
                }
            }
            for (key, nodes) in langs_map {
                if nodes.len() > 1 {
                    // If there are multiple nodes with the same language, report a violation
                    debug!(
                        "Duplicated lang: {}, nodes {:?}",
                        key,
                        nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>()
                    );
                    let component = Object::iri(component.into());
                    let message = format!(
                        "Unique lang failed for lang {} with values: {}",
                        key,
                        nodes
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    let validation_result =
                        ValidationResult::new(shape.id().clone(), component, shape.severity())
                            .with_message(message.as_str())
                            .with_path(maybe_path.clone());
                    validation_results.push(validation_result);
                }
            }
        }
        Ok(validation_results)
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for UniqueLang {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for UniqueLang {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}
