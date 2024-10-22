use crate::atom;
use crate::validator_error::*;
use crate::validator_runner::Engine;
use crate::PosAtom;
use crate::Reason;
use crate::ResultValue;
use crate::ValidatorConfig;
use either::Either;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use serde_json::Value;
use shapemap::query_shape_map::QueryShapeMap;
use shapemap::ResultShapeMap;
use shapemap::ValidationStatus;
use shex_ast::compiled::compiled_schema::CompiledSchema;
use shex_ast::compiled::shape_expr::ShapeExpr;
use shex_ast::compiled::shape_label::ShapeLabel;
use shex_ast::object_value::ObjectValue;
use shex_ast::Node;
use shex_ast::ShapeExprLabel;
use shex_ast::ShapeLabelIdx;
use srdf::SRDF;
use tracing::debug;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;

#[derive(Debug)]
pub struct Validator {
    schema: CompiledSchema,
    runner: Engine,
}

impl Validator {
    pub fn new(schema: CompiledSchema, config: &ValidatorConfig) -> Validator {
        Validator {
            schema,
            runner: Engine::new(config),
        }
    }

    /// validate a node against a shape label
    pub fn validate_node_shape<S>(&mut self, node: &Node, shape: &ShapeLabel, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        let idx = self.get_idx(shape)?;
        self.runner.add_pending(node.clone(), idx);
        debug!("Before while loop: ${}@{}", node, idx);
        self.loop_validating(rdf)?;
        Ok(())
    }

    fn get_shape_expr_label(&mut self, label: &ShapeExprLabel) -> Result<ShapeLabelIdx> {
        self.schema
            .find_ref(label)
            .map_err(|error| ValidatorError::ShapeLabelNotFoundError {
                shape_label: label.clone(),
                error: format!("{error}"),
            })
    }

    pub fn validate_shapemap<S>(&mut self, shapemap: &QueryShapeMap, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        self.fill_pending(shapemap, rdf)?;
        self.loop_validating(rdf)?;
        Ok(())
    }

    fn fill_pending<S>(&mut self, shapemap: &QueryShapeMap, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        for (node_value, label) in shapemap.iter_node_shape(rdf) {
            let idx = self.get_shape_expr_label(label)?;
            let node = self.node_from_object_value(node_value, rdf)?;
            self.runner.add_pending(node.clone(), idx);
        }
        Ok(())
    }

    fn node_from_object_value<S>(&mut self, value: &ObjectValue, rdf: &S) -> Result<Node>
    where
        S: SRDF,
    {
        match value {
            ObjectValue::IriRef(IriRef::Iri(iri)) => Ok(Node::iri(iri.clone())),
            ObjectValue::IriRef(IriRef::Prefixed { prefix, local }) => {
                let iri = rdf.resolve_prefix_local(prefix, local)?;
                Ok(Node::iri(iri.clone()))
            }
            ObjectValue::Literal(_lit) => todo!(),
        }
    }

    fn loop_validating<S>(&mut self, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        while self.runner.no_end_steps() && self.runner.more_pending() {
            self.runner.new_step();
            let atom = self.runner.pop_pending().unwrap();
            debug!("Processing atom: ${atom:?}");
            self.runner.add_processing(&atom);
            let passed = self.check_node_atom(&atom, rdf)?;
            self.runner.remove_processing(&atom);
            match passed {
                Either::Right(reasons) => {
                    self.runner.add_checked_pos(atom, reasons);
                }
                Either::Left(errors) => {
                    self.runner.add_checked_neg(atom.negated(), errors);
                }
            }
        }
        Ok(())
    }

    pub fn check_node_atom<S>(
        &mut self,
        atom: &Atom,
        rdf: &S,
    ) -> Result<Either<Vec<ValidatorError>, Vec<Reason>>>
    where
        S: SRDF,
    {
        let (node, idx) = atom.get_value();
        let se = find_shape_idx(idx, &self.schema);
        match atom {
            Atom::Pos { .. } => self.runner.check_node_shape_expr(node, se, rdf),
            Atom::Neg { .. } => {
                // Check if a node doesn't conform to a shape expr
                todo!()
            }
        }
    }

    pub fn get_result(&self, node: &Node, shape: &ShapeLabel) -> Result<ResultValue> {
        if let Some(idx) = self.schema.find_shape_label_idx(shape) {
            let pos_atom = PosAtom::new((node.clone(), *idx));
            let atom = Atom::pos(&pos_atom);
            Ok(self.runner.get_result(&atom))
        } else {
            Err(ValidatorError::NotFoundShapeLabel {
                shape: shape.clone(),
            })
        }
    }

    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.runner.set_max_steps(max_steps);
        self
    }

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
        Ok(label)
    }

    pub fn result_map(&self, maybe_nodes_prefixmap: Option<PrefixMap>) -> Result<ResultShapeMap> {
        let mut result = match maybe_nodes_prefixmap {
            None => ResultShapeMap::new(),
            Some(pm) => ResultShapeMap::new().with_nodes_prefixmap(&pm),
        };
        for atom in &self.runner.checked() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            match atom {
                Atom::Pos(pa) => {
                    let reasons = self.runner.find_reasons(pa);
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
                    let errors = self.runner.find_errors(na);
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
        for atom in &self.runner.pending() {
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

fn find_shape_idx<'a>(idx: &'a ShapeLabelIdx, schema: &'a CompiledSchema) -> &'a ShapeExpr {
    let (_label, se) = schema.find_shape_idx(idx).unwrap();
    se
}

fn show_errors(errors: &Vec<ValidatorError>) -> String {
    let mut result = String::new();
    for (err, idx) in errors.iter().enumerate() {
        result.push_str(format!("Error #{idx}: {err}\n").as_str());
    }
    result
}

fn json_errors(_errors: &Vec<ValidatorError>) -> Value {
    let vs = vec!["todo", "errors"];
    vs.into()
}

fn json_reasons(_reasons: &Vec<Reason>) -> Value {
    let vs = vec!["todo", "reasons"];
    vs.into()
}

fn show_reasons(reasons: &Vec<Reason>) -> String {
    let mut result = String::new();
    for (reason, idx) in reasons.iter().enumerate() {
        result.push_str(format!("Reason #{idx}: {reason}\n").as_str());
    }
    result
}

#[cfg(test)]
mod tests {}
