use crate::Reason;
use crate::Reasons;
use crate::ValidatorConfig;
use crate::atom;
use crate::validator_error::*;
use either::Either;
use indexmap::IndexSet;
use iri_s::iri;
use itertools::Itertools;
use shex_ast::Expr;
use shex_ast::Node;
use shex_ast::Pred;
use shex_ast::ShapeLabelIdx;
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape::Shape;
use shex_ast::ir::shape_expr::ShapeExpr;
use srdf::BlankNode;
use srdf::Iri as _;
use srdf::{NeighsRDF, Object};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use tracing::debug;
use tracing::trace;

type Result<T> = std::result::Result<T, ValidatorError>;
type Atom = atom::Atom<(Node, ShapeLabelIdx)>;
type NegAtom = (Node, ShapeLabelIdx);
type PosAtom = (Node, ShapeLabelIdx);
type Neighs = (Vec<(Pred, Node)>, Vec<Pred>);
type ValidationResult = Either<Vec<ValidatorError>, Vec<Reason>>;
type Typing = HashSet<(Node, ShapeLabelIdx)>;

#[derive(Debug, Clone)]
pub struct Engine {
    checked: IndexSet<Atom>,
    pending: IndexSet<Atom>,
    config: ValidatorConfig,
    step_counter: usize,
    reasons: HashMap<PosAtom, Vec<Reason>>,
    errors: HashMap<NegAtom, Vec<ValidatorError>>,
}

