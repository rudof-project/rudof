use crate::atom;
use crate::validator_error::*;
use crate::validator_runner::Engine;
use crate::Reason;
use crate::ValidatorConfig;
use either::Either;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use serde_json::Value;
use shapemap::query_shape_map::QueryShapeMap;
use shapemap::ResultShapeMap;
use shapemap::ValidationStatus;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::object_value::ObjectValue;
use shex_ast::Node;
use shex_ast::ShapeExprLabel;
use shex_ast::ShapeLabelIdx;
use srdf::Query;
use tracing::debug;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;

#[derive(Debug)]
pub struct Validator {
    schema: SchemaIR,
    config: ValidatorConfig,
}

impl Validator {
    pub fn new(schema: SchemaIR, config: &ValidatorConfig) -> Result<Validator> {
        if config.check_negation_requirement.unwrap_or(true) && schema.has_neg_cycle() {
            let neg_cycles = schema.neg_cycles();
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
        // let engine = Engine::new(config);
        Ok(Validator {
            schema: schema,
            config: config.clone(),
        })
    }

    /*pub fn reset_result_map(&mut self) {
        self.runner.reset()
    }*/

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
        maybe_shapes_prefixmap: &Option<PrefixMap>,
    ) -> Result<ResultShapeMap>
    where
        S: Query,
    {
        let idx = self.get_idx(shape)?;
        let mut engine = Engine::new(&self.config);
        engine.add_pending(node.clone(), idx);
        debug!("Before while loop: ${}@{}", node, idx);
        self.loop_validating(&mut engine, rdf, schema)?;
        let result =
            self.result_map(&mut engine, &maybe_nodes_prefixmap, &maybe_shapes_prefixmap)?;
        Ok(result)
    }

    fn get_shape_expr_label(
        &self,
        label: &ShapeExprLabel,
        schema: &SchemaIR,
    ) -> Result<ShapeLabelIdx> {
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
        maybe_shapes_prefixmap: &Option<PrefixMap>,
    ) -> Result<ResultShapeMap>
    where
        S: Query,
    {
        let mut engine = Engine::new(&self.config);
        self.fill_pending(&mut engine, shapemap, rdf, schema)?;
        self.loop_validating(&mut engine, rdf, schema)?;
        let result =
            self.result_map(&mut engine, &maybe_nodes_prefixmap, &maybe_shapes_prefixmap)?;
        Ok(result)
    }

    fn fill_pending<S>(
        &self,
        engine: &mut Engine,
        shapemap: &QueryShapeMap,
        rdf: &S,
        schema: &SchemaIR,
    ) -> Result<()>
    where
        S: Query,
    {
        for (node_value, label) in shapemap.iter_node_shape(rdf) {
            let idx = self.get_shape_expr_label(label, schema)?;
            let node = self.node_from_object_value(node_value, rdf)?;
            engine.add_pending(node.clone(), idx);
        }
        Ok(())
    }

    fn node_from_object_value<S>(&self, value: &ObjectValue, rdf: &S) -> Result<Node>
    where
        S: Query,
    {
        match value {
            ObjectValue::IriRef(IriRef::Iri(iri)) => Ok(Node::iri(iri.clone())),
            ObjectValue::IriRef(IriRef::Prefixed { prefix, local }) => {
                let iri = rdf.resolve_prefix_local(prefix, local)?;
                Ok(Node::iri(iri.clone()))
            }
            ObjectValue::Literal(lit) => Ok(Node::literal(lit.clone())),
        }
    }

    fn loop_validating<S>(&self, engine: &mut Engine, rdf: &S, schema: &SchemaIR) -> Result<()>
    where
        S: Query,
    {
        while engine.no_end_steps() && engine.more_pending() {
            engine.new_step();
            let atom = engine.pop_pending().unwrap();
            debug!("Processing atom: {}", show(&atom));
            engine.add_processing(&atom);
            let passed = self.check_node_atom(engine, &atom, rdf, schema)?;
            engine.remove_processing(&atom);
            match passed {
                Either::Right(reasons) => {
                    engine.add_checked_pos(atom, reasons);
                }
                Either::Left(errors) => {
                    engine.add_checked_neg(atom.negated(), errors);
                }
            }
        }
        Ok(())
    }

    pub fn check_node_atom<S>(
        &self,
        engine: &mut Engine,
        atom: &Atom,
        rdf: &S,
        schema: &SchemaIR,
    ) -> Result<Either<Vec<ValidatorError>, Vec<Reason>>>
    where
        S: Query,
    {
        let (node, idx) = atom.get_value();
        let se = find_shape_idx(idx, &self.schema);
        match atom {
            Atom::Pos { .. } => engine.check_node_shape_expr(node, se, rdf, schema),
            Atom::Neg { .. } => {
                // Check if a node doesn't conform to a shape expr
                todo!()
            }
        }
    }

