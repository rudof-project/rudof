use crate::Reason;
use crate::ValidatorConfig;
use crate::atom;
use crate::engine::Engine;
use crate::validator_error::*;
use colored::Color;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_core::{NeighsRDF, query::QueryRDF};
use serde_json::Value;
use shex_ast::Node;
use shex_ast::ShapeExprLabel;
use shex_ast::ShapeLabelIdx;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::shapemap::ResultShapeMap;
use shex_ast::shapemap::ValidationStatus;
use shex_ast::shapemap::query_shape_map::QueryShapeMap;
use std::str::FromStr;
use tracing::trace;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;

#[derive(Debug)]
pub struct Validator {
    schema: SchemaIR,
    config: ValidatorConfig,
}

impl Validator {
    /// Creates a validator, checking for negation cycles per the config.
    ///
    /// Equivalent to [`Validator::with_neg_cycle_check`] with
    /// `check_neg_cycles = true`. Callers that have already verified the
    /// dependency graph (e.g. loaders that trust a precompiled cache
    /// header) should call [`Validator::with_neg_cycle_check`] with
    /// `false` to skip the redundant Tarjan SCC pass.
    pub fn new(schema: &SchemaIR, config: &ValidatorConfig) -> Result<Validator> {
        Self::with_neg_cycle_check(schema, config, true)
    }

    /// Creates a validator, optionally skipping the negation-cycle check.
    ///
    /// When `check_neg_cycles` is `false`, the constructor trusts the
    /// caller and does not run Tarjan SCC on the dependency graph.
    pub fn with_neg_cycle_check(
        schema: &SchemaIR,
        config: &ValidatorConfig,
        check_neg_cycles: bool,
    ) -> Result<Validator> {
        if check_neg_cycles && config.check_negation_requirement && schema.has_neg_cycle() {
            trace!("Checking negation cycles...");
            let neg_cycles = schema.neg_cycles();
            trace!("Negation cycles: {neg_cycles:?}");
            let mut neg_cycles_displayed = Vec::new();
            for cycle in neg_cycles.iter() {
                let mut cycle_displayed = Vec::new();
                for (source, target, shapes) in cycle.iter() {
                    let source_str = if let Some(label) = schema.shape_label_from_idx(source) {
                        schema.show_label(label)
                    } else {
                        format!("internal_{source}")
                    };
                    let target_str = if let Some(label) = schema.shape_label_from_idx(target) {
                        schema.show_label(label)
                    } else {
                        format!("internal_{target}")
                    };
                    let mut shapes_str = Vec::new();
                    for shape in shapes.iter() {
                        let shape_str = if let Some(label) = schema.shape_label_from_idx(shape) {
                            schema.show_label(label)
                        } else {
                            format!("internal_{shape}")
                        };
                        shapes_str.push(shape_str);
                    }
                    cycle_displayed.push((source_str, target_str, shapes_str));
                }
                neg_cycles_displayed.push(cycle_displayed);
            }
            return Err(ValidatorError::NegCycleError {
                neg_cycles: neg_cycles_displayed,
            });
        }
        Ok(Validator {
            schema: schema.clone(),
            config: config.clone(),
        })
    }

    pub fn schema(&self) -> &SchemaIR {
        &self.schema
    }

    /// The config to hand `Engine::new`: same as `self.config`, except that
    /// when `show_intermediate_results` is on and no observer was installed
    /// explicitly, a [`ConsoleTypingObserver`] is added, bound to the
    /// prefixmaps available for *this* call — `schema`'s own prefixmap for
    /// shape labels, and `maybe_nodes_prefixmap` (only known per-call, not
    /// at construction time) for nodes.
    fn engine_config(&self, schema: &SchemaIR, maybe_nodes_prefixmap: &Option<PrefixMap>) -> ValidatorConfig {
        if self.config.show_intermediate_results() && self.config.typing_observer().is_none() {
            let nodes_prefixmap = maybe_nodes_prefixmap.clone().unwrap_or_default();
            let conformant_color = Color::from_str(self.config.conformant_color()).unwrap_or(Color::Green);
            let non_conformant_color = Color::from_str(self.config.non_conformant_color()).unwrap_or(Color::Red);
            let observer = crate::typing::ConsoleTypingObserver::new(
                schema,
                nodes_prefixmap,
                conformant_color,
                non_conformant_color,
            );
            self.config.clone().with_typing_observer(std::sync::Arc::new(observer))
        } else {
            self.config.clone()
        }
    }

    /// validate a node against a shape label
    pub fn validate_node_shape<S>(
        &mut self,
        node: &Node,
        shape: &ShapeLabel,
        rdf: &S,
        schema: &SchemaIR,
        maybe_nodes_prefixmap: &Option<PrefixMap>,
    ) -> Result<ResultShapeMap>
    where
        S: NeighsRDF + QueryRDF,
    {
        let mut engine = Engine::new(&self.engine_config(schema, maybe_nodes_prefixmap));
        let shape_expr_label: ShapeExprLabel = shape.into();
        let idx = self.get_shape_expr_label(&shape_expr_label, schema)?;
        engine.add_pending(node.clone(), idx);
        engine.validate_pending(rdf, schema)?;
        let result = self.result_map(&mut engine, maybe_nodes_prefixmap)?;
        Ok(result)
    }

