use crate::result_map::*;
use crate::validator_error::*;
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

    pub fn validate_node_shape<S>(&mut self, node: Node, shape: ShapeLabel, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        if let Some((idx, _se)) = self.schema.find_label(&shape) {
            self.runner.add_pending(node, *idx);
            while self.runner.no_end_steps() && self.runner.more_pending() {
                self.runner.new_step();
                println!("In validate_node_shape loop: step {}", self.runner.steps());
                let (n, idx) = self.runner.pop_pending().unwrap();
                self.runner.add_current(&n, &idx);
                self.check_node_idx(&n, &idx, rdf)?;
            }
            Ok(())
        } else {
            Err(ValidatorError::NotFoundShapeLabel { shape })
        }
    }

    pub fn check_node_idx<S>(&mut self, node: &Node, idx: &ShapeLabelIdx, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        let se = find_shape_idx(idx, &self.schema); // self.schema.get_shape_expr(shape).unwrap(); // Self::find_shape_expr(shape, &self.schema);
        self.runner.check_node_shape_expr(node, se, rdf)
    }

    pub fn result_map(&self) -> ResultMap<Node, ShapeLabelIdx> {
        self.runner.result_map()
    }

    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.runner.max_steps = max_steps;
        self
    }
}

#[derive(Debug)]
struct ValidatorRunner {
    current_goal: Option<(Node, ShapeLabelIdx)>,
    result_map: ResultMap<Node, ShapeLabelIdx>,
    alternatives: Vec<ResultMap<Node, ShapeLabelIdx>>,
    max_steps: usize,
    step_counter: usize,
}

impl ValidatorRunner {
    pub fn new() -> ValidatorRunner {
        ValidatorRunner {
            current_goal: None,
            result_map: ResultMap::new(),
            alternatives: Vec::new(),
            max_steps: MAX_STEPS,
            step_counter: 0,
        }
    }

    pub fn result_map(&self) -> ResultMap<Node, ShapeLabelIdx> {
        self.result_map.clone()
    }

    fn add_current(&mut self, node: &Node, shape: &ShapeLabelIdx) {
        self.set_current_goal(&node, &shape);
    }

    pub fn set_current_goal(&mut self, n: &Node, s: &ShapeLabelIdx) {
        self.current_goal = Some(((*n).clone(), (*s).clone()));
    }

    pub fn is_current_goal(&self, n: &Node, s: &ShapeLabelIdx) -> bool {
        if let Some((cn, cs)) = &self.current_goal {
            *cn == *n && *cs == *s
        } else {
            false
        }
    }

    pub fn new_step(&mut self) {
        self.step_counter += 1;
    }

    pub fn add_ok(&mut self, n: Node, s: ShapeLabelIdx) {
        self.result_map.add_ok(n, s);
    }

    pub fn more_pending(&self) -> bool {
        self.result_map.more_pending()
    }

    pub fn add_pending(&mut self, n: Node, s: ShapeLabelIdx) {
        self.result_map.add_pending(n, s);
    }

    pub fn pop_pending(&mut self) -> Option<(Node, ShapeLabelIdx)> {
        self.result_map.pop_pending()
    }

    pub fn steps(&self) -> usize {
        self.step_counter
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }

    fn no_end_steps(&self) -> bool {
        self.steps() < self.max_steps()
    }

    fn check_node_shape_expr<S>(&mut self, node: &Node, se: &ShapeExpr, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        debug!(
            "Step {}. Checking node {node:?} with shape_expr: {se:?}",
            self.step_counter
        );
        match se {
            ShapeExpr::NodeConstraint {
                node_kind,
                datatype,
                xs_facet,
                values,
                cond,
            } => {
                cond.matches(node)?;
                Ok(())
            }
            ShapeExpr::Ref { idx } => {
                todo!()
            }
            ShapeExpr::ShapeAnd { exprs } => {
                todo!()
            }
            ShapeExpr::ShapeNot { expr } => {
                todo!()
            }
            ShapeExpr::ShapeOr { exprs } => {
                todo!()
            }
            ShapeExpr::Shape {
                closed,
                extra,
                rbe_table,
                sem_acts,
                annotations,
            } => {
                let values = self.neighs(node, rdf)?;
                let mut rs = rbe_table.matches(values)?;
                if let Some(pending_result) = rs.next() {
                    println!("### obtained pending result, step {}", self.step_counter);
                    let pending = pending_result?;
                    let mut effective_pending = Pending::new();
                    for (p, v) in pending.iter() {
                        if self.is_current_goal(p, v) {
                            self.add_ok(p.clone(), *v);
                        } else {
                            effective_pending.insert(p.clone(), v.clone());
                        }
                    }
                    self.result_map.merge_pending(effective_pending);
                    // TODO: Add alternatives...
                    Ok(())
                } else {
                    Err(ValidatorError::RbeFailed())
                }
            }
            ShapeExpr::Empty => Ok(()),
            ShapeExpr::ShapeExternal {} => {
                todo!()
            }
        }
    }

    fn cnv_iri<S>(&self, iri: S::IRI) -> Pred
    where
        S: SRDF,
    {
        let iri = S::iri2iri_s(iri);
        Pred::from(iri)
    }

    fn cnv_object<S>(&self, term: S::Term) -> Node
    where
        S: SRDF,
    {
        let object = S::term2object(term);
        Node::from(object)
    }

    fn neighs<S>(&self, node: &Node, rdf: &S) -> Result<Vec<(Pred, Node)>>
    where
        S: SRDF,
    {
        let node = self.get_rdf_node(&node, rdf);
        if let Some(subject) = rdf.term_as_subject(&node) {
            let preds = rdf
                .get_predicates_for_subject(&subject)
                .map_err(|e| self.cnv_err::<S>(e))?;
            let mut result = Vec::new();
            for p in &preds {
                let objects = rdf
                    .get_objects_for_subject_predicate(&subject, &p)
                    .map_err(|e| self.cnv_err::<S>(e))?;
                let iri = self.cnv_iri::<S>(p.clone());
                debug!("neighs...iri: {iri:?} p: {:?}", p.to_string());
                for o in objects {
                    let object = self.cnv_object::<S>(o);
                    result.push((iri.clone(), object));
                }
            }
            Ok(result)
        } else {
            todo!()
        }
    }

    fn cnv_err<S>(&self, err: S::Err) -> ValidatorError
    where
        S: SRDF,
    {
        todo!()
    }

    fn get_rdf_node<S>(&self, node: &Node, rdf: &S) -> S::Term
    where
        S: SRDF,
    {
        match node.as_object() {
            Object::Iri { iri } => {
                let i = S::iri_s2iri(iri);
                S::iri_as_term((*i).clone())
            }
            Object::BlankNode(id) => {
                todo!()
            }
            Object::Literal(lit) => {
                todo!()
            }
        }
    }

    /*pub fn result_map(&self) -> ResultMap<Object, ShapeLabelIdx> {
        self.validation_state.result_map.clone()
    }*/
}

fn find_shape_idx<'a>(idx: &'a ShapeLabelIdx, schema: &'a CompiledSchema) -> &'a ShapeExpr {
    let se = schema.find_shape_idx(idx).unwrap();
    se
}

#[cfg(test)]
mod tests {}