    /* This method should be internal as it exposes the engine
    pub fn get_result(
        &self,
        engine: &mut Engine,
        node: &Node,
        shape: &ShapeLabel,
    ) -> Result<ResultValue> {
        if let Some(idx) = self.schema.find_shape_label_idx(shape) {
            let pos_atom = (node.clone(), *idx);
            let atom = Atom::pos(&pos_atom);
            Ok(engine.get_result(&atom))
        } else {
            Err(ValidatorError::NotFoundShapeLabel {
                shape: shape.clone(),
            })
        }
    } */

    /*pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.runner.set_max_steps(max_steps);
        self
    }*/

    fn get_idx(&self, shape: &ShapeLabel) -> Result<ShapeLabelIdx> {
        match self.schema.find_label(shape) {
            Some((idx, _se)) => Ok(*idx),
            None => Err(ValidatorError::NotFoundShapeLabel {
                shape: (*shape).clone(),
            }),
        }
    }

    fn get_shape_label(&self, idx: &ShapeLabelIdx) -> Result<&ShapeLabel> {
        let (label, _se) = self.schema.find_shape_idx(idx).unwrap();
        match label {
            Some(label) => Ok(label),
            None => Err(ValidatorError::NotFoundShapeLabelWithIndex { idx: *idx }),
        }
    }

    pub fn result_map(
        &self,
        engine: &mut Engine,
        maybe_nodes_prefixmap: &Option<PrefixMap>,
        maybe_shapes_prefixmap: &Option<PrefixMap>,
    ) -> Result<ResultShapeMap> {
        let mut result = match (maybe_nodes_prefixmap, maybe_shapes_prefixmap) {
            (None, None) => ResultShapeMap::new(),
            (Some(npm), None) => ResultShapeMap::new().with_nodes_prefixmap(&npm),
            (None, Some(spm)) => ResultShapeMap::new().with_shapes_prefixmap(&spm),
            (Some(npm), Some(spm)) => ResultShapeMap::new()
                .with_nodes_prefixmap(&npm)
                .with_shapes_prefixmap(&spm),
        };
        for atom in &engine.checked() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            match atom {
                Atom::Pos(pa) => {
                    let reasons = engine.find_reasons(pa);
                    let status = ValidationStatus::conformant(
                        show_reasons(&reasons),
                        json_reasons(&reasons),
                    );
                    // result.add_ok()
                    result
                        .add_result((*node).clone(), label.clone(), status)
                        .map_err(|e| ValidatorError::AddingConformantError {
                            node: node.to_string(),
                            label: label.to_string(),
                            error: format!("{e}"),
                        })?;
                }
                Atom::Neg(na) => {
                    let errors = engine.find_errors(na);
                    let status = ValidationStatus::non_conformant(
                        show_errors(&errors),
                        json_errors(&errors),
                    );
                    result
                        .add_result((*node).clone(), label.clone(), status)
                        .map_err(|e| ValidatorError::AddingNonConformantError {
                            node: node.to_string(),
                            label: label.to_string(),
                            error: format!("{e}"),
                        })?;
                }
            }
        }
        for atom in &engine.pending() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            let status = ValidationStatus::pending();
            result
                .add_result((*node).clone(), label.clone(), status)
                .map_err(|e| ValidatorError::AddingPendingError {
                    node: node.to_string(),
                    label: label.to_string(),
                    error: format!("{e}"),
                })?;
        }
        Ok(result)
    }

    pub fn shapes_prefixmap(&self) -> PrefixMap {
        self.schema.prefixmap()
    }
}

fn find_shape_idx<'a>(idx: &'a ShapeLabelIdx, schema: &'a SchemaIR) -> &'a ShapeExpr {
    let (_label, se) = schema.find_shape_idx(idx).unwrap();
    se
}

fn show_errors(errors: &[ValidatorError]) -> String {
    let mut result = String::new();
    for (err, idx) in errors.iter().enumerate() {
        result.push_str(format!("Error #{idx}: {err}\n").as_str());
    }
    result
}

fn json_errors(_errors: &[ValidatorError]) -> Value {
    let vs = vec!["todo", "errors"];
    vs.into()
}

fn json_reasons(reasons: &[Reason]) -> Value {
    let value = Value::Array(reasons.iter().map(|reason| reason.as_json()).collect());
    value
}

fn show_reasons(reasons: &[Reason]) -> String {
    let mut result = String::new();
    for (reason, idx) in reasons.iter().enumerate() {
        result.push_str(format!("Reason #{idx}: {reason}\n").as_str());
    }
    result
}

fn show(atom: &Atom) -> String {
    match atom {
        Atom::Pos((node, idx)) => format!("+({node},{idx})"),
        Atom::Neg((node, idx)) => format!("!({node},{idx})"),
    }
}

#[cfg(test)]
mod tests {}