    fn get_shape_expr_label(&self, label: &ShapeExprLabel, schema: &SchemaIR) -> Result<ShapeLabelIdx> {
        schema
            .find_ref(label)
            .map_err(|error| ValidatorError::ShapeLabelNotFoundError {
                shape_label: label.clone(),
                error: format!("{error}"),
            })
    }

    pub fn validate_shapemap<S>(
        &self,
        shapemap: &QueryShapeMap,
        rdf: &S,
        schema: &SchemaIR,
        maybe_nodes_prefixmap: &Option<PrefixMap>,
    ) -> Result<ResultShapeMap>
    where
        S: NeighsRDF + QueryRDF,
    {
        let mut engine = Engine::new(&self.engine_config(schema, maybe_nodes_prefixmap));

        // Fill the engine's pending atoms with the node-shape pairs from the QueryShapeMap,
        // converting shape labels to indices and nodes to objects as needed.
        // Collect any failures for node-shape pairs that could not be processed due to missing shape labels or node conversion errors,
        // so that they can be reported in the final ResultShapeMap.
        let failures = self.fill_pending(&mut engine, shapemap, rdf, schema)?;

        // Validate the pending atoms in the engine, which will process the valid node-shape pairs
        engine.validate_pending(rdf, schema)?;

        let mut result = self.result_map(&mut engine, maybe_nodes_prefixmap)?;
        for (node, shape_label, error_msg) in failures {
            let status = ValidationStatus::non_conformant(error_msg, Value::Null);
            let node_str = node.to_string();
            let label_str = shape_label.to_string();
            result
                .add_result(node, shape_label, status)
                .map_err(|e| ValidatorError::AddingNonConformantError {
                    node: node_str,
                    label: label_str,
                    error: format!("{e}"),
                })?;
        }
        Ok(result)
    }

    /// Fill the engine's pending atoms with the node-shape pairs from the QueryShapeMap,
    /// converting shape labels to indices and nodes to objects as needed.
    ///
    /// Returns a list of failures for any node-shape pairs that could not be processed
    /// due to missing shape labels or node conversion errors, so that they can be reported
    /// in the final ResultShapeMap.
    fn fill_pending<S>(
        &self,
        engine: &mut Engine,
        shapemap: &QueryShapeMap,
        rdf: &S,
        schema: &SchemaIR,
    ) -> Result<Vec<(Node, ShapeLabel, String)>>
    where
        S: QueryRDF,
    {
        let pairs = shapemap
            .node_shapes(rdf)
            .map_err(|e| ValidatorError::ShapeMapError { error: e.to_string() })?;
        let mut failures = Vec::new();
        for (node, label) in pairs.iter() {
            match self.get_shape_expr_label(label, schema) {
                Err(e) => {
                    match ShapeLabel::from_shape_expr_label(label, &schema.prefixmap()) {
                        Ok(shape_label) => match S::term_as_object(node) {
                            Ok(obj_node) => {
                                failures.push((Node::new(obj_node), shape_label, e.to_string()));
                            },
                            Err(node_err) => {
                                trace!(
                                    "fill_pending: Could not convert node {} while handling missing label error: {}",
                                    node, node_err
                                );
                                // TODO: Should we push a failure for this case as well?
                                // It would be a bit redundant with the error message we already have, but it would allow us to report the node in the result map as well.
                            },
                        },
                        Err(label_err) => {
                            trace!(
                                "fill_pending: Could not convert shape label {} while handling missing label error: {}",
                                label, label_err
                            );
                            // TODO: Should we push a failure for this case as well?
                            // It would be a bit redundant with the error message we already have,
                            // but it would allow us to report the label in the result map as well.
                        },
                    }
                },
                Ok(idx) => {
                    let node = S::term_as_object(node).map_err(|e| ValidatorError::FillingShapeMapNodes {
                        node: node.to_string(),
                        error: e.to_string(),
                    })?;
                    engine.add_pending(Node::new(node), idx);
                },
            }
        }
        Ok(failures)
    }

    fn get_shape_label(&self, idx: &ShapeLabelIdx) -> Result<&ShapeLabel> {
        let info = self.schema.find_shape_idx(idx).unwrap();
        match info.label() {
            Some(label) => Ok(label),
            None => Err(ValidatorError::NotFoundShapeLabelWithIndex { idx: *idx }),
        }
    }

