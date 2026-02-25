use crate::constraints::get_shape_from_idx;
use crate::focus_nodes::FocusNodes;
use crate::shacl_engine::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use iri_s::IriS;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{
    NeighsRDF, Rdf, SHACLPath,
    term::{Object, Triple},
};
use shacl_ir::compiled::property_shape::PropertyShapeIR;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::reifier_info::ReifierInfo;
use shacl_ir::{compiled::node_shape::NodeShapeIR, schema_ir::SchemaIR};
use std::{collections::HashSet, fmt::Debug};
use tracing::trace;

/// Validate RDF data using SHACL
pub trait Validate<S: Rdf> {
    fn validate(
        &self,
        store: &S,
        runner: &mut dyn Engine<S>,
        targets: Option<&FocusNodes<S>>,
        source_shape: Option<&ShapeIR>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, Box<ValidateError>>;
}

impl<S: NeighsRDF + Debug> Validate<S> for ShapeIR {
    fn validate(
        &self,
        store: &S,
        runner: &mut dyn Engine<S>,
        targets: Option<&FocusNodes<S>>,
        source_shape: Option<&ShapeIR>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, Box<ValidateError>> {
        trace!("Shape.validate with shape {}", self.id());

        // Skip validation if it is deactivated
        if self.deactivated() {
            return Ok(Vec::default());
        }

        // Get focus nodes
        let focus_nodes = match targets {
            Some(targets) => targets.to_owned(),
            None => self.focus_nodes(store, runner),
        };
        trace!("Focus nodes for shape {}: {focus_nodes}", self.id());

        // ValueNodes = set of nodes that are going to be used during validation.
        // This set of nodes is obtained from the set of focus nodes
        let value_nodes = self.value_nodes(store, &focus_nodes, runner)?;
        trace!("Value nodes for shape {}: {value_nodes}", self.id());

        let components = self.components();

        // 3. Check each of the components
        let mut component_validation_results = Vec::new();
        for component in components.iter() {
            let results = runner.evaluate(
                store,
                self,
                component,
                &value_nodes,
                source_shape,
                self.path(),
                shapes_graph,
            )?;
            trace!(
                "Results for component {component}: with value nodes {value_nodes}\n{}\nend results",
                results.iter().map(|r| r.to_string()).collect::<Vec<_>>().join("\n")
            );
            component_validation_results.extend(results);
        }

        // After validating the constraints that are defined in the current
        //    Shape, it is important to also perform the validation over those
        //    nested PropertyShapes.
        //  The validation needs to occur over the focus_nodes
        //    that have been computed for the current shape
        let mut property_shapes_validation_results = Vec::new();
        for prop_shape in self.property_shapes().iter() {
            let shape = shapes_graph.get_shape_from_idx(prop_shape).unwrap_or_else(|| {
                panic!(
                    "Internal error: Property shape for idx: {} not found in schema",
                    prop_shape
                )
            });
            let results = shape.validate(store, runner, Some(&focus_nodes), Some(self), shapes_graph)?;
            property_shapes_validation_results.extend(results);
        }

        // Check if there are extra properties but the shape is closed
        let mut closed_validation_results = Vec::new();
        if self.closed() {
            for focus_node in focus_nodes.iter() {
                let allowed_properties: HashSet<IriS> = self.allowed_properties();

                let all_properties: HashSet<IriS> = match S::term_as_subject(focus_node) {
                    Ok(subj) => {
                        let ts = store
                            .triples_with_subject(&subj)
                            .map_err(|e| ValidateError::TriplesWithSubject {
                                subject: format!("{focus_node:?}"),
                                error: e.to_string(),
                            })?;
                        Ok::<HashSet<IriS>, ValidateError>(ts.map(|t| t.pred().clone().into()).collect())
                    },
                    Err(_) => Ok::<HashSet<IriS>, ValidateError>(HashSet::new()),
                }?;

                let invalid_properties: Vec<IriS> = all_properties
                    .difference(&allowed_properties.iter().cloned().collect())
                    .cloned()
                    .collect();

                for property in invalid_properties {
                    let vr_single = ValidationResult::new(
                        self.id().clone(),
                        Object::iri(ShaclVocab::sh_closed_constraint_component().clone()),
                        self.severity(),
                    )
                    .with_path(Some(SHACLPath::iri(property)));
                    closed_validation_results.push(vr_single);
                }
            }
        }

        let reification_results = if let Some(reifier_info) = self.reifier_info() {
            validate_reifiers(
                self,
                store,
                runner,
                source_shape,
                &reifier_info,
                &focus_nodes,
                shapes_graph,
            )?
        } else {
            Vec::new()
        };

        // Collect all validation results
        let validation_results = component_validation_results
            .into_iter()
            .chain(property_shapes_validation_results)
            .chain(closed_validation_results)
            .chain(reification_results)
            .collect();

        Ok(validation_results)
    }
}

fn validate_reifiers<S>(
    shape: &ShapeIR,
    store: &S,
    runner: &mut dyn Engine<S>,
    source_shape: Option<&ShapeIR>,
    reifier_info: &ReifierInfo,
    focus_nodes: &FocusNodes<S>,
    shapes_graph: &SchemaIR,
) -> Result<Vec<ValidationResult>, Box<ValidateError>>
where
    S: NeighsRDF + Debug,
{
    let mut results = Vec::new();
    for focus_node in focus_nodes.iter() {
        for reifier_shape in reifier_info.reifier_shape() {
            let pred = reifier_info.predicate();
            let pred_iri: S::IRI = pred.clone().into();
            let subject = S::term_as_subject(focus_node).map_err(|_| ValidateError::TriplesWithSubject {
                subject: format!("{focus_node:?}"),
                error: "Cannot convert to subject".to_string(),
            })?;
            let triples = store.triples_with_subject_predicate(&subject, &pred_iri).map_err(|e| {
                ValidateError::TriplesWithSubjectPredicate {
                    subject: format!("{focus_node}"),
                    predicate: pred.to_string(),
                    error: e.to_string(),
                }
            })?;
            for triple in triples {
                let reifier_subjects =
                    store
                        .reifiers_of_triple(&triple)
                        .map_err(|e| ValidateError::ReifiersOfTriple {
                            triple: format!("{triple:?}"),
                            error: e.to_string(),
                        })?;
                let reifier_subjects = reifier_subjects.collect::<Vec<_>>();
                if reifier_subjects.is_empty() && reifier_info.reification_required() {
                    let vr_single = ValidationResult::new(
                        shape.id().clone(),
                        Object::iri(ShaclVocab::sh_reifier_shape_constraint_component().clone()),
                        shape.severity(),
                    )
                    .with_message(&format!(
                        "Reification required but no reifier found for triple {} with predicate {}",
                        triple, pred
                    ))
                    .with_path(Some(SHACLPath::iri(pred.clone())))
                    .with_source(source_shape.map(|s| s.id()).cloned());
                    results.push(vr_single);
                    continue;
                }
                let reifier_nodes = reifier_subjects
                    .iter()
                    .map(|subj| S::subject_as_term(subj))
                    .collect::<HashSet<_>>();
                let reifier_shape =
                    get_shape_from_idx(shapes_graph, reifier_shape).map_err(|e| ValidateError::ShapeNotFound {
                        shape_idx: *reifier_shape,
                        error: e.to_string(),
                    })?;
                let vr_iter = reifier_shape.validate(
                    store,
                    runner,
                    Some(&FocusNodes::from_iter(reifier_nodes)),
                    Some(shape),
                    shapes_graph,
                )?;
                results.extend(vr_iter.into_iter());
            }
        }
    }
    Ok(results)
}

pub trait FocusNodesOps<S: Rdf> {
    fn focus_nodes(&self, store: &S, runner: &dyn Engine<S>) -> FocusNodes<S>;
}

impl<S: NeighsRDF> FocusNodesOps<S> for ShapeIR {
    fn focus_nodes(&self, store: &S, runner: &dyn Engine<S>) -> FocusNodes<S> {
        runner
            .focus_nodes(store, self.targets())
            .expect("Failed to retrieve focus nodes")
    }
}

pub trait ValueNodesOps<S: Rdf> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> Result<ValueNodes<S>, Box<ValidateError>>;
}

impl<S: NeighsRDF> ValueNodesOps<S> for ShapeIR {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> Result<ValueNodes<S>, Box<ValidateError>> {
        match self {
            ShapeIR::NodeShape(ns) => ns.value_nodes(store, focus_nodes, runner),
            ShapeIR::PropertyShape(ps) => ps.value_nodes(store, focus_nodes, runner),
        }
    }
}

impl<S: Rdf> ValueNodesOps<S> for NodeShapeIR {
    fn value_nodes(
        &self,
        _: &S,
        focus_nodes: &FocusNodes<S>,
        _: &dyn Engine<S>,
    ) -> Result<ValueNodes<S>, Box<ValidateError>> {
        let value_nodes = focus_nodes.iter().map(|focus_node| {
            (
                focus_node.clone(),
                FocusNodes::from_iter(std::iter::once(focus_node.clone())),
            )
        });
        Ok(ValueNodes::new(value_nodes))
    }
}

impl<S: NeighsRDF> ValueNodesOps<S> for PropertyShapeIR {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> Result<ValueNodes<S>, Box<ValidateError>> {
        let value_nodes = focus_nodes.iter().filter_map(|focus_node| {
            match runner.path(store, self, focus_node) {
                Ok(ts) => Some((focus_node.clone(), ts)),
                Err(e) => {
                    trace!(
                        "Error calculating nodes for focus node {} with path {}: {}",
                        focus_node,
                        self.path(),
                        e
                    );
                    // We are currently ust ignoring this case
                    // TODO: Should we add a violation for this case?
                    None
                },
            }
        });
        Ok(ValueNodes::new(value_nodes))
    }
}
