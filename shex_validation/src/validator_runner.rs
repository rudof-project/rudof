use crate::atom;
use crate::validator_error::*;
use crate::Reason;
use crate::ResultValue;
use crate::ValidatorConfig;
use either::Either;
use indexmap::IndexSet;
use iri_s::IriS;
use rbe::MatchTableIter;
use shex_ast::compiled::preds::Preds;
use shex_ast::compiled::shape::Shape;
use shex_ast::compiled::shape_expr::ShapeExpr;
use shex_ast::Node;
use shex_ast::Pred;
use shex_ast::ShapeLabelIdx;
use srdf::{Object, Query};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use tracing::debug;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;
type NegAtom = atom::NegAtom<(Node, ShapeLabelIdx)>;
type PosAtom = atom::PosAtom<(Node, ShapeLabelIdx)>;
// type Rule = rule::Rule<(Node, ShapeLabelIdx)>;
type Neighs = (Vec<(Pred, Node)>, Vec<Pred>);

#[derive(Debug)]
pub struct Engine {
    checked: IndexSet<Atom>,
    processing: IndexSet<Atom>,
    pending: IndexSet<Atom>,
    //rules: Vec<Rule>,
    alternative_match_iterators: Vec<MatchTableIter<Pred, Node, ShapeLabelIdx>>,
    // alternatives: Vec<ResultMap<Node, ShapeLabelIdx>>,
    config: ValidatorConfig,
    step_counter: usize,
    reasons: HashMap<PosAtom, Vec<Reason>>,
    errors: HashMap<NegAtom, Vec<ValidatorError>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(&ValidatorConfig::default())
    }
}

