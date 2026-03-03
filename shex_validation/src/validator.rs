use crate::Reason;
use crate::ValidatorConfig;
use crate::atom;
use crate::engine::Engine;
use crate::validator_error::*;
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
use tracing::trace;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;

#[derive(Debug)]
pub struct Validator {
    schema: SchemaIR,
    config: ValidatorConfig,
}

impl Validator {
    pub fn new(schema: SchemaIR, config: &ValidatorConfig) -> Result<Validator> {
        trace!("Creating Validator...");
        if config.check_negation_requirement.unwrap_or(true) && schema.has_neg_cycle() {
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
            schema,
            config: config.clone(),
        })
    }

    pub fn schema(&self) -> &SchemaIR {
        &self.schema
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
        let mut engine = Engine::new(&self.config);
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
        let mut engine = Engine::new(&self.config);
        let failures = self.fill_pending(&mut engine, shapemap, rdf, schema)?;
        trace!("Filled pending atoms: {:?}", engine.pending());
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
        trace!(
            "fill_pending: After filling pending atoms from QueryShapeMap: {}",
            shapemap
        );
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

    pub fn result_map(&self, engine: &mut Engine, maybe_nodes_prefixmap: &Option<PrefixMap>) -> Result<ResultShapeMap> {
        let nodes_prefixmap = match maybe_nodes_prefixmap {
            Some(pm) => pm.clone(),
            None => PrefixMap::default(),
        };
        let mut result = ResultShapeMap::new()
            .with_nodes_prefixmap(&nodes_prefixmap)
            .with_shapes_prefixmap(&self.schema.prefixmap());
        for atom in &engine.checked() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            match atom {
                Atom::Pos(pa) => {
                    let reasons = engine.find_reasons(pa);
                    let json_reasons = json_reasons(&reasons)?;
                    let str_reasons = show_reasons(&reasons, &nodes_prefixmap, &self.schema, self.config.width())?;
                    let status = ValidationStatus::conformant(str_reasons, json_reasons);
                    // result.add_ok()
                    result.add_result((*node).clone(), label.clone(), status).map_err(|e| {
                        ValidatorError::AddingConformantError {
                            node: node.to_string(),
                            label: label.to_string(),
                            error: format!("{e}"),
                        }
                    })?;
                },
                Atom::Neg(na) => {
                    let errors = engine.find_errors(na);
                    let json_errors = json_errors(&errors)?;
                    let status = ValidationStatus::non_conformant(show_errors(&errors), json_errors);
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

fn show_errors(errors: &[ValidatorError]) -> String {
    let mut result = String::new();
    if errors.len() == 1 {
        result.push_str(format!("Error {}\n", errors.first().unwrap()).as_str());
    } else {
        for (idx, err) in errors.iter().enumerate() {
            result.push_str(format!("Error #{idx}: {err}\n").as_str());
        }
    }
    result
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
            result.push_str("No detailed reason provided.\n");
            return Ok(result);
        },
        1 => {
            let str = reasons[0].show_qualified(nodes_prefixmap, schema, width)?;
            result.push_str(&str);
            return Ok(result);
        },
        _ => {
            for (idx, reason) in reasons.iter().enumerate() {
                result.push_str(
                    format!(
                        "Reason #{idx}: {}\n",
                        reason.show_qualified(nodes_prefixmap, schema, width)?
                    )
                    .as_str(),
                );
            }
        },
    }
    Ok(result)
}

#[cfg(test)]
mod tests {}