impl Engine {
    pub fn new(config: &ValidatorConfig) -> Engine {
        Engine {
            checked: IndexSet::new(),
            pending: IndexSet::new(),
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

    pub(crate) fn validate_pending(
        &mut self,
        rdf: &impl NeighsRDF,
        schema: &SchemaIR,
    ) -> Result<()> {
        while let Some(atom) = self.pop_pending() {
            match atom.clone() {
                Atom::Pos((node, idx)) => {
                    let mut hyp = Vec::new();
                    match self.prove(&node, &idx, &mut hyp, schema, rdf)? {
                        Either::Right(reasons) => {
                            self.add_checked_pos(atom, reasons);
                        }
                        Either::Left(errors) => {
                            self.add_checked_neg(atom, errors);
                        }
                    }
                }
                Atom::Neg((_node, _idx)) => {
                    todo!()
                }
            }
        }
        Ok(())
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
        match atom.clone() {
            Atom::Neg(na) => {
                self.checked.insert(atom);
                self.add_errors(na, errors)
            }
            Atom::Pos(na) => {
                self.checked.insert(atom.negated());
                self.add_errors(na, errors)
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

    pub(crate) fn dep(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        schema: &SchemaIR,
        rdf: &impl NeighsRDF,
    ) -> Result<HashSet<(Node, ShapeLabelIdx)>> {
        if let Some(info) = schema.find_shape_idx(idx) {
            let se = info.expr();
            let mut dep = HashSet::new();

            // Search all direct references of the shape expression
            for idx in se.direct_references().iter() {
                dep.insert((node.clone(), *idx));
            }

            // Search all pairs (node1, idx1) in the shape expr referenced by idx such that there is a triple constraint (pred, ref)
            // and the neighbours of node are (pred, node1)
            let references = se.references(schema);
            // trace!("References in shape expr: {:?}", references);
            let preds = references.keys().cloned().collect::<Vec<_>>();
            let (neighs, _remainder) = self.neighs(node, preds, rdf)?;
            for (pred, neigh_node) in neighs {
                if let Some(idx_list) = references.get(&pred) {
                    for idx in idx_list {
                        dep.insert((neigh_node.clone(), *idx));
                    }
                } else {
                    debug!("No references found for predicate {pred}");
                }
            }
            trace!(
                "Dependencies of {node}@{idx} are: [{}]",
                dep.iter().map(|(n, i)| format!("{n}@{i}")).join(", ")
            );
            Ok(dep)
        } else {
            Err(ValidatorError::ShapeExprNotFound { idx: *idx })
        }
    }

    pub(crate) fn prove(
        &self,
        node: &Node,
        label: &ShapeLabelIdx,
        hyp: &mut Vec<(Node, ShapeLabelIdx)>,
        schema: &SchemaIR,
        rdf: &impl NeighsRDF,
    ) -> Result<ValidationResult> {
        // Implements algorithm presented in page 14 of this paper:
        // https://labra.weso.es/publication/2017_semantics-validation-shapes-schemas/
        debug!(
            "Proving {node}@{label} with hyp: [{}]",
            hyp.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );
        hyp.push((node.clone(), *label));
        let hyp_as_set: HashSet<(Node, ShapeLabelIdx)> = hyp
            .iter()
            .map(|(n, l)| (n.clone(), *l))
            .collect::<HashSet<_>>();
        let mut matched = HashSet::new();
        let candidates = self.dep(node, label, schema, rdf)?;
        let cleaned_candidates: HashSet<_> = candidates.difference(&hyp_as_set).cloned().collect();
        for (n1, l1) in cleaned_candidates {
            match self.prove(&n1, &l1, hyp, schema, rdf)? {
                Either::Right(_rs) => {
                    matched.insert((n1.clone(), l1));
                }
                Either::Left(_errors) => {
                    // Should we collect errors here?
                }
            }
        }
        let mut typing: HashSet<_> = matched.union(&hyp_as_set).cloned().collect();
        let result = self.check_node_idx(node, label, schema, rdf, &mut typing)?;
        hyp.pop();
        debug!(
            "{} {node}@{label} with result: {}, hyp: [{}]",
            if result.is_right() {
                "Proved"
            } else {
                "Failed to prove"
            },
            show_result(&result),
            hyp.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );
        Ok(result)
    }

    pub(crate) fn check_node_idx(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        schema: &SchemaIR,
        rdf: &impl NeighsRDF,
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult> {
        trace!(
            "Checking {node}@{idx}, typing: [{}]",
            typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );
        if let Some(info) = schema.find_shape_idx(idx) {
            let se = info.expr();
            let result = self.check_node_shape_expr(idx, node, se, schema, rdf, typing)?;
            tracing::debug!("Result of {node}@{idx} is: {}", show_result(&result),);
            Ok(result)
        } else {
            Err(ValidatorError::ShapeExprNotFound { idx: *idx })
        }
    }

    pub(crate) fn check_node_shape_expr(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        se: &ShapeExpr,
        schema: &SchemaIR,
        rdf: &impl NeighsRDF,
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult> {
        match se {
            ShapeExpr::ShapeAnd { exprs, .. } => {
                tracing::debug!(
                    "Checking node {node} with AND, typing: [{}]",
                    typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                );
                let mut reasons_collection = Vec::new();
                for e in exprs {
                    match self.check_node_ref(node, e, typing)? {
                        Either::Left(errors) => {
                            trace!(
                                "AND failed at component {e} for node {node}\nErrors: {}\nTyping: {}",
                                errors.iter().map(|err| format!("{err}")).join(", "),
                                typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                            );
                            return Ok(Either::Left(vec![ValidatorError::ShapeAndError {
                                shape_expr: *e,
                                node: Box::new(node.clone()),
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
            ShapeExpr::ShapeOr { exprs, .. } => {
                let mut errors_collection = Vec::new();
                for e in exprs {
                    match self.check_node_ref(node, e, typing)? {
                        Either::Left(errors) => {
                            trace!(
                                "OR branch {e} failed for node {node}\nErrors: {}\nTyping: {}",
                                errors.iter().map(|err| format!("{err}")).join(", "),
                                typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                            );
                            errors_collection.push((*e, ValidatorErrors::new(errors)));
                        }
                        Either::Right(reasons) => {
                            return Ok(Either::Right(vec![Reason::ShapeOrPassed {
                                shape_expr: *e,
                                node: node.clone(),
                                reasons: Reasons::new(reasons),
                            }]));
                        }
                    }
                }
                // If we didn't return inside the loop, all branches failed
                Ok(Either::Left(vec![ValidatorError::ShapeOrError {
                    shape_expr: Box::new(se.clone()),
                    node: Box::new(node.clone()),
                    errors: errors_collection.clone(),
                }]))
            }
            ShapeExpr::ShapeNot { expr, .. } => {
                let result = self.check_node_ref(node, expr, typing)?;
                match result {
                    Either::Left(errors) => Ok(Either::Right(vec![Reason::ShapeNotPassed {
                        node: node.clone(),
                        shape_expr: se.clone(),
                        errors_evidences: ValidatorErrors::new(errors),
                    }])),
                    Either::Right(reasons) => {
                        Ok(Either::Left(vec![ValidatorError::ShapeNotError {
                            node: Box::new(node.clone()),
                            shape_expr: Box::new(se.clone()),
                            reasons: Reasons::new(reasons),
                        }]))
                    }
                }
            }
            ShapeExpr::NodeConstraint(nc) => {
                match nc.cond().matches(node) {
                    Ok(_pending) => {
                        // We ignore pending nodes here, because node constraints are not expected to generate pending nodes
                        pass(Reason::NodeConstraintPassed {
                            node: node.clone(),
                            nc: nc.clone(),
                        })
                    }
                    Err(err) => fail(ValidatorError::RbeError(err)),
                }
            }
            ShapeExpr::Shape(shape) => {
                if shape.extends().is_empty() {
                    self.check_node_shape(idx, node, shape, schema, rdf, typing)
                } else {
                    self.check_node_shape_extends(idx, node, shape, schema, rdf, typing)
                }
            }
            ShapeExpr::External {} => {
                debug!("External shape expression encountered for node {node} with shape {se}");
                pass(Reason::ExternalPassed { node: node.clone() })
            }
            ShapeExpr::Ref { idx } => self.check_node_ref(node, idx, typing),
            ShapeExpr::Empty => pass(Reason::EmptyPassed { node: node.clone() }),
        }
    }

    fn check_node_ref(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult> {
        debug!("Checking node {node} with shape ref {idx}");
        {
            // If the node is already in the typing, we can return true
            if typing.contains(&(node.clone(), *idx)) {
                pass(Reason::ShapeRefPassed {
                    node: node.clone(),
                    idx: *idx,
                })
            } else {
                fail(ValidatorError::ShapeRefFailed {
                    node: Box::new(node.clone()),
                    idx: *idx,
                })
            }
        }
    }

    pub(crate) fn check_node_shape(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        shape: &Shape,
        _schema: &SchemaIR,
        rdf: &impl NeighsRDF,
        typing: &mut Typing,
    ) -> Result<ValidationResult> {
        debug!("check_node_shape: node = {node}, shape = {idx} [No extends]");
        let (values, remainder) = self.neighs(node, shape.preds(), rdf)?;
        if shape.is_closed() && !remainder.is_empty() {
            trace!("Closed shape with remainder preds: {remainder:?}");
            fail(ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(remainder),
                declared: Preds::new(shape.preds().into_iter().collect()),
            })
        } else {
            check_expr_neigh(shape.triple_expr(), &values, node, shape, typing)
        }
    }

    pub(crate) fn check_node_shape_extends(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        shape: &Shape,
        schema: &SchemaIR,
        rdf: &impl NeighsRDF,
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult> {
        debug!("check_node_shape_extends: node={node}, shape={idx}");
        let preds_extends = Vec::from_iter(schema.get_preds_extends(idx));
        trace!(
            "Predicates in this shape with extends: [{}]",
            preds_extends.iter().map(|p| p.to_string()).join(", ")
        );
        let (values, remainder) = self.neighs(node, preds_extends, rdf)?;
        if shape.is_closed() && !remainder.is_empty() {
            trace!("Closed shape with remainder preds: {remainder:?}");
            return fail(ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(remainder),
                declared: Preds::new(shape.preds().into_iter().collect()),
            });
        }
        trace!(
            "Neighs of {node} [{}]",
            values.iter().map(|(p, v)| format!("{p} {v}")).join(", ")
        );
        let triple_exprs = schema.get_triple_exprs(idx).unwrap();
        debug!(
            "Candidate triple exprs of {node} [{}]",
            triple_exprs
                .iter()
                .map(|(maybe_label, te)| format!(
                    "{} -> [{}]",
                    maybe_label
                        .map(|l| l.to_string())
                        .unwrap_or("[]".to_string()),
                    te.iter().map(|p| p.show_rbe_simplified()).join(", ")
                ))
                .join("| ")
        );

        let parts_iter = crate::partitions_iter(&values, &triple_exprs);
        for (npart, partition) in parts_iter.enumerate() {
            debug!("Partition {npart}: {}", show_partition(&partition));
            let mut ok_partition = true;
            for (maybe_label, rbes, neighs_subset) in partition.iter() {
                debug!(
                    " Part {npart}| Trying component: {}, neighs [{}] ",
                    maybe_label
                        .map(|l| l.to_string())
                        .unwrap_or("[]".to_string()),
                    neighs_subset
                        .iter()
                        .map(|(p, v)| format!("{p} {v}"))
                        .join(", ")
                );
                let result = check_exprs_neigh(rbes, neighs_subset, node, shape, typing)?;
                if result.is_right() {
                    // TODO: Accumulate reasons from each triple expr
                    debug!(
                        " Part {npart}| Success component {}: neighs {}",
                        maybe_label
                            .map(|l| l.to_string())
                            .unwrap_or("[]".to_string()),
                        neighs_subset
                            .iter()
                            .map(|(p, v)| format!("{p} {v}"))
                            .join(", ")
                    );
                } else {
                    ok_partition = false;
                    debug!(
                        " Part {npart}| Failed component {}: neighs {}",
                        maybe_label
                            .map(|l| l.to_string())
                            .unwrap_or("[]".to_string()),
                        neighs_subset
                            .iter()
                            .map(|(p, v)| format!("{p} {v}"))
                            .join(", ")
                    );
                    break;
                }
                // If the partition failed, we search another one
            }
            if ok_partition {
                debug!(" Part {npart}| Partition succeeded",);
                return pass(Reason::ShapeExtendsPassed {
                    node: node.clone(),
                    shape: shape.clone(),
                    reasons: Reasons::new(vec![]), // TODO: Collect reasons from each triple expr
                });
            }
        }
        debug!(" Failed shape {idx}. All partitions failed",);
        fail(ValidatorError::ShapeFails {
            node: Box::new(node.clone()),
            shape: Box::new(shape.clone()),
            errors: Vec::new(),
        })
    }

    fn cnv_iri<S>(&self, iri: S::IRI) -> Pred
    where
        S: NeighsRDF,
    {
        let iri_string = iri.as_str();
        let iri_s = iri!(iri_string);
        Pred::from(iri_s)
    }

    fn cnv_object<S>(&self, term: &S::Term) -> Result<Node>
    where
        S: NeighsRDF,
    {
        let obj = term
            .clone()
            .try_into()
            .map_err(|_| ValidatorError::TermToRDFNodeFailed {
                term: format!("{term}"),
            })?;
        Ok(Node::from(obj))
    }

    /// Get the neighbours of a node for a list of predicates
    /// Returns a tuple (values, remainder) where:
    /// - values is a list of pairs (pred, obj) where obj is an object of the node for the given pred
    /// - remainder is a list of predicates for which there are no objects
    pub(crate) fn neighs<S>(&self, node: &Node, preds: Vec<Pred>, rdf: &S) -> Result<Neighs>
    where
        S: NeighsRDF,
    {
        let node = self.get_rdf_node(node, rdf);
        let list: Vec<_> = preds.iter().map(|pred| pred.iri().clone().into()).collect();
        if let Ok(subject) = S::term_as_subject(&node) {
            let (outgoing_arcs, remainder) = rdf
                .outgoing_arcs_from_list(&subject, &list)
                .map_err(|e| self.cnv_err::<S>(e))?;
            let mut result = Vec::new();
            for (pred, values) in outgoing_arcs.into_iter() {
                for obj in values.into_iter() {
                    let iri = self.cnv_iri::<S>(pred.clone());
                    let object = self.cnv_object::<S>(&obj)?;
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
            Ok((Vec::new(), Vec::new()))
        }
    }

    fn cnv_err<S>(&self, _err: S::Err) -> ValidatorError
    where
        S: NeighsRDF,
    {
        todo!()
    }

    fn get_rdf_node<S>(&self, node: &Node, _rdf: &S) -> S::Term
    where
        S: NeighsRDF,
    {
        match node.as_object() {
            Object::Iri(iri_s) => {
                let iri: S::IRI = iri_s.clone().into();
                iri.into().into()
            }
            Object::BlankNode(id) => {
                let bnode: S::BNode = BlankNode::new(id);
                bnode.into()
            }
            Object::Literal(lit) => {
                let lit: S::Literal = lit.clone().into();
                let term: S::Term = lit.into();
                term
            }
            Object::Triple { .. } => todo!(),
        }
    }

    pub fn insert_pending(&mut self, atom: &Atom) {
        self.pending.insert((*atom).clone());
    }
}

fn pass(reason: Reason) -> Result<ValidationResult> {
    Ok(Either::Right(vec![reason]))
}

fn fail(err: ValidatorError) -> Result<ValidationResult> {
    Ok(Either::Left(vec![err]))
}

fn show_result(result: &Either<Vec<ValidatorError>, Vec<Reason>>) -> String {
    match result {
        Either::Left(errors) => format!(
            "False with errors: {}",
            errors.iter().map(|e| e.to_string()).join(", ")
        ),
        Either::Right(reasons) => format!(
            "True with reasons: {}",
            reasons.iter().map(|r| r.to_string()).join(", ")
        ),
    }
}

fn check_exprs_neigh(
    exprs: &Vec<Expr>,
    neighs: &Vec<(Pred, Node)>,
    node: &Node,
    shape: &Shape,
    typing: &Typing,
) -> Result<ValidationResult> {
    for rbe in exprs.iter() {
        let result = check_expr_neigh(rbe, neighs, node, shape, typing)?;
        if result.is_left() {
            return fail(ValidatorError::ShapeFails {
                node: Box::new(node.clone()),
                shape: Box::new(shape.clone()),
                errors: result.left().unwrap().clone(),
            });
        }
    }
    pass(Reason::ShapePassed {
        node: node.clone(),
        shape: Box::new(shape.clone()),
    })
}

fn check_expr_neigh(
    expr: &Expr,
    neighs: &Vec<(Pred, Node)>,
    node: &Node,
    shape: &Shape,
    typing: &Typing,
) -> Result<ValidationResult> {
    debug!(
        "Checking expr {} with neighs: [{}]",
        expr,
        neighs.iter().map(|(p, o)| format!("{p} {o}")).join(", ")
    );
    let result_iter = expr.matches(neighs.clone())?;
    let mut errors = Vec::new();
    for result in result_iter {
        debug!(
            "Result of {expr} with neighs: {}: {:?}",
            neighs.iter().map(|(p, o)| format!("{p} {o}")).join(", "),
            result
        );
        match result {
            Ok(pending_values) => {
                let mut failed_pending = Vec::new();
                // Check if all pending values are in typing
                for (n, idx) in pending_values.iter() {
                    let pair = (n.clone(), *idx);
                    if !typing.contains(&pair) {
                        failed_pending.push(pair)
                        // TODO: if (stop_at_first) break
                        // We don't need to compute all the failed pending values once we find the first pair
                    }
                }
                if failed_pending.is_empty() {
                    return pass(Reason::ShapePassed {
                        node: node.clone(),
                        shape: Box::new(shape.clone()),
                    });
                } else {
                    errors.push(ValidatorError::FailedPending {
                        failed_pending: failed_pending.clone(),
                    })
                }
            }
            Err(err) => {
                debug!("Result with error: {err}");
                return fail(ValidatorError::RbeError(err));
            }
        }
    }
    debug!(
        "expr failed {expr} with neighs: [{}]",
        neighs.iter().map(|(p, o)| format!("{p} {o}")).join(", ")
    );
    fail(ValidatorError::ShapeFails {
        node: Box::new(node.clone()),
        shape: Box::new(shape.clone()),
        errors,
    })
}

fn show_partition(
    partition: &Vec<(Option<ShapeLabelIdx>, Vec<Expr>, Vec<(Pred, Node)>)>,
) -> String {
    partition
        .iter()
        .map(|(maybe_label, _rbes, neighs_subset)| {
            let label_str = maybe_label
                .map(|l| l.to_string())
                .unwrap_or("[]".to_string());
            let neighs_str = neighs_subset
                .iter()
                .map(|(p, o)| format!("{p} {o}"))
                .join(", ");
            format!("{} -> [{}]", label_str, neighs_str)
        })
        .join(" | ")
}
