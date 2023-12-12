use crate::result_map::*;
use crate::solver;
use crate::validator_error::*;
use crate::validator_runner::ValidatorRunner;
use crate::ResultValue;
use log::debug;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use shapemap::query_shape_map::QueryShapeMap;
use shex_ast::ShapeExprLabel;
use shex_ast::Node;
use shex_ast::ShapeLabelIdx;
use shex_ast::compiled::compiled_schema::CompiledSchema;
use shex_ast::compiled::shape_expr::ShapeExpr;
use shex_ast::compiled::shape_label::ShapeLabel;
use shex_ast::object_value::ObjectValue;
use srdf::SRDF;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = solver::Atom<(Node, ShapeLabelIdx)>;

pub struct Validator {
    schema: CompiledSchema,
    runner: ValidatorRunner,
}

impl Validator {
    pub fn new(schema: CompiledSchema) -> Validator {
        Validator {
            schema,
            runner: ValidatorRunner::new(),
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
        self.schema.find_ref(label).map_err(|e| ValidatorError::ShapeLabelNotFoundError {
            shape_label: label.clone(),
            err: e
        })
    }

    pub fn validate_shapemap<S>(&mut self, shapemap: &QueryShapeMap, rdf: &S) -> Result<()> 
    where S: SRDF {
        self.fill_pending(shapemap, rdf)?;
        self.loop_validating(rdf)?;
        Ok(())
    }

    fn fill_pending<S>(&mut self, shapemap: &QueryShapeMap, rdf: &S) -> Result<()> 
    where S: SRDF {
        for (node_value, label) in shapemap.iter_node_shape(rdf) {
            let idx = self.get_shape_expr_label(label)?;
            let node = self.node_from_object_value(node_value, rdf)?;
            self.runner.add_pending(node.clone(), idx);
        };
        Ok(())
    }

    fn node_from_object_value<S>(&mut self, value: &ObjectValue, rdf: &S) -> Result<Node> 
    where S: SRDF {
        match value {
            ObjectValue::IriRef(IriRef::Iri(iri)) => Ok(Node::iri(iri.clone())),
            ObjectValue::IriRef(IriRef::Prefixed { prefix, local }) => {
                let iri = rdf.resolve_prefix_local(prefix, local)?;
                Ok(Node::iri(iri.clone()))
            }
            ObjectValue::Literal(lit) => todo!(),
        }
    }

    fn loop_validating<S>(&mut self, rdf: &S) -> Result<()> where S: SRDF {
        while self.runner.no_end_steps() && self.runner.more_pending() {
            self.runner.new_step();
            let atom = self.runner.pop_pending().unwrap();
            debug!("Processing atom: ${atom:?}");
            self.runner.add_processing(&atom);
            let passed = self.check_node_atom(&atom, rdf)?;
            self.runner.remove_processing(&atom);
            if passed {
                self.runner.add_checked(&atom);
            } else {
                self.runner.add_checked(&atom.negated());
            }
        }
        Ok(())
    }

    pub fn check_node_atom<S>(&mut self, atom: &Atom, rdf: &S) -> Result<bool>
    where
        S: SRDF,
    {
        let (node, idx) = atom.get_value();
        let se = find_shape_idx(idx, &self.schema);
        match atom {
            Atom::Pos { .. } => self.runner.check_node_shape_expr(node, se, rdf),
            Atom::Neg { .. } => {
                todo!()
            }
        }
    }

    pub fn get_result(&self, node: &Node, shape: &ShapeLabel) -> Result<ResultValue> {
        if let Some(idx) = self.schema.find_shape_label_idx(shape) {
            let atom = Atom::pos((node.clone(), idx.clone()));
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
            Some((idx, _se)) => Ok(idx.clone()),
            None => Err(ValidatorError::NotFoundShapeLabel {
                shape: (*shape).clone(),
            }),
        }
    }

    fn get_shape_label(&self, idx: &ShapeLabelIdx) -> Result<&ShapeLabel> {
        let (label, _se) = self.schema.find_shape_idx(idx).unwrap();
        Ok(label)
    }

    pub fn result_map(&self, maybe_nodes_prefixmap: Option<PrefixMap>) -> Result<ResultMap> {
        let mut result = match maybe_nodes_prefixmap {
            None => ResultMap::new(),
            Some(pm) => ResultMap::new().with_nodes_prefixmap(pm),
        };
        for atom in &self.runner.checked() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            match atom {
                Atom::Pos { .. } => result.add_ok((*node).clone(), label.clone()),
                Atom::Neg { .. } => result.add_fail((*node).clone(), label.clone()),
            }
        }
        for atom in &self.runner.pending() {
            let (node, idx) = atom.get_value();
            let label = self.get_shape_label(idx)?;
            result.add_pending((*node).clone(), label.clone());
        }
        // TODO: Should I also add processing nodes as pending?
        Ok(result)
    }
}

fn find_shape_idx<'a>(idx: &'a ShapeLabelIdx, schema: &'a CompiledSchema) -> &'a ShapeExpr {
    let (_label, se) = schema.find_shape_idx(idx).unwrap();
    se
}

#[cfg(test)]
mod tests {}