    /// Build a ResultShapeMap from the engine's checked and pending atoms, using the provided nodes prefix map and
    /// the schema's prefix map for shapes.
    pub fn result_map(&self, engine: &mut Engine, maybe_nodes_prefixmap: &Option<PrefixMap>) -> Result<ResultShapeMap> {
        let nodes_prefixmap = match maybe_nodes_prefixmap {
            Some(pm) => pm.clone(),
            None => PrefixMap::default(),
        }
        // Predicates shown alongside nodes in errors/reasons come from the schema
        // (e.g. `<#name>` under a `BASE`), so relativize them the same way shape
        // labels are relativized via `self.schema.prefixmap()`.
        .with_base(self.schema.base().cloned());
        let mut result = ResultShapeMap::new()
            .with_nodes_prefixmap(&nodes_prefixmap)
            .with_shapes_prefixmap(&self.schema.prefixmap());
        for atom in &engine.checked() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            match atom {
                Atom::Pos(positive_atom) => {
                    let reasons = engine.find_reasons(positive_atom);
                    let json_reasons = json_reasons(&reasons)?;
                    let str_reasons = show_reasons(&reasons, &nodes_prefixmap, &self.schema, self.config.width())?;
                    let status = ValidationStatus::conformant(str_reasons, json_reasons);
                    result.add_result((*node).clone(), label.clone(), status).map_err(|e| {
                        ValidatorError::AddingConformantError {
                            node: node.to_string(),
                            label: label.to_string(),
                            error: format!("{e}"),
                        }
                    })?;
                },
                Atom::Neg(negative_atom) => {
                    let errors = engine.find_errors(negative_atom);
                    let json_errors = json_errors(&errors)?;
                    let str_errors = show_errors(&errors, &nodes_prefixmap, &self.schema, self.config.width())?;
                    let status = ValidationStatus::non_conformant(str_errors, json_errors);
                    result.add_result((*node).clone(), label.clone(), status).map_err(|e| {
                        ValidatorError::AddingNonConformantError {
                            node: node.to_string(),
                            label: label.to_string(),
                            error: format!("{e}"),
                        }
                    })?;
                },
            }
        }
        for atom in &engine.pending() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            let status = ValidationStatus::pending();
            result.add_result((*node).clone(), label.clone(), status).map_err(|e| {
                ValidatorError::AddingPendingError {
                    node: node.to_string(),
                    label: label.to_string(),
                    error: format!("{e}"),
                }
            })?;
        }
        Ok(result)
    }

    pub fn shapes_prefixmap(&self) -> PrefixMap {
        self.schema.prefixmap()
    }
}

fn show_errors(
    errors: &[ValidatorError],
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
    width: usize,
) -> Result<String> {
    let mut result = String::new();
    match errors.len() {
        0 => {
            result.push_str("No detailed error provided.");
        },
        1 => {
            let str = errors[0].show_qualified(nodes_prefixmap, schema, width)?;
            result.push_str(str.trim_end_matches('\n'));
        },
        _ => {
            for (idx, error) in errors.iter().enumerate() {
                result.push_str(
                    format!(
                        "Error #{idx}: {}",
                        error
                            .show_qualified(nodes_prefixmap, schema, width)?
                            .trim_end_matches('\n')
                    )
                    .as_str(),
                );
                if idx + 1 < errors.len() {
                    result.push('\n');
                }
            }
        },
    }
    Ok(result)
}

fn json_errors(errors: &[ValidatorError]) -> Result<Value> {
    let vs: Result<Vec<_>> = errors
        .iter()
        .map(|err| {
            serde_json::to_value(err).map_err(|e| ValidatorError::ErrorSerializationError {
                source_error: err.to_string(),
                error: e.to_string(),
            })
        })
        .collect();
    let vs = vs?;
    let vs = Value::Array(vs);
    Ok(vs)
}

fn json_reasons(reasons: &[Reason]) -> Result<Value> {
    let rs: Result<Vec<_>> = reasons
        .iter()
        .map(|reason| {
            let r = reason.as_json().map_err(|e| ValidatorError::ReasonSerializationError {
                reason: reason.to_string(),
                error: format!("{e}"),
            })?;
            Ok(r)
        })
        .collect();
    let vs = rs?;
    let value = Value::Array(vs);
    Ok(value)
}

fn show_reasons(reasons: &[Reason], nodes_prefixmap: &PrefixMap, schema: &SchemaIR, width: usize) -> Result<String> {
    let mut result = String::new();
    match reasons.len() {
        0 => {
            result.push_str("No detailed reason provided.");
        },
        1 => {
            let str = reasons[0].show_qualified(nodes_prefixmap, schema, width)?;
            result.push_str(str.trim_end_matches('\n'));
        },
        _ => {
            for (idx, reason) in reasons.iter().enumerate() {
                result.push_str(
                    format!(
                        "Reason #{idx}: {}",
                        reason
                            .show_qualified(nodes_prefixmap, schema, width)?
                            .trim_end_matches('\n')
                    )
                    .as_str(),
                );
                if idx + 1 < reasons.len() {
                    result.push('\n');
                }
            }
        },
    }
    Ok(result)
}

#[cfg(test)]
mod tests {}
