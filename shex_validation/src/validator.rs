use crate::result_map::*;
use crate::solver;
use crate::validator_error::*;
use crate::validator_runner::ValidatorRunner;
use crate::ResultValue;
use crate::MAX_STEPS;
use iri_s::IriS;
use log::debug;
use rbe::Pending;
use shex_ast::compiled_schema::*;
use shex_ast::Node;
use shex_ast::Pred;
use shex_ast::ShapeLabelIdx;
use shex_ast::{compiled_schema::CompiledSchema, ShapeLabel};
use srdf::literal::Literal;
use srdf::NeighsIterator;
use srdf::{Object, SRDFComparisons, SRDF};
use std::collections::HashSet;
use std::hash::Hash;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

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
        while self.runner.no_end_steps() && self.runner.more_pending() {
            self.runner.new_step();
            let atom = self.runner.pop_pending().unwrap();
            debug!("Processing atom: ${atom:?}");
            self.runner.add_processing(&atom);
            self.check_node_atom(&atom, rdf)?;
        }
        Ok(())
    }

    pub fn check_node_atom<S>(&mut self, atom: &Atom, rdf: &S) -> Result<()>
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
        let idx = self.get_idx(shape)?;
        Ok(self.runner.result_map().get_result(&node, &idx))
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

    pub fn result_map(&self) -> ResultMap<Node, ShapeLabelIdx> {
        self.runner.result_map()
    }
}

fn find_shape_idx<'a>(idx: &'a ShapeLabelIdx, schema: &'a CompiledSchema) -> &'a ShapeExpr {
    let se = schema.find_shape_idx(idx).unwrap();
    se
}

#[cfg(test)]
mod tests {}
