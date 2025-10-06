use crate::focus_nodes::FocusNodes;
use crate::shacl_engine::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use iri_s::{IriS, iri};
use shacl_ir::compiled::node_shape::NodeShapeIR;
use shacl_ir::compiled::property_shape::PropertyShapeIR;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::{NeighsRDF, Object, Rdf, SHACLPath, Triple};
use std::{collections::HashSet, fmt::Debug};
use tracing::debug;

/// Validate RDF data using SHACL
pub trait Validate<S: Rdf> {
    fn validate(
        &self,
        store: &S,
        runner: &dyn Engine<S>,
        targets: Option<&FocusNodes<S>>,
        source_shape: Option<&ShapeIR>,
    ) -> Result<Vec<ValidationResult>, ValidateError>;
}

impl<S: NeighsRDF + Debug> Validate<S> for ShapeIR {
    fn validate(
        &self,
        store: &S,
        runner: &dyn Engine<S>,
        targets: Option<&FocusNodes<S>>,
        source_shape: Option<&ShapeIR>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        debug!("Shape.validate with shape {}", self.id());

        // Skip validation if it is deactivated
        if self.deactivated() {
            return Ok(Vec::default());
        }

        // Get focus nodes
        let focus_nodes = match targets {
            Some(targets) => targets.to_owned(),
            None => self.focus_nodes(store, runner),
        };
        debug!("Focus nodes for shape {}: {focus_nodes}", self.id());

        // ValueNodes = set of nodes that are going to be used during validation.
        // This set of nodes is obtained from the set of focus nodes
        let value_nodes = self.value_nodes(store, &focus_nodes, runner)?;
        debug!("Value nodes for shape {}: {value_nodes}", self.id());

        // 3. Check each of the components
        let component_validation_results = self.components().iter().flat_map(move |component| {
            runner.evaluate(
                store,
                self,
                component,
                &value_nodes,
                source_shape,
                self.path(),
            )
        });

        // After validating the constraints that are defined in the current
        //    Shape, it is important to also perform the validation over those
        //    nested PropertyShapes. The validation needs to occur over the focus_nodes
        //    that have been computed for the current shape
        let property_shapes_validation_results =
            self.property_shapes().iter().flat_map(|prop_shape| {
                prop_shape.validate(store, runner, Some(&focus_nodes), Some(self))
            });

        // Check if there are extra properties but the shape is closed
        let mut closed_validation_results = Vec::new();
        if self.closed() {
            for focus_node in focus_nodes.iter() {
                let allowed_properties: HashSet<IriS> = self.allowed_properties();

                let all_properties: HashSet<IriS> = match S::term_as_subject(focus_node) {
                    Ok(subj) => {
                        let ts = store.triples_with_subject(subj).map_err(|e| {
                            ValidateError::TriplesWithSubject {
                                subject: format!("{focus_node:?}"),
                                error: e.to_string(),
                            }
                        })?;
                        Ok::<HashSet<IriS>, ValidateError>(
                            ts.map(|t| t.pred().clone().into()).collect(),
                        )
                    }
                    Err(_) => Ok::<HashSet<IriS>, ValidateError>(HashSet::new()),
                }?;

                let invalid_properties: Vec<IriS> = all_properties
                    .difference(&allowed_properties.iter().cloned().collect())
                    .cloned()
                    .collect();

                for property in invalid_properties {
                    let vr_single = ValidationResult::new(
                        self.id().clone(),
                        closed_constraint_component(),
                        self.severity(),
                    )
                    .with_path(Some(SHACLPath::iri(property)));
                    closed_validation_results.push(vr_single);
                }
            }
        }

        // Collect all validation results
        let validation_results = component_validation_results
            .chain(property_shapes_validation_results)
            .chain(vec![closed_validation_results])
            .flatten()
            .collect();

        Ok(validation_results)
    }
}

fn closed_constraint_component() -> Object {
    Object::Iri(iri!("http://www.w3.org/ns/shacl#ClosedConstraintComponent"))
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
    ) -> Result<ValueNodes<S>, ValidateError>;
}

impl<S: NeighsRDF> ValueNodesOps<S> for ShapeIR {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &FocusNodes<S>,
        runner: &dyn Engine<S>,
    ) -> Result<ValueNodes<S>, ValidateError> {
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
    ) -> Result<ValueNodes<S>, ValidateError> {
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
    ) -> Result<ValueNodes<S>, ValidateError> {
        let value_nodes = focus_nodes.iter().filter_map(|focus_node| {
            match runner.path(store, self, focus_node) {
                Ok(ts) => Some((focus_node.clone(), ts)),
                Err(e) => {
                    debug!(
                        "Error calculating nodes for focus node {} with path {}: {}",
                        focus_node,
                        self.path(),
                        e
                    );
                    // We are currently ust ignoring this case
                    // TODO: Should we add a violation for this case?
                    None
                }
            }
        });
        Ok(ValueNodes::new(value_nodes))
    }
}