impl Engine {
    pub fn new(config: &ValidatorConfig) -> Engine {
        Engine {
            checked: IndexSet::new(),
            processing: IndexSet::new(),
            pending: IndexSet::new(),
            //rules: Vec::new(),
            alternative_match_iterators: Vec::new(),
            config: config.clone(),
            step_counter: 0,
            reasons: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        let config = self.config.clone();
        *self = Engine::new(&config);
    }

    pub(crate) fn add_processing(&mut self, atom: &Atom) {
        self.processing.insert((*atom).clone());
    }

    pub(crate) fn remove_processing(&mut self, atom: &Atom) {
        self.processing.swap_remove(atom);
    }

    pub(crate) fn add_checked_pos(&mut self, atom: Atom, reasons: Vec<Reason>) {
        let new_atom = atom.clone();
        match atom {
            Atom::Pos(pa) => {
                self.checked.insert(new_atom);
                self.add_reasons(pa, reasons)
            }
            Atom::Neg(_na) => {
                todo!()
            }
        }
    }

    pub(crate) fn add_checked_neg(&mut self, atom: Atom, errors: Vec<ValidatorError>) {
        let new_atom = atom.clone();
        match atom {
            Atom::Neg(na) => {
                self.checked.insert(new_atom);
                self.add_errors(na, errors)
            }
            Atom::Pos(_na) => {
                todo!()
            }
        }
    }

    pub(crate) fn checked(&self) -> IndexSet<Atom> {
        self.checked.clone()
    }

    fn add_reasons(&mut self, pa: PosAtom, rs: Vec<Reason>) {
        match self.reasons.entry(pa) {
            Entry::Occupied(mut vs) => vs.get_mut().extend(rs),
            Entry::Vacant(vac) => {
                vac.insert(rs);
            }
        }
    }

    fn add_errors(&mut self, na: NegAtom, es: Vec<ValidatorError>) {
        match self.errors.entry(na) {
            Entry::Occupied(mut vs) => vs.get_mut().extend(es),
            Entry::Vacant(vac) => {
                vac.insert(es);
            }
        }
    }

    pub(crate) fn pending(&self) -> IndexSet<Atom> {
        self.pending.clone()
    }

    pub fn set_max_steps(&mut self, max_steps: usize) {
        self.config.set_max_steps(max_steps);
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
        let pa = PosAtom::new((n, s));
        self.checked.insert(Atom::pos(&pa));
    }

    pub fn add_failed(&mut self, n: Node, s: ShapeLabelIdx, err: ValidatorError) {
        let atom = NegAtom::new((n, s));
        self.checked.insert(Atom::neg(&atom));
        match self.errors.entry(atom) {
            Entry::Occupied(mut es) => es.get_mut().push(err),
            Entry::Vacant(vacant) => {
                vacant.insert(vec![err]);
            }
        }
    }

    pub fn more_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    pub fn add_pending(&mut self, n: Node, s: ShapeLabelIdx) {
        // self.result_map.add_pending(n, s);
        let pos_atom = PosAtom::new((n, s));
        self.pending.insert(Atom::pos(&pos_atom));
    }

    pub fn pop_pending(&mut self) -> Option<Atom> {
        // self.result_map.pop_pending()
        self.pending.pop()
    }

    pub fn steps(&self) -> usize {
        self.step_counter
    }

    pub fn max_steps(&self) -> usize {
        self.config.max_steps()
    }

    pub(crate) fn no_end_steps(&self) -> bool {
        self.steps() < self.max_steps()
    }

    pub(crate) fn find_errors(&self, na: &NegAtom) -> Vec<ValidatorError> {
        match self.errors.get(na) {
            Some(vs) => vs.to_vec(),
            None => Vec::new(),
        }
    }

    pub(crate) fn find_reasons(&self, pa: &PosAtom) -> Vec<Reason> {
        match self.reasons.get(pa) {
            Some(vs) => vs.to_vec(),
            None => Vec::new(),
        }
    }

    pub(crate) fn check_node_shape_expr<S>(
        &mut self,
        node: &Node,
        se: &ShapeExpr,
        rdf: &S,
    ) -> Result<Either<Vec<ValidatorError>, Vec<Reason>>>
    where
        S: Query,
    {
        debug!(
            "Step {}. Checking node {node:?} with shape_expr: {se:?}",
            self.step_counter
        );
        match se {
            ShapeExpr::NodeConstraint(nc) => match nc.cond().matches(node) {
                Ok(_pending) => {
                    // TODO: Add pending to pending nodes
                    // I think this is not needed because node constraints will not generate pending nodes
                    Ok(Either::Right(vec![Reason::NodeConstraintPassed {
                        node: node.clone(),
                        nc: nc.clone(),
                    }]))
                }
                Err(err) => Ok(Either::Left(vec![ValidatorError::RbeError(err)])),
            },
            ShapeExpr::Ref { .. } => {
                todo!()
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                for e in exprs {
                    let result = self.check_node_shape_expr(node, e, rdf)?;
                    if let Some(errors) = result.left() {
                        return Ok(Either::Left(vec![ValidatorError::ShapeAndError {
                            shape_expr: e.clone(),
                            node: node.clone(),
                            errors: ValidatorErrors::new(errors),
                        }]));
                    }
                }
                Ok(Either::Right(vec![Reason::ShapeAndPassed {
                    node: node.clone(),
                    se: se.clone(),
                }]))
            }
            ShapeExpr::ShapeNot { expr, .. } => {
                let result = self.check_node_shape_expr(node, expr, rdf)?;
                match result {
                    Either::Left(_errors) => {
                        todo!()
                    }
                    Either::Right(_errors) => {
                        todo!()
                    }
                }
            }
            ShapeExpr::ShapeOr { .. } => {
                todo!()
                /*for e in exprs {
                    let result = self.check_node_shape_expr(node, e, rdf)?;
                    if result {
                        return Ok(true);
                    }
                }
                Ok(false)*/
            }
            ShapeExpr::Shape(shape) => self.check_node_shape(node, shape, rdf),
            ShapeExpr::Empty => Ok(Either::Right(Vec::new())),
            ShapeExpr::External {} => Ok(Either::Right(Vec::new())),
        }
    }

    fn check_node_shape<S>(
        &mut self,
        node: &Node,
        shape: &Shape,
        rdf: &S,
    ) -> Result<Either<Vec<ValidatorError>, Vec<Reason>>>
    where
        S: Query,
    {
        let (values, remainder) = self.neighs(node, shape.preds(), rdf)?;
        if shape.is_closed() && !remainder.is_empty() {
            let errs = vec![ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(remainder),
                declared: Preds::new(
                    shape
                        .preds()
                        .iter()
                        .map(|iri| {
                            let new_iri = iri.clone();
                            Pred::from(new_iri)
                        })
                        .collect(),
                ),
            }];
            return Ok(Either::Left(errs));
        };
        debug!("Neighs of {node}: {values:?}");
        let mut result_iter = shape.rbe_table().matches(values)?;
        let mut current_err = None;
        let counter = self.step_counter;
        let mut found = false;
        let mut iter_count = 0;

        // Search for the first result which is not an err
        while let Some(next_result) = result_iter.next() {
            iter_count += 1;
            match next_result {
                Ok(pending_values) => {
                    debug!("Found result, iteration {iter_count}");
                    for (p, v) in pending_values.iter() {
                        debug!("Step {counter}: Value in pending: {p}/{v}");
                        let pos_atom = PosAtom::new(((*p).clone(), *v));
                        let atom = Atom::pos(&pos_atom);
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
                    // We keep alternative match iterators which will be recovered in case of failure
                    self.alternative_match_iterators.push(result_iter);
                    found = true;
                    break;
                }
                Err(err) => {
                    debug!("Result with error {err} at iteration {iter_count}");
                    current_err = Some(err);
                }
            }
        }
        if !found {
            let errs = match current_err {
                Some(rbe_err) => vec![ValidatorError::RbeError(rbe_err)],
                None => {
                    debug!("No value found for node/shape where node = {node}, shape = {shape:?}. Current_err = empty");
                    Vec::new()
                }
            };
            Ok(Either::Left(errs))
        } else {
            Ok(Either::Right(vec![Reason::ShapePassed {
                node: node.clone(),
                shape: shape.clone(),
            }]))
        }
    }

    fn cnv_iri<S>(&self, iri: S::IRI) -> Pred
    where
        S: Query,
    {
        let iri = S::iri2iri_s(&iri);
        Pred::from(iri)
    }

    fn cnv_object<S>(&self, term: &S::Term) -> Node
    where
        S: Query,
    {
        let object = S::term_as_object(term);
        Node::from(object)
    }

    fn neighs<S>(&self, node: &Node, preds: Vec<IriS>, rdf: &S) -> Result<Neighs>
    where
        S: Query,
    {
        let node = self.get_rdf_node(node, rdf);
        let list: Vec<_> = preds.iter().map(|pred| S::iri_s2iri(pred)).collect();
        if let Ok(subject) = node.try_into() {
            let (outgoing_arcs, remainder) = rdf
                .outgoing_arcs_from_list(&subject, &list)
                .map_err(|e| self.cnv_err::<S>(e))?;
            let mut result = Vec::new();
            for (pred, values) in outgoing_arcs.into_iter() {
                for obj in values.into_iter() {
                    let iri = self.cnv_iri::<S>(pred.clone());
                    let object = self.cnv_object::<S>(&obj);
                    result.push((iri.clone(), object))
                }
            }
            let mut remainder_preds = Vec::new();
            for r in remainder {
                let iri_r = self.cnv_iri::<S>(r.clone());
                remainder_preds.push(iri_r)
            }
            Ok((result, remainder_preds))
        } else {
            todo!()
        }
    }

    fn cnv_err<S>(&self, _err: S::Err) -> ValidatorError
    where
        S: Query,
    {
        todo!()
    }

    fn get_rdf_node<S>(&self, node: &Node, _rdf: &S) -> S::Term
    where
        S: Query,
    {
        match node.as_object() {
            Object::Iri(iri) => {
                let i = S::iri_s2iri(iri);
                S::iri_as_term(i)
            }
            Object::BlankNode(_id) => {
                todo!()
            }
            Object::Literal(_lit) => {
                todo!()
            }
        }
    }

    pub fn insert_pending(&mut self, atom: &Atom) {
        self.pending.insert((*atom).clone());
    }
}
