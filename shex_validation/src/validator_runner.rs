use crate::atom;
use crate::validator_error::*;
use crate::Reason;
use crate::Reasons;
use crate::ValidatorConfig;
use either::Either;
use indexmap::IndexSet;
use iri_s::iri;
use iri_s::IriS;
use rbe::MatchTableIter;
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape::Shape;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::Node;
use shex_ast::Pred;
use shex_ast::ShapeLabelIdx;
use srdf::Iri as _;
use srdf::{Object, Query};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use tracing::debug;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;
type NegAtom = (Node, ShapeLabelIdx);
type PosAtom = (Node, ShapeLabelIdx);
// type Rule = rule::Rule<(Node, ShapeLabelIdx)>;
type Neighs = (Vec<(Pred, Node)>, Vec<Pred>);

#[derive(Debug, Clone)]
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

impl Engine {
    pub fn new(config: &ValidatorConfig) -> Engine {
        Engine {
            checked: IndexSet::new(),
            processing: IndexSet::new(),
            pending: IndexSet::new(),
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

    /*pub(crate) fn get_result(&self, atom: &Atom) -> ResultValue {
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
    }*/

    pub fn new_step(&mut self) {
        self.step_counter += 1;
    }

    pub fn add_ok(&mut self, n: Node, s: ShapeLabelIdx) {
        let pa = (n, s);
        self.checked.insert(Atom::pos(&pa));
    }

    pub fn add_failed(&mut self, n: Node, s: ShapeLabelIdx, err: ValidatorError) {
        let atom = (n, s);
        self.checked.insert(Atom::neg(&atom));
        match self.errors.entry(atom) {
            Entry::Occupied(mut es) => es.get_mut().push(err),
            Entry::Vacant(vacant) => {
                vacant.insert(vec![err]);
            }
        }
    }

    pub fn get_shape_expr(&self, _idx: ShapeLabelIdx) -> &ShapeExpr {
        // self.config.get_shape_expr(idx)
        todo!()
    }

    pub fn more_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    pub fn add_pending(&mut self, n: Node, s: ShapeLabelIdx) {
        let pos_atom = (n, s);
        self.pending.insert(Atom::pos(&pos_atom));
    }

    pub fn pop_pending(&mut self) -> Option<Atom> {
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
        schema: &SchemaIR,
    ) -> Result<Either<Vec<ValidatorError>, Vec<Reason>>>
    where
        S: Query,
    {
        debug!(
            "Step {}. Checking node {node} with shape_expr: {se}",
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
            ShapeExpr::Ref { idx } => {
                // TODO: Should we remove the next
                self.add_pending(node.clone(), *idx);
                if let Some((_maybe_label, se)) = schema.find_shape_idx(idx) {
                    self.check_node_shape_expr(node, se, rdf, schema)
                } else {
                    Ok(Either::Left(vec![ValidatorError::ShapeExprNotFound {
                        idx: *idx,
                    }]))
                }
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                let mut reasons_collection = Vec::new();
                for e in exprs {
                    let result = self.check_node_shape_expr(node, e, rdf, schema)?;
                    match result {
                        Either::Left(errors) => {
                            return Ok(Either::Left(vec![ValidatorError::ShapeAndError {
                                shape_expr: e.clone(),
                                node: node.clone(),
                                errors: ValidatorErrors::new(errors),
                            }]));
                        }
                        Either::Right(reasons) => {
                            reasons_collection.push(reasons);
                        }
                    }
                }
                Ok(Either::Right(vec![Reason::ShapeAndPassed {
                    node: node.clone(),
                    se: se.clone(),
                    reasons: reasons_collection,
                }]))
            }
            ShapeExpr::ShapeNot { expr, .. } => {
                let result = self.check_node_shape_expr(node, expr, rdf, schema)?;
                match result {
                    Either::Left(errors) => Ok(Either::Right(vec![Reason::ShapeNotPassed {
                        node: node.clone(),
                        shape_expr: se.clone(),
                        errors_evidences: ValidatorErrors::new(errors),
                    }])),
                    Either::Right(reasons) => {
                        Ok(Either::Left(vec![ValidatorError::ShapeNotError {
                            node: node.clone(),
                            shape_expr: se.clone(),
                            reasons: Reasons::new(reasons),
                        }]))
                    }
                }
            }
            ShapeExpr::ShapeOr { exprs, .. } => {
                let mut errors_collection = Vec::new();
                for e in exprs {
                    let result = self.check_node_shape_expr(node, e, rdf, schema)?;
                    match result {
                        Either::Left(errors) => {
                            errors_collection.push((e.clone(), ValidatorErrors::new(errors)));
                        }
                        Either::Right(reasons) => {
                            return Ok(Either::Right(vec![Reason::ShapeOrPassed {
                                shape_expr: e.clone(),
                                node: node.clone(),
                                reasons: Reasons::new(reasons),
                            }]));
                        }
                    }
                }
                Ok(Either::Left(vec![ValidatorError::ShapeOrError {
                    shape_expr: se.clone(),
                    node: node.clone(),
                    errors: errors_collection.clone(),
                }]))
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
                        let pos_atom = ((*p).clone(), *v);
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
        let iri_string = iri.as_str();
        let iri_s = iri!(iri_string);
        Pred::from(iri_s)
    }

    fn cnv_object<S>(&self, term: &S::Term) -> Node
    where
        S: Query,
    {
        Node::from(term.clone().into())
    }

    fn neighs<S>(&self, node: &Node, preds: Vec<IriS>, rdf: &S) -> Result<Neighs>
    where
        S: Query,
    {
        let node = self.get_rdf_node(node, rdf);
        let list: Vec<_> = preds.iter().map(|pred| pred.clone().into()).collect();
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
            Object::Iri(iri_s) => {
                let iri: S::IRI = iri_s.clone().into();
                iri.into()
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
