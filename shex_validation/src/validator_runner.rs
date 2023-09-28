use crate::result_map::*;
use crate::solver;
use crate::validator_error::*;
use crate::ResultValue;
use crate::MAX_STEPS;
use indexmap::IndexSet;
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

#[derive(Debug)]
pub struct ValidatorRunner {
    checked: IndexSet<Atom>,
    processing: IndexSet<Atom>,
    pending: IndexSet<Atom>,
    rules: IndexSet<solver::Rule<Atom>>,
    // alternatives: Vec<ResultMap<Node, ShapeLabelIdx>>,
    max_steps: usize,
    step_counter: usize,
}

impl ValidatorRunner {
    pub fn new() -> ValidatorRunner {
        ValidatorRunner {
            checked: IndexSet::new(),
            processing: IndexSet::new(),
            pending: IndexSet::new(),
            rules: IndexSet::new(),
            // alternatives: Vec::new(),
            max_steps: MAX_STEPS,
            step_counter: 0,
        }
    }

    pub(crate) fn add_processing(&mut self, atom: &Atom) {
        self.processing.insert((*atom).clone());
    }

    pub(crate) fn remove_processing(&mut self, atom: &Atom) {
        self.processing.remove(atom);
    }

    pub(crate) fn add_checked(&mut self, atom: &Atom) {
        self.checked.insert((*atom).clone());
    }

    pub(crate) fn checked(&self) -> IndexSet<Atom> {
        self.checked.clone()
    }

    pub(crate) fn pending(&self) -> IndexSet<Atom> {
        self.pending.clone()
    }

    pub fn set_max_steps(&mut self, max_steps: usize) {
        self.max_steps = max_steps;
    }

    pub(crate) fn is_processing(&self, atom: &Atom) -> bool {
        self.processing.contains(atom)
    }

    pub(crate) fn get_result(&self, atom: &Atom) -> ResultValue {
        if self.checked.contains(atom) {
            ResultValue::Ok
        } else if self.checked.contains(&atom.negated()) {
            ResultValue::Failed
        } else if self.pending.contains(atom) {
            ResultValue::Pending
        } else if self.processing.contains(atom) {
            ResultValue::Processing
        } else {
            ResultValue::Unknown
        }
    }

    pub fn new_step(&mut self) {
        self.step_counter += 1;
    }

    pub fn add_ok(&mut self, n: Node, s: ShapeLabelIdx) {
        self.checked.insert(Atom::pos((n, s)));
    }

    pub fn more_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    pub fn add_pending(&mut self, n: Node, s: ShapeLabelIdx) {
        // self.result_map.add_pending(n, s);
        self.pending.insert(Atom::pos((n, s)));
    }

    pub fn pop_pending(&mut self) -> Option<Atom> {
        // self.result_map.pop_pending()
        self.pending.pop()
    }

    pub fn steps(&self) -> usize {
        self.step_counter
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }

    pub(crate) fn no_end_steps(&self) -> bool {
        self.steps() < self.max_steps()
    }

    pub(crate) fn check_node_shape_expr<S>(
        &mut self,
        node: &Node,
        se: &ShapeExpr,
        rdf: &S,
    ) -> Result<bool>
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
            } => match cond.matches(node) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            ShapeExpr::Ref { idx } => {
                todo!()
            }
            ShapeExpr::ShapeAnd { exprs } => {
                for e in exprs {
                    let result = self.check_node_shape_expr(node, e, rdf)?;
                    if !result {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ShapeExpr::ShapeNot { expr } => {
                let result = self.check_node_shape_expr(node, expr, rdf)?;
                if !result {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            ShapeExpr::ShapeOr { exprs } => {
                for e in exprs {
                    let result = self.check_node_shape_expr(node, e, rdf)?;
                    if result {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            ShapeExpr::Shape {
                closed,
                extra,
                rbe_table,
                sem_acts,
                annotations,
            } => {
                let values = self.neighs(node, rdf)?;
                let mut result_iter = rbe_table.matches(values)?;
                let mut current_err = None;
                let counter = self.step_counter;
                // let next_result = result_iter.next();
                let mut found = false;
                while let Some(next_result) = result_iter.next() {
                    match next_result {
                        Ok(pending) => {
                            for (p, v) in pending.iter() {
                                debug!("Step {counter}: Value in pending: {p}/{v}");
                                let atom = Atom::pos(((*p).clone(), *v));
                                if self.is_processing(&atom) {
                                    let pred = p.clone();
                                    debug!(
                                        "Step {counter} Adding ok: {}/{v} because it was already processed",
                                        &pred
                                    );
                                    self.add_ok(pred, *v);
                                } else {
                                    self.insert_pending(&atom);
                                    // Should also atore result_iter in some place?
                                }
                            }
                            found = true;
                            break
                        }
                        Err(err) => {
                            current_err = Some(err);
                        }
                    }
                }
                Ok(found)
                /*                 if let Some(result) = result_iter.next() {
                    let counter = self.step_counter;
                    let pending = match result {
                        Ok(pending) => pending,
                        Err(err) => {
                            debug!("Failed entry: {err}");
                            todo!();
                        }
                    };
                    debug!(
                        "Step {counter}: Pending {pending:?}, Processing: {:?}",
                        &self.processing
                    );
                    for (p, v) in pending.iter() {
                        debug!("Step {counter}: Value in pending: {p}/{v}");
                        let atom = Atom::pos(((*p).clone(), *v));
                        if self.is_processing(&atom) {
                            let pred = p.clone();
                            debug!(
                                "Step {counter} Adding ok: {}/{v} because it was already processed",
                                &pred
                            );
                            self.add_ok(pred, *v);
                        } else {
                            self.insert_pending(&atom);
                        }
                    }
                    // self.result_map.merge_pending(effective_pending);
                    // TODO: Add alternatives...
                    Ok(true)
                } else {
                    Err(ValidatorError::RbeFailed())
                } */
            }
            ShapeExpr::Empty => Ok(true),
            ShapeExpr::ShapeExternal {} => Ok(true),
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

    pub fn insert_pending(&mut self, atom: &Atom) {
        self.pending.insert((*atom).clone());
    }
}
