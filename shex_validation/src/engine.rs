use crate::PartitionDisplay;
use crate::PartitionsDisplay;
use crate::Reason;
use crate::Reasons;
use crate::ValidatorConfig;
use crate::ValidatorErrors;
use crate::atom;
use crate::validator_error::*;
use either::Either;
use indexmap::IndexSet;
use itertools::Itertools;
// use prefixmap::PrefixMap;
use rbe::MatchCond;
use rudof_iri::iri;
use rudof_rdf::rdf_core::{
    NeighsRDF,
    query::QueryRDF,
    term::{BlankNode, Iri as _, Object},
};
use shex_ast::Expr;
use shex_ast::Node;
use shex_ast::Pred;
use shex_ast::ShapeLabelIdx;
use shex_ast::ir::external_resolver::{DispatchOutcome, ExternalResolveCtx};
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::sem_act::SemAct;
use shex_ast::ir::semantic_action_context::SemanticActionContext;
use shex_ast::ir::shape::Shape;
use shex_ast::ir::shape_expr::ShapeExpr;
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
/// Tracks which `(Node, ShapeLabelIdx)` pairs have already been proved (`passed`),
/// and, for pairs whose proof was attempted and failed, the errors that caused
/// the failure. This lets callers that consume a failed pending reference (e.g.
/// `FailedPending`) explain *why* the reference failed, not just that it did.
#[derive(Debug, Clone, Default)]
pub(crate) struct RefTyping {
    passed: HashSet<(Node, ShapeLabelIdx)>,
    errors: HashMap<(Node, ShapeLabelIdx), Vec<ValidatorError>>,
}

impl RefTyping {
    fn new() -> Self {
        RefTyping::default()
    }

    fn contains(&self, pair: &(Node, ShapeLabelIdx)) -> bool {
        self.passed.contains(pair)
    }

    fn insert_passed(&mut self, node: Node, idx: ShapeLabelIdx) {
        self.passed.insert((node, idx));
    }

    fn insert_failed(&mut self, node: Node, idx: ShapeLabelIdx, errors: Vec<ValidatorError>) {
        self.errors.entry((node, idx)).or_default().extend(errors);
    }

    fn errors_for(&self, pair: &(Node, ShapeLabelIdx)) -> Vec<ValidatorError> {
        self.errors.get(pair).cloned().unwrap_or_default()
    }
}

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

    pub(crate) fn validate_pending<R>(&mut self, rdf: &R, schema: &SchemaIR) -> Result<()>
    where
        R: NeighsRDF + QueryRDF,
    {
        while let Some(atom) = self.pop_pending() {
            match atom.clone() {
                Atom::Pos((node, idx)) => {
                    if !check_start_acts(schema.start_acts(), &node, &idx, schema)? {
                        self.add_checked_neg(
                            atom.clone(),
                            vec![ValidatorError::StartActFailed {
                                node: Box::new(node.clone()),
                                idx,
                            }],
                        );
                        // We can abort validation if start actions failed
                        continue;
                    }
                    let mut hyp = Vec::new();
                    match self.prove(&node, &idx, &mut hyp, schema, rdf)? {
                        Either::Right(reasons) => {
                            self.add_checked_pos(atom, reasons);
                        },
                        Either::Left(errors) => {
                            self.add_checked_neg(atom, errors);
                        },
                    }
                },
                Atom::Neg((node, idx)) => {
                    todo!("Handle the case where we have a pending negative atom. Node: {node}, shape idx: {idx}")
                },
            }
        }
        Ok(())
    }

    pub(crate) fn add_checked_pos(&mut self, atom: Atom, reasons: Vec<Reason>) {
        let new_atom = atom.clone();
        match atom {
            Atom::Pos(positive_atom) => {
                self.checked.insert(new_atom);
                self.add_reasons(positive_atom, reasons)
            },
            Atom::Neg(negative_atom) => {
                todo!("Handle the case where we have a checked negative atom {negative_atom:?}")
            },
        }
    }

    pub(crate) fn add_checked_neg(&mut self, atom: Atom, errors: Vec<ValidatorError>) {
        match atom.clone() {
            Atom::Neg(na) => {
                self.checked.insert(atom);
                self.add_errors(na, errors)
            },
            Atom::Pos(na) => {
                self.checked.insert(atom.negated());
                self.add_errors(na, errors)
            },
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
            },
        }
    }

    fn add_errors(&mut self, na: NegAtom, es: Vec<ValidatorError>) {
        match self.errors.entry(na) {
            Entry::Occupied(mut vs) => vs.get_mut().extend(es),
            Entry::Vacant(vac) => {
                vac.insert(es);
            },
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

    // TODO: We may remove this method
    fn _add_failed(&mut self, n: Node, s: ShapeLabelIdx, err: ValidatorError) {
        let atom = (n, s);
        self.checked.insert(Atom::neg(&atom));
        match self.errors.entry(atom) {
            Entry::Occupied(mut es) => es.get_mut().push(err),
            Entry::Vacant(vacant) => {
                vacant.insert(vec![err]);
            },
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

    /// Returns the set of pairs `(node, shape_idx)` that are dependencies of `node@idx`,
    /// i.e. all pairs `(node1, idx1)` such that:
    /// - `node1@idx1` is a direct reference in the shape expression of `idx`, or
    /// - there is a triple constraint `(pred, ref)` in the shape expression of `idx` and
    ///   the neighbours of `node` are `(pred, node1)`
    pub(crate) fn dep<R>(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        schema: &SchemaIR,
        rdf: &R,
    ) -> Result<HashSet<(Node, ShapeLabelIdx)>>
    where
        R: QueryRDF + NeighsRDF,
    {
        if let Some(info) = schema.find_shape_idx(idx) {
            let se = info.expr();
            let mut dep = HashSet::new();

            // Search all direct references of the shape expression
            for idx in se.direct_references().iter() {
                dep.insert((node.clone(), *idx));
            }

            // Search all pairs `(node1, idx1)` in the shape expr referenced by `idx` such that there is a triple constraint `(pred, ref)`
            // and the neighbours of `node` are `(pred, node1)`
            let references = se.references(schema);
            // trace!("References in shape expr: {:?}", references);
            let preds = references.keys().cloned().collect::<Vec<_>>();
            let (neighs, _) = self.neighs(node, preds, rdf)?;
            for (pred, neigh_node) in neighs {
                if let Some(idx_list) = references.get(&pred) {
                    for idx in idx_list {
                        dep.insert((neigh_node.clone(), *idx));
                    }
                } else {
                    /*debug!("No references found for predicate {pred}");*/
                }
            }
            /*trace!(
                "Dependencies of {node}@{idx} are: [{}]",
                dep.iter().map(|(n, i)| format!("{n}@{i}")).join(", ")
            );*/
            Ok(dep)
        } else {
            Err(ValidatorError::ShapeExprNotFound { idx: *idx })
        }
    }

    // Implements algorithm presented in page 14 of this paper:
    // https://labra.weso.es/publication/2017_semantics-validation-shapes-schemas/
    pub(crate) fn prove<R>(
        &self,
        node: &Node,
        label: &ShapeLabelIdx,
        hyp: &mut Vec<(Node, ShapeLabelIdx)>,
        schema: &SchemaIR,
        rdf: &R,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        hyp.push((node.clone(), *label));
        let hyp_as_set: HashSet<(Node, ShapeLabelIdx)> =
            hyp.iter().map(|(n, l)| (n.clone(), *l)).collect::<HashSet<_>>();
        let mut typing = RefTyping::new();
        let candidates = self.dep(node, label, schema, rdf)?;
        let cleaned_candidates: HashSet<_> = candidates.difference(&hyp_as_set).cloned().collect();
        for (n1, l1) in cleaned_candidates {
            match self.prove(&n1, &l1, hyp, schema, rdf)? {
                Either::Right(_reasons) => {
                    /*debug!(
                        "Proved {n1}@{l1} while proving {node}@{label}: {}",
                        rs.iter().map(|r| format!("{r}")).join(", ")
                    );*/
                    typing.insert_passed(n1.clone(), l1);
                },
                Either::Left(errors) => {
                    /*debug!(
                        "Failed to prove {n1}@{l1} while proving {node}@{label}: {}",
                        errors.iter().map(|e| format!("{e}")).join(", ")
                    );*/
                    typing.insert_failed(n1.clone(), l1, errors);
                },
            }
        }
        for (n, l) in hyp_as_set.iter() {
            typing.insert_passed(n.clone(), *l);
        }
        let result = self.check_node_idx(node, label, schema, rdf, &mut typing, hyp)?;
        hyp.pop();
        /*debug!(
            "{} {node}@{label} with result: {}, hyp: [{}]",
            if result.is_right() { "Proved" } else { "Failed to prove" },
            show_result(
                &result,
                &rdf.prefixmap().unwrap_or_default(),
                schema,
                self.config.width()
            )?,
            hyp.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );*/
        Ok(result)
    }

    pub(crate) fn check_node_idx<R>(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        schema: &SchemaIR,
        rdf: &R,
        typing: &mut RefTyping,
        hyp: &mut Vec<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        /*trace!(
            "Checking {node}@{idx}, typing: [{}]",
            typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );*/
        if let Some(info) = schema.find_shape_idx(idx) {
            if schema.is_abstract(idx) {
                let descendants = schema.descendants(idx);
                if descendants.is_empty() {
                    return Err(ValidatorError::AbstractShapeNoDescendants { idx: *idx });
                }
                // Each descendant gets its own prove call so it builds a fresh typing
                // with all the dependencies it actually needs (RESTRICTS semantics).
                let descendants_result = self.check_descendants(node, idx, descendants, schema, rdf, hyp)?;
                if descendants_result.is_right() {
                    Ok(descendants_result)
                } else {
                    // Indicate that the main shape and its descendants failed
                    Ok(Either::Left(vec![ValidatorError::AbstractShapeError {
                        idx: *idx,
                        node: Box::new(node.clone()),
                        errors: ValidatorErrors::new(descendants_result.left().unwrap().to_vec()),
                    }]))
                }
            } else {
                let se = info.expr();
                let result = self.check_node_shape_expr(idx, node, se, schema, rdf, typing)?;
                /*tracing::debug!(
                    "Result of {node}@{idx} is: {}",
                    show_result(
                        &result,
                        &rdf.prefixmap().unwrap_or_default(),
                        schema,
                        self.config.width()
                    )?,
                );*/
                if result.is_right() {
                    Ok(result)
                } else {
                    let descendants = schema.descendants(idx);
                    if descendants.is_empty() {
                        return Ok(result);
                    }
                    // Each descendant gets its own prove call so it builds a fresh typing
                    // with all the dependencies it actually needs (RESTRICTS semantics).
                    let descendants_result = self.check_descendants(node, idx, descendants, schema, rdf, hyp)?;
                    if descendants_result.is_right() {
                        Ok(descendants_result)
                    } else {
                        // Indicate that the main shape and its descendants failed
                        let errors_descendants = descendants_result.left().unwrap();
                        let errors_result = result.left().unwrap();
                        let all_errors = errors_result
                            .iter()
                            .cloned()
                            .chain(errors_descendants.iter().cloned())
                            .collect::<Vec<_>>();
                        Ok(Either::Left(all_errors))
                    }
                }
            }
        } else {
            Err(ValidatorError::ShapeExprNotFound { idx: *idx })
        }
    }

    fn check_descendants<R>(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        descendants: Vec<ShapeLabelIdx>,
        schema: &SchemaIR,
        rdf: &R,
        hyp: &mut Vec<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        let mut errors_collection = Vec::new();
        for desc in descendants {
            // Use prove (not check_node_idx) so each descendant builds its own complete
            // typing from its own dependency set.  This implements the ShEx RESTRICTS
            // property: if the node conforms to a descendant, it conforms to the ancestor.
            match self.prove(node, &desc, hyp, schema, rdf)? {
                Either::Left(errors) => {
                    trace!(
                        "Descendant {desc} failed for node {node}\nErrors: {}",
                        errors.iter().map(|err| format!("{err}")).join(", "),
                    );
                    errors_collection.push(ValidatorError::DescendantShapeError {
                        current: *idx,
                        desc,
                        node: Box::new(node.clone()),
                        errors: ValidatorErrors::new(errors),
                    });
                },
                Either::Right(reasons) => {
                    return Ok(Either::Right(vec![Reason::DescendantShape {
                        node: node.clone(),
                        shape: *idx,
                        reasons: Reasons::new(reasons.clone()),
                    }]));
                },
            }
        }
        Ok(Either::Left(vec![ValidatorError::DescendantsShapeError {
            idx: *idx,
            node: Box::new(node.clone()),
            errors: ValidatorErrors::new(errors_collection),
        }]))
    }

    pub(crate) fn check_node_shape_expr<R>(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        se: &ShapeExpr,
        schema: &SchemaIR,
        rdf: &R,
        typing: &mut RefTyping,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        match se {
            ShapeExpr::ShapeAnd { exprs, .. } => {
                /*tracing::debug!(
                    "Checking node {node} with AND, typing: [{}]",
                    typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                );*/
                let mut reasons_collection = Vec::new();
                for e in exprs {
                    match self.check_node_ref(node, e, typing)? {
                        Either::Left(errors) => {
                            /*trace!(
                                "AND failed at component {e} for node {node}\nErrors: {}\nTyping: {}",
                                errors.iter().map(|err| format!("{err}")).join(", "),
                                typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                            );*/
                            return Ok(Either::Left(vec![ValidatorError::ShapeAndError {
                                shape_expr: *e,
                                node: Box::new(node.clone()),
                                errors: ValidatorErrors::new(errors),
                            }]));
                        },
                        Either::Right(reasons) => {
                            reasons_collection.push(reasons);
                        },
                    }
                }
                Ok(Either::Right(vec![Reason::ShapeAnd {
                    node: node.clone(),
                    se: Box::new(se.clone()),
                    reasons: reasons_collection,
                }]))
            },
            ShapeExpr::ShapeOr { exprs, .. } => {
                let mut errors_collection = Vec::new();
                for e in exprs {
                    match self.check_node_ref(node, e, typing)? {
                        Either::Left(errors) => {
                            /*trace!(
                                "OR branch {e} failed for node {node}\nErrors: {}\nTyping: {}",
                                errors.iter().map(|err| format!("{err}")).join(", "),
                                typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                            );*/
                            errors_collection.push((*e, ValidatorErrors::new(errors)));
                        },
                        Either::Right(reasons) => {
                            return Ok(Either::Right(vec![Reason::ShapeOr {
                                shape_expr: *e,
                                node: node.clone(),
                                reasons: Reasons::new(reasons),
                            }]));
                        },
                    }
                }
                // If we didn't return inside the loop, all branches failed
                Ok(Either::Left(vec![ValidatorError::ShapeOrError {
                    shape_expr: Box::new(se.clone()),
                    node: Box::new(node.clone()),
                    errors: errors_collection.clone(),
                }]))
            },
            ShapeExpr::ShapeNot { expr, .. } => {
                let result = self.check_node_ref(node, expr, typing)?;
                match result {
                    Either::Left(errors) => Ok(Either::Right(vec![Reason::ShapeNot {
                        node: node.clone(),
                        shape_expr: se.clone(),
                        errors_evidences: ValidatorErrors::new(errors),
                    }])),
                    Either::Right(reasons) => Ok(Either::Left(vec![ValidatorError::ShapeNotError {
                        node: Box::new(node.clone()),
                        shape_expr: Box::new(se.clone()),
                        reasons: Reasons::new(reasons),
                    }])),
                }
            },
            ShapeExpr::NodeConstraint(nc) => {
                // TODO: In the case of a node constraint...is the context only the subject?
                let ctx = SemanticActionContext::subject(node);
                match nc.cond().matches(node, &ctx) {
                    Ok(_pending) => {
                        // We ignore pending nodes here, as node constraints are not expected to generate pending nodes
                        pass(Reason::NodeConstraint {
                            node: node.clone(),
                            nc: nc.clone(),
                        })
                    },
                    Err(err) => fail(ValidatorError::RbeError(err)),
                }
            },
            ShapeExpr::Shape(shape) => {
                if shape.extends().is_empty() {
                    self.check_node_shape(idx, node, shape, schema, rdf, typing)
                } else {
                    self.check_node_shape_extends(idx, node, shape, schema, rdf, typing)
                }
            },
            ShapeExpr::External {} => {
                let label = schema.shape_label_from_idx(idx);
                let ctx = ExternalResolveCtx {
                    node,
                    shape_idx: *idx,
                    shape_label: label,
                    schema,
                };
                match self.config.external_resolvers().dispatch(&ctx) {
                    DispatchOutcome::Conformant { resolver, rationale } => {
                        debug!("EXTERNAL conformant for {node}@{idx} via '{resolver}': {rationale}");
                        pass(Reason::External {
                            node: node.clone(),
                            resolver,
                            rationale,
                        })
                    },
                    DispatchOutcome::NonConformant { resolver, rationale } => {
                        debug!("EXTERNAL non-conformant for {node}@{idx} via '{resolver}': {rationale}");
                        fail(ValidatorError::ExternalShapeRejected {
                            node: Box::new(node.clone()),
                            idx: *idx,
                            resolver,
                            rationale,
                        })
                    },
                    DispatchOutcome::Abstain => {
                        debug!("EXTERNAL unresolved for {node}@{idx}: no resolver answered");
                        fail(ValidatorError::ExternalShapeUnresolved {
                            node: Box::new(node.clone()),
                            idx: *idx,
                        })
                    },
                }
            },
            ShapeExpr::Ref { idx } => self.check_node_ref(node, idx, typing),
            ShapeExpr::Empty => pass(Reason::Empty { node: node.clone() }),
        }
    }

    fn check_node_ref(&self, node: &Node, idx: &ShapeLabelIdx, typing: &mut RefTyping) -> Result<ValidationResult> {
        /*debug!("Checking node {node} with shape ref {idx}"); */

        // If the node is already in the typing, we can return true
        if typing.contains(&(node.clone(), *idx)) {
            pass(Reason::ShapeRef {
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

    pub(crate) fn check_node_shape<R>(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        shape: &Shape,
        _schema: &SchemaIR,
        rdf: &R,
        typing: &mut RefTyping,
    ) -> Result<ValidationResult>
    where
        R: QueryRDF + NeighsRDF,
    {
        // trace!("check_node_shape: node = {node}, shape = {idx} [No extends]");
        let extra_preds = shape.extra().clone();
        let mut candidate_preds = shape.preds();
        for p in &extra_preds {
            if !candidate_preds.contains(p) {
                candidate_preds.push(p.clone());
            }
        }
        let (values, reminder) = self.neighs(node, candidate_preds.clone(), rdf)?;
        let values_ctx = values
            .iter()
            .map(|(p, v)| (p.clone(), v.clone(), SemanticActionContext::triple(node, p, v)))
            .filter(|(pred, value, ctx)| {
                // Strict predicates (not in EXTRA) always go into M^∈.
                if !extra_preds.contains(pred) {
                    return true;
                }
                // Lenient predicates (in EXTRA): only values that satisfy a leaf condition
                // participate in the RBE; non-matching values fall into M^∉ and are ignored.
                matches_any_leaf(shape.triple_expr(), pred, value, ctx)
            })
            .collect::<Vec<_>>();
        if shape.is_closed() && !reminder.is_empty() {
            return fail(ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(reminder),
                declared: Preds::new(candidate_preds),
            });
        }
        check_expr_neigh(shape.triple_expr(), &values_ctx, node, shape, idx, typing)
    }

    pub(crate) fn check_node_shape_extends<R>(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        shape: &Shape,
        schema: &SchemaIR,
        rdf: &R,
        typing: &mut RefTyping,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        // trace!("check_node_shape_extends: node={node}, shape={idx}");
        let extra_preds = shape.extra().clone();
        let mut candidate_preds = Vec::from_iter(schema.get_preds_extends(idx));
        for p in &extra_preds {
            if !candidate_preds.contains(p) {
                candidate_preds.push(p.clone());
            }
        }
        /*trace!(
            "Predicates in this shape with extends: [{}]",
            candidate_preds.iter().map(|p| p.to_string()).join(", ")
        );*/
        let (values, reminder) = self.neighs(node, candidate_preds.clone(), rdf)?;

        if shape.is_closed() && !reminder.is_empty() {
            /*debug!(
                "Closed shape {idx} with extends has remainder preds: [{}]",
                reminder.iter().map(|p| p.to_string()).join(", ")
            );*/
            return fail(ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(reminder),
                declared: Preds::new(candidate_preds),
            });
        }
        /*if !reminder.is_empty() {
            debug!(
                "Shape {idx} has extra preds: [{}] but is not closed",
                reminder.iter().map(|p| p.to_string()).join(", ")
            );
        }*/
        /*debug!(
            "Neighs of {node} [{}]",
            values.iter().map(|(p, v)| format!("{p} {v}")).join(", ")
        );*/
        let triple_exprs = merge_ancestor_exprs(schema.get_triple_exprs(idx).unwrap(), schema);
        /*debug!(
            "Candidate triple exprs of {node}:\n{}",
            triple_exprs
                .iter()
                .map(|(maybe_label, te)| format!(
                    "   {} -> [{}]",
                    maybe_label.map(|l| l.to_string()).unwrap_or("_?".to_string()),
                    te.iter().map(|p| p.show_rbe_simplified()).join("\n")
                ))
                .join("\n")
        );*/
        let values_ctx_raw: Vec<_> = values
            .iter()
            .map(|(p, v)| (p.clone(), v.clone(), SemanticActionContext::triple(node, p, v)))
            .collect();

        // Collect every triple-expression reachable from the parent shapes through the full
        // extends / ShapeAnd / ShapeRef hierarchy.  This is needed to distinguish two cases:
        //   (a) A triple that matches no leaf-bucket condition but IS covered by some transitive
        //       ancestor's condition.  These are validated by check_node_extends_main_shape
        //       (exhaustive semantics) and must be excluded from the partition to avoid spurious
        //       partition failures.
        //   (b) A triple that matches no leaf-bucket condition and is NOT covered by any ancestor
        //       either.  These are truly invalid and must remain so the partition correctly fails.
        let all_ancestor_exprs: Vec<Expr> = {
            let mut exprs = Vec::new();
            let mut visited = HashSet::new();
            for pi in triple_exprs.keys().filter_map(|k| *k) {
                exprs.extend(collect_all_exprs_for_shape(&pi, schema, &mut visited));
            }
            // Include None (the shape's own) bucket exprs as well
            if let Some(none_exprs) = triple_exprs.get(&None) {
                exprs.extend(none_exprs.iter().cloned());
            }
            exprs
        };

        let values_ctx: Vec<_> = values_ctx_raw
            .into_iter()
            .filter(|(pred, value, ctx)| {
                // Keep if the triple satisfies at least one leaf-bucket condition
                // (conservatively: Ref conditions count as matching for M^∈ placement).
                let matches_leaf = triple_exprs.values().any(|rbes| {
                    rbes.iter().any(|rbe| {
                        rbe.components().any(|(_, key, cond)| {
                            &key == pred && (cond_has_ref(&cond) || cond.matches(value, ctx).is_ok())
                        })
                    })
                });
                if matches_leaf {
                    return true;
                }
                // EXTRA predicate with no satisfying leaf → goes into M^∉, exclude from partition.
                if extra_preds.contains(pred) {
                    return false;
                }
                // Triple is not needed by any leaf bucket.
                // Keep it only if it is also NOT covered by any ancestor (case b: truly invalid).
                // If it IS covered by an ancestor (case a), exclude it — the exhaustive check
                // in check_node_extends_main_shape already handles it.
                let covered_by_ancestor = all_ancestor_exprs.iter().any(|rbe| {
                    rbe.components()
                        .any(|(_, key, cond)| &key == pred && cond.matches(value, ctx).is_ok())
                });
                !covered_by_ancestor
            })
            .collect();

        let mut reasons_parents = Vec::new();
        tracing::trace!("Checking extends of shape {idx} for node {node}");
        for e in shape.extends() {
            tracing::trace!("Checking extends of shape {idx} for node {node} with extends {e}");
            let result_parents = self.check_node_extends_main_shape(node, e, shape, schema, rdf, typing)?;
            match result_parents {
                Either::Left(errors) => {
                    /*debug!(
                        "Main shape {idx} or some of its ancestors failed for node {node}\nErrors: {}\nTyping: {}",
                        errors.iter().map(|err| format!("{err}")).join(", "),
                        typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                    );*/
                    return Ok(Either::Left(vec![ValidatorError::ShapeExtendsError {
                        node: Box::new(node.clone()),
                        shape: Box::new(shape.clone()),
                        idx: *idx,
                        extends: *e,
                        errors: ValidatorErrors::new(errors),
                    }]));
                },
                Either::Right(reasons) => {
                    /*debug!(
                        "Main shape {idx} and its ancestors succeeded for node {node}\nReasons: {}\nTyping: {}",
                        reasons.iter().map(|r| format!("{r}")).join(", "),
                        typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                    );*/
                    reasons_parents.push(Reason::ShapeExtends {
                        node: node.clone(),
                        shape: Box::new(shape.clone()),
                        reasons: Reasons::new(reasons),
                    });
                },
            }
        }

        let parts_iter = crate::partitions_iter(&values_ctx, &triple_exprs);
        let mut parts_peekable = parts_iter.peekable();
        if let Some(_parts) = parts_peekable.peek() {
            // There are some partitions to check, we will check them one by one until we find one that works or we exhaust all of them. We use peekable to avoid computing the first partition twice (once for the debug message and once for the loop). We could also compute the first partition before the loop and then use a regular iterator, but this way we avoid computing any partition if there are no partitions at all.
            /*debug!(
                "Some partition found for node {node} and shape {idx}, showing the first one:\n{}",
                parts
                    .iter()
                    .enumerate()
                    .map(|(npart, partition)| format!(" Part {npart}: {}\n", show_partition(partition)))
                    .join("\n")
            );*/

            let mut errors_in_partitions = Vec::new();
            for (npart, partition) in parts_peekable.enumerate() {
                let partition_display = create_partitions_display(&partition);
                //debug!("Partition {npart}: {}", partition_display);
                let mut ok_partition = true;
                let mut errors_in_loop = Vec::new();
                let mut reasons_in_loop = Vec::new();
                for (maybe_label, rbes, neighs_subset) in partition.iter() {
                    let result = check_exprs_neigh(rbes, neighs_subset, node, shape, idx, typing)?;
                    match result {
                        Either::Right(reasons) => {
                            /*debug!(
                                " Part {npart}| Success component {}: neighs {}",
                                maybe_label.map(|l| l.to_string()).unwrap_or("[]".to_string()),
                                neighs_subset.iter().map(|(p, v, _ctx)| format!("{p} {v}")).join(", ")
                            );*/
                            reasons_in_loop.push(Reason::PartitionComponent {
                                maybe_label: *maybe_label,
                                node: node.clone(),
                                shape: Box::new(shape.clone()),
                                idx: *idx,
                                partition_idx: npart,
                                partition: partition_display.clone(),
                                neighs: neighs_subset.iter().map(|(p, v, _ctx)| format!("{p} {v}")).join(", "),
                                reasons: Reasons::new(reasons),
                            });
                        },
                        Either::Left(errs) => {
                            errors_in_loop.push(ValidatorError::PartitionComponentFailed {
                                maybe_label: *maybe_label,
                                node: Box::new(node.clone()),
                                shape: Box::new(shape.clone()),
                                idx: *idx,
                                partition_idx: npart,
                                partition: partition_display.clone(),
                                neighs: neighs_subset.iter().map(|(p, v, _ctx)| format!("{p} {v}")).join(", "),
                                errors: ValidatorErrors::new(errs),
                            });
                            ok_partition = false;
                            /*debug!(
                                " Part {npart}| Failed component {}: neighs {}",
                                maybe_label.map(|l| l.to_string()).unwrap_or("[]".to_string()),
                                neighs_subset.iter().map(|(p, v, _ctx)| format!("{p} {v}")).join(", ")
                            );*/
                            // We could collect errors here to provide more information about why the partition failed, but for now we just
                            // indicate that the partition failed without going into details about the failure of each triple expr
                            break;
                        },
                    }
                }
                if ok_partition {
                    // debug!(" Part {npart}| Partition succeeded",);
                    return pass(Reason::ShapeExtends {
                        node: node.clone(),
                        shape: Box::new(shape.clone()),
                        reasons: Reasons::new(reasons_in_loop),
                    });
                } else {
                    // debug!(" Part {npart}| Partition failed",);
                    errors_in_partitions.push(ValidatorError::PartitionFailed {
                        node: Box::new(node.clone()),
                        shape: Box::new(shape.clone()),
                        idx: *idx,
                        partition: partition_display.clone(),
                        errors: ValidatorErrors::new(errors_in_loop),
                    });
                }
            }
            // If we exhaust all partitions, the shape fails
            // debug!(" Failed shape {idx}. All partitions failed",);
            fail(ValidatorError::ShapeFailed {
                node: Box::new(node.clone()),
                shape: Box::new(shape.clone()),
                idx: *idx,
                errors: errors_in_partitions,
            })
        } else {
            debug!("No partitions to check for node {node} and shape {idx}");
            fail(ValidatorError::ShapeFailedNoPartitions {
                node: Box::new(node.clone()),
                shape: Box::new(shape.clone()),
                idx: *idx,
            })
        }
    }

    fn check_node_extends_main_shape<R>(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        shape: &Shape,
        schema: &SchemaIR,
        rdf: &R,
        typing: &mut RefTyping,
    ) -> Result<ValidationResult>
    where
        R: QueryRDF + NeighsRDF,
    {
        tracing::debug!("Checking node {node} against main shape {idx}, shape: {shape}");
        if let Some((ncs, maybe_main_shape, rest_exprs)) = schema.get_main_shape_constraints(idx) {
            tracing::debug!(
                "Main shape {idx} has node constraints: [{}], main shape: {}, rest exprs: [{}]",
                ncs.iter().map(|nc| nc.to_string()).join(", "),
                maybe_main_shape
                    .clone()
                    .map(|ms| ms.to_string())
                    .unwrap_or("None".to_string()),
                rest_exprs.iter().map(|e| e.to_string()).join(", ")
            );
            let mut errors = Vec::new();
            let mut reasons = Vec::new();

            if let Some(main_shape) = maybe_main_shape {
                // For shapes with their own triple expression alongside extends (e.g.,
                // EXTENDS @<Parent> { <p> [4] }), validate the triple expression using
                // exhaustive semantics: pre-filter triples to those matching any component's
                // value condition, then check cardinality. This handles open shapes correctly
                // since extra values that don't match any constraint are simply ignored.
                // Skip this check for shapes with shape-reference value conditions (MatchCond::Ref),
                // which require the partition algorithm to correctly distribute triples.
                let main_preds = main_shape.preds();
                let no_shape_refs = main_shape
                    .triple_expr()
                    .components()
                    .all(|(_, _, cond)| !cond_has_ref(&cond));
                if !main_preds.is_empty() && no_shape_refs {
                    let (all_values, _) = self.neighs(node, main_preds, rdf)?;
                    let all_values_ctx: Vec<_> = all_values
                        .iter()
                        .map(|(p, v)| (p.clone(), v.clone(), SemanticActionContext::triple(node, p, v)))
                        .collect();
                    // Keep only triples where the value satisfies some component's condition
                    let filtered: Vec<_> = all_values_ctx
                        .iter()
                        .filter(|(pred, value, ctx)| {
                            main_shape
                                .triple_expr()
                                .components()
                                .any(|(_, key, cond)| &key == pred && cond.matches(value, ctx).is_ok())
                        })
                        .cloned()
                        .collect();
                    match check_expr_neigh(main_shape.triple_expr(), &filtered, node, shape, idx, typing)? {
                        Either::Left(errs) => {
                            errors.push(ValidatorError::ParentShapeMainShapeFailed {
                                node: Box::new(node.clone()),
                                shape: Box::new(main_shape.clone()),
                                idx: *idx,
                                errors: ValidatorErrors::new(errs),
                            });
                        },
                        Either::Right(rs) => {
                            reasons.push(Reason::ParentShapeMainShapePassed {
                                node: node.clone(),
                                shape: Box::new(main_shape.clone()),
                                idx: *idx,
                                reasons: Reasons::new(rs),
                            });
                        },
                    }
                }
                // Recursively check node constraints and triple expressions through the
                // ancestor extends chain.
                for e in main_shape.extends() {
                    match self.check_node_extends_main_shape(node, e, &main_shape, schema, rdf, typing)? {
                        Either::Left(errs) => {
                            errors.push(ValidatorError::ParentShapeMainShapeFailed {
                                node: Box::new(node.clone()),
                                shape: Box::new(main_shape.clone()),
                                idx: *idx,
                                errors: ValidatorErrors::new(errs),
                            });
                        },
                        Either::Right(rs) => {
                            reasons.push(Reason::ParentShapeMainShapePassed {
                                node: node.clone(),
                                shape: Box::new(main_shape.clone()),
                                idx: *idx,
                                reasons: Reasons::new(rs),
                            });
                        },
                    }
                }
            }
            // We also validate the node constraints of the main shape
            for nc in ncs {
                match nc.cond().matches(node, &SemanticActionContext::subject(node)) {
                    Ok(_pending) => {
                        reasons.push(Reason::ParentShapeNodeConstraint {
                            node: node.clone(),
                            idx: *idx,
                            nc: nc.clone(),
                        });
                    },
                    Err(error) => {
                        errors.push(ValidatorError::ParentShapeNodeConstraintFailed {
                            node: Box::new(node.clone()),
                            idx: *idx,
                            nc: Box::new(nc.clone()),
                            error: error.to_string(),
                        });
                    },
                }
            }
            // Validate NodeConstraints from ShapeRef AND-components (e.g. @<C> in a ShapeAnd).
            // Triple constraints in the referenced shape are already included in the partition's
            // triple_exprs (via get_triple_exprs → Ref.get_triple_exprs), so only the
            // NodeConstraint parts need to be checked here.
            for rest_se in &rest_exprs {
                if let ShapeExpr::Ref { idx: ref_idx } = rest_se
                    && let Some((ref_ncs, _, _)) = schema.get_main_shape_constraints(ref_idx)
                {
                    for nc in ref_ncs {
                        match nc.cond().matches(node, &SemanticActionContext::subject(node)) {
                            Ok(_) => {},
                            Err(error) => {
                                errors.push(ValidatorError::ParentShapeNodeConstraintFailed {
                                    node: Box::new(node.clone()),
                                    idx: *ref_idx,
                                    nc: Box::new(nc.clone()),
                                    error: error.to_string(),
                                });
                            },
                        }
                    }
                }
            }
            if errors.is_empty() {
                pass(Reason::ParentShapePassed {
                    node: node.clone(),
                    idx: *idx,
                    reasons: Reasons::new(reasons),
                })
            } else {
                fail(ValidatorError::ParentShapeFailed {
                    node: Box::new(node.clone()),
                    idx: *idx,
                    errors: ValidatorErrors::new(errors),
                })
            }
        } else {
            tracing::error!("No info for shape {idx}. This is non-extendable");
            fail(ValidatorError::ShapeExtendsNoMainShape {
                idx: *idx,
                node: Box::new(node.clone()),
            })
        }
    }

    fn cnv_iri<S>(&self, iri: S::IRI) -> Pred
    where
        S: NeighsRDF,
    {
        let iri_string = iri.as_str();
        let iri_s = iri!(iri_string);
        Pred::from(iri_s)
    }

    fn cnv_inverse_iri<S>(&self, iri: S::IRI) -> Pred
    where
        S: NeighsRDF,
    {
        let iri_string = iri.as_str();
        let iri_s = iri!(iri_string);
        Pred::new(iri_s, false)
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
    /// - values is a list of pairs (pred, node) where pred.is_direct() determines direction:
    ///   if true, node is the outgoing object; if false, node is the incoming subject
    /// - remainder is the list of outgoing predicates on the node not present in preds as direct predicates
    pub(crate) fn neighs<S>(&self, node: &Node, preds: Vec<Pred>, rdf: &S) -> Result<Neighs>
    where
        S: NeighsRDF,
    {
        let node_term = self.get_rdf_node(node, rdf);

        let (outgoing_preds, incoming_preds): (Vec<_>, Vec<_>) = preds.iter().partition(|pred| pred.is_direct());
        let outgoing_iris: Vec<S::IRI> = outgoing_preds.iter().map(|p| p.iri().clone().into()).collect();
        let incoming_iris: Vec<S::IRI> = incoming_preds.iter().map(|p| p.iri().clone().into()).collect();

        let mut result = Vec::new();
        let mut reminder_preds = Vec::new();

        if let Ok(subject) = S::term_as_subject(&node_term) {
            let (outgoing_arcs, reminder) = rdf
                .outgoing_arcs_from_list(&subject, &outgoing_iris)
                .map_err(|e| self.cnv_err::<S>(e))?;
            for (pred, values) in outgoing_arcs.into_iter() {
                for obj in values.into_iter() {
                    let pred_s = self.cnv_iri::<S>(pred.clone());
                    let object = self.cnv_object::<S>(&obj)?;
                    result.push((pred_s, object))
                }
            }
            for r in reminder {
                let iri_r = self.cnv_iri::<S>(r.clone());
                reminder_preds.push(iri_r);
            }
        }

        if !incoming_iris.is_empty() {
            let incoming_arcs = rdf
                .incoming_arcs_from_list(&node_term, &incoming_iris)
                .map_err(|e| self.cnv_err::<S>(e))?;
            for (pred, subjects) in incoming_arcs.into_iter() {
                for subj in subjects.into_iter() {
                    let pred_s = self.cnv_inverse_iri::<S>(pred.clone());
                    let subject_term: S::Term = subj.into();
                    let subject_node = self.cnv_object::<S>(&subject_term)?;
                    result.push((pred_s, subject_node))
                }
            }
        }

        Ok((result, reminder_preds))
    }

    fn cnv_err<S>(&self, err: S::Err) -> ValidatorError
    where
        S: NeighsRDF,
    {
        tracing::trace!("cnv_err: {err}");
        ValidatorError::SRDFError { error: err.to_string() }
    }

    fn get_rdf_node<S>(&self, node: &Node, _rdf: &S) -> S::Term
    where
        S: NeighsRDF,
    {
        match node.as_object() {
            Object::Iri(iri_s) => {
                let iri: S::IRI = iri_s.clone().into();
                iri.into().into()
            },
            Object::BlankNode(id) => {
                let bnode: S::BNode = BlankNode::new(id);
                bnode.into()
            },
            Object::Literal(lit) => {
                let lit: S::Literal = lit.clone().into();
                let term: S::Term = lit.into();
                term
            },
            Object::Triple { .. } => todo!(),
        }
    }

    pub fn insert_pending(&mut self, atom: &Atom) {
        self.pending.insert((*atom).clone());
    }
}

/// Recursively collects every `Expr` (triple-expression RbeTable) reachable from `idx` by
/// traversing ShapeAnd components, ShapeRef references, and extends chains.  This gives the
/// complete set of conditions declared anywhere in the shape hierarchy rooted at `idx`, which
/// is used to decide whether a particular triple is "covered" by the extends chain (and thus
/// handled by check_node_extends_main_shape's exhaustive semantics) or truly undeclared.
fn collect_all_exprs_for_shape(
    idx: &ShapeLabelIdx,
    schema: &SchemaIR,
    visited: &mut HashSet<ShapeLabelIdx>,
) -> Vec<Expr> {
    if !visited.insert(*idx) {
        return vec![];
    }
    if let Some(info) = schema.find_shape_idx(idx) {
        match info.expr() {
            ShapeExpr::Shape(shape) => {
                let mut exprs = vec![shape.triple_expr().clone()];
                for e in shape.extends() {
                    exprs.extend(collect_all_exprs_for_shape(e, schema, visited));
                }
                exprs
            },
            ShapeExpr::ShapeAnd { exprs } => exprs
                .iter()
                .flat_map(|e| collect_all_exprs_for_shape(e, schema, visited))
                .collect(),
            ShapeExpr::Ref { idx: ref_idx } => collect_all_exprs_for_shape(ref_idx, schema, visited),
            ShapeExpr::NodeConstraint(_) | ShapeExpr::Empty | ShapeExpr::External {} => vec![],
            _ => vec![],
        }
    } else {
        vec![]
    }
}

/// When a shape S extends parents P1, P2, … and some Pi is a transitive ancestor of Pj,
/// the partition would create separate buckets for Pi and Pj.  Because each triple can only
/// go to one bucket, a triple that must satisfy BOTH Pi's and Pj's constraints would fail
/// (diamond inheritance).
///
/// Fix: merge the triple expressions of every "covered" parent (one that is a transitive
/// ancestor of another parent in the set) into the bucket of its leaf descendant.  Only
/// leaf parents (those not subsumed by any other parent in the set) become partition buckets.
fn merge_ancestor_exprs(
    triple_exprs: HashMap<Option<ShapeLabelIdx>, Vec<Expr>>,
    schema: &SchemaIR,
) -> HashMap<Option<ShapeLabelIdx>, Vec<Expr>> {
    let all_parents: Vec<ShapeLabelIdx> = triple_exprs.keys().filter_map(|k| *k).collect();
    if all_parents.len() <= 1 {
        return triple_exprs;
    }

    // For each parent, collect its transitive ancestors.
    let parent_ancestors: Vec<(ShapeLabelIdx, HashSet<ShapeLabelIdx>)> = all_parents
        .iter()
        .map(|&pi| {
            let ancestors: HashSet<ShapeLabelIdx> = schema.parents(&pi).into_iter().collect();
            (pi, ancestors)
        })
        .collect();

    // A parent pj is "covered" if some other parent pi has pj as a transitive ancestor
    // (i.e. pi extends pj directly or transitively).
    let mut covered: HashSet<ShapeLabelIdx> = HashSet::new();
    for (pi, pi_ancestors) in &parent_ancestors {
        for &pj in &all_parents {
            if pj != *pi && pi_ancestors.contains(&pj) {
                covered.insert(pj);
            }
        }
    }

    if covered.is_empty() {
        return triple_exprs;
    }

    let mut result: HashMap<Option<ShapeLabelIdx>, Vec<Expr>> = HashMap::new();

    // Keep the None (own shape) entry unchanged.
    if let Some(exprs) = triple_exprs.get(&None) {
        result.insert(None, exprs.clone());
    }

    // For each uncovered (leaf) parent, collect its own exprs plus the exprs of every
    // covered ancestor that is reachable from it.
    for (pi, pi_ancestors) in &parent_ancestors {
        if !covered.contains(pi) {
            let mut merged = triple_exprs.get(&Some(*pi)).cloned().unwrap_or_default();
            for &pj in &all_parents {
                if covered.contains(&pj)
                    && pi_ancestors.contains(&pj)
                    && let Some(pj_exprs) = triple_exprs.get(&Some(pj))
                {
                    merged.extend(pj_exprs.iter().cloned());
                }
            }
            result.insert(Some(*pi), merged);
        }
    }

    result
}

fn pass(reason: Reason) -> Result<ValidationResult> {
    Ok(Either::Right(vec![reason]))
}

fn fail(err: ValidatorError) -> Result<ValidationResult> {
    Ok(Either::Left(vec![err]))
}

fn check_exprs_neigh(
    exprs: &[Expr],
    neighs: &[(Pred, Node, SemanticActionContext)],
    node: &Node,
    shape: &Shape,
    idx: &ShapeLabelIdx,
    typing: &RefTyping,
) -> Result<ValidationResult> {
    for rbe in exprs.iter() {
        let result = check_expr_neigh(rbe, neighs, node, shape, idx, typing)?;
        if result.is_left() {
            return fail(ValidatorError::ShapeFailed {
                node: Box::new(node.clone()),
                shape: Box::new(shape.clone()),
                idx: *idx,
                errors: result.left().unwrap().clone(),
            });
        }
    }
    pass(Reason::Shape {
        node: node.clone(),
        shape: Box::new(shape.clone()),
        idx: *idx,
    })
}

fn check_expr_neigh(
    expr: &Expr,
    neighs: &[(Pred, Node, SemanticActionContext)],
    node: &Node,
    shape: &Shape,
    idx: &ShapeLabelIdx,
    typing: &RefTyping,
) -> Result<ValidationResult> {
    /*trace!(
        "Checking expr {} with neighs: [{}]",
        expr,
        neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", ")
    );*/
    let mut result_iter = expr.matches(neighs.to_vec())?;
    let first_result = result_iter.next();
    if first_result.is_none() {
        /*debug!(
            "expr {expr} produced no candidates for neighs: [{}]",
            neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", ")
        );*/
        let mut reasons: Vec<NoMatchReason> = result_iter
            .failed_candidates()
            .iter()
            .map(|(candidate, predicate, value, error)| NoMatchReason::ConditionFailed {
                candidate: candidate.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
                error: error.clone(),
            })
            .collect();
        for (candidate, err) in result_iter.failed_cardinality() {
            match expr.cardinality_violations(err) {
                Ok(violations) => {
                    for (predicate, expected, current) in violations {
                        reasons.push(NoMatchReason::CardinalityFailed {
                            candidate: candidate.clone(),
                            predicate,
                            expected,
                            current,
                        });
                    }
                },
                Err(detail) => reasons.push(NoMatchReason::Other {
                    candidate: candidate.clone(),
                    detail,
                }),
            }
        }
        return fail(ValidatorError::NoMatchesFound {
            node: Box::new(node.clone()),
            shape: Box::new(shape.clone()),
            idx: *idx,
            reasons,
        });
    }
    let mut errors = Vec::new();
    for result in first_result.into_iter().chain(result_iter) {
        /*trace!(
            "Result of {expr} with neighs: {}: {:?}",
            neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", "),
            result
        );*/
        match result {
            Ok(pending_values) => {
                if !pending_values.is_empty() {
                    /*tracing::trace!(
                        "Pending values for expr {expr} with neighs: [{}]:\n{pending_values}",
                        neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", "),
                    );*/
                    let mut failed_pending = Vec::new();
                    // Check if all pending values are in typing
                    for (n, idx, ks) in pending_values.iter_vr() {
                        let pair = (n.clone(), *idx);
                        if !typing.contains(&pair) {
                            /*tracing::trace!(
                                "Pending value ({},{}) is not in typing, keys: [{}]",
                                n.clone(),
                                *idx,
                                ks.iter().map(|k| k.to_string()).join(", ")
                            );*/
                            failed_pending.push((
                                n.clone(),
                                *idx,
                                ks.iter().cloned().collect::<Vec<_>>(),
                                typing.errors_for(&pair),
                            ))
                            // TODO: if (stop_at_first) break
                            // We don't need to compute all the failed pending values once we find the first pair
                        }
                    }
                    if failed_pending.is_empty() {
                        //tracing::trace!("All pending values were in typing {pending_values}");
                        return pass(Reason::Shape {
                            node: node.clone(),
                            shape: Box::new(shape.clone()),
                            idx: *idx,
                            // TODO: Add pending_values to reason
                        });
                    } else {
                        /*tracing::trace!(
                            "Failed pending values: {}",
                            failed_pending
                                .iter()
                                .map(|(n, idx, _ks)| format!("{n}@{idx}"))
                                .join(", ")
                        );*/
                        errors.push(ValidatorError::FailedPending {
                            failed_pending: failed_pending.clone(),
                        })
                    }
                } else {
                    // No Pending values
                    return pass(Reason::Shape {
                        node: node.clone(),
                        shape: Box::new(shape.clone()),
                        idx: *idx,
                    });
                }
            },
            Err(err) => {
                // debug!("Result with error: {err}");
                return fail(ValidatorError::RbeError(err));
            },
        }
    }
    // If we reach this point, all results have been processed and all of them have pending values that are not in typing, so the shape failed
    // We can collect all the failed pending values from all the results and return them as errors
    /*debug!(
        "expr failed {expr} with neighs: [{}]. No matching found. Errors: [{}]",
        neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", "),
        errors.iter().map(|e| format!("{e}")).join(", ")
    );*/
    fail(ValidatorError::ShapeFailed {
        node: Box::new(node.clone()),
        shape: Box::new(shape.clone()),
        idx: *idx,
        errors,
    })
}

type PartitionInfo = (
    Option<ShapeLabelIdx>,
    Vec<Expr>,
    Vec<(Pred, Node, SemanticActionContext)>,
);

/*
fn show_partition(partition: &PartitionInfo) -> String {
    let (maybe_label, _rbes, neighs_subset) = partition;
    let label_str = maybe_label.map(|l| l.to_string()).unwrap_or("[]".to_string());
    let neighs_str = neighs_subset.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", ");
    format!("{} -> [{}]", label_str, neighs_str)
}

fn show_result(
    result: &Either<Vec<ValidatorError>, Vec<Reason>>,
    nodes_prefixmap: &PrefixMap,
    schema: &SchemaIR,
    width: usize,
) -> Result<String> {
    match result {
        Either::Left(errors) => {
            let es: Vec<Result<String>> = errors
                .iter()
                .map(|e| {
                    e.show_qualified(nodes_prefixmap, schema, width)
                        .map_err(ValidatorError::PrefixMapError)
                })
                .collect();
            let vs: Vec<String> = es.into_iter().collect::<Result<Vec<_>>>()?;
            Ok(vs.join(", "))
        },
        Either::Right(reasons) => {
            let rs: Vec<Result<String>> = reasons
                .iter()
                .map(|r| {
                    r.show_qualified(nodes_prefixmap, schema, width)
                        .map_err(ValidatorError::PrefixMapError)
                })
                .collect();
            let vs: Vec<String> = rs.into_iter().collect::<Result<Vec<_>>>()?;
            Ok(vs.join(", "))
        },
    }
}*/

fn create_partitions_display(ps: &[PartitionInfo]) -> PartitionsDisplay {
    let partitions_display: Vec<PartitionDisplay> = ps
        .iter()
        .map(|(maybe_label, rbes, neighs_subset)| PartitionDisplay::new(*maybe_label, rbes, neighs_subset))
        .collect();
    PartitionsDisplay::new(&partitions_display)
}

fn check_start_acts(start_acts: &[SemAct], _node: &Node, _idx: &ShapeLabelIdx, schema: &SchemaIR) -> Result<bool> {
    let registry = schema.semantic_actions_registry();
    let context = SemanticActionContext::new_start_act_context();
    for act in start_acts {
        let parameter = act.code().map(|code| code.as_str());
        let result = registry.run_action(act.name(), parameter, &context);
        if let Err(err) = result {
            tracing::error!("Start action {act} failed with error: {err}");
            return Ok(false);
        }
    }
    Ok(true)
}

/// Returns true if a MatchCond contains a shape reference (MatchCond::Ref),
/// which means the condition delegates to a shape's typing check rather than
/// directly verifying a value set.
fn cond_has_ref(cond: &MatchCond<Pred, Node, ShapeLabelIdx, SemanticActionContext>) -> bool {
    match cond {
        MatchCond::Ref(_) => true,
        MatchCond::And(vs) => vs.iter().any(cond_has_ref),
        MatchCond::Single(_) => false,
    }
}

/// Returns true if `(pred, value)` satisfies at least one leaf condition in `expr`.
///
/// For `MatchCond::Ref` leaves the value is kept in M^∈ conservatively — the RBE /
/// pending-typing path already handles shape-reference validation correctly.
fn matches_any_leaf(expr: &Expr, pred: &Pred, value: &Node, ctx: &SemanticActionContext) -> bool {
    for (_, key, cond) in expr.components() {
        if &key != pred {
            continue;
        }
        if cond_has_ref(&cond) || cond.matches(value, ctx).is_ok() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValidatorConfig;
    use rudof_iri::IriS;
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};

    // :alice :knows :bob .
    // :carol :knows :alice .
    // :alice :age  30 .       (outgoing pred not requested in some tests)
    const TEST_GRAPH: &str = r#"
        prefix : <http://example.org/>
        :alice :knows :bob .
        :carol :knows :alice .
        :alice :age 30 .
    "#;

    fn make_graph() -> OxigraphInMemory {
        OxigraphInMemory::from_str(TEST_GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap()
    }

    fn engine() -> Engine {
        Engine::new(&ValidatorConfig::default())
    }

    fn alice() -> Node {
        Node::iri(IriS::new_unchecked("http://example.org/alice"))
    }

    fn bob() -> Node {
        Node::iri(IriS::new_unchecked("http://example.org/bob"))
    }

    fn carol() -> Node {
        Node::iri(IriS::new_unchecked("http://example.org/carol"))
    }

    fn pred_knows_direct() -> Pred {
        Pred::new(IriS::new_unchecked("http://example.org/knows"), true)
    }

    fn pred_knows_inverse() -> Pred {
        Pred::new(IriS::new_unchecked("http://example.org/knows"), false)
    }

    fn pred_age_direct() -> Pred {
        Pred::new(IriS::new_unchecked("http://example.org/age"), true)
    }

    // Requesting only a direct predicate returns matching outgoing arcs;
    // the remainder contains all other outgoing predicates on the node.
    #[test]
    fn test_neighs_direct_pred() {
        let (values, remainder) = engine()
            .neighs(&alice(), vec![pred_knows_direct()], &make_graph())
            .unwrap();

        assert_eq!(values.len(), 1);
        assert!(values.contains(&(pred_knows_direct(), bob())));
        // :age is an outgoing pred of alice that was not requested
        assert_eq!(remainder.len(), 1);
        assert!(remainder.contains(&pred_age_direct()));
    }

    // Requesting only an inverse predicate returns matching incoming arcs
    // with is_direct = false; all outgoing preds go to the remainder.
    #[test]
    fn test_neighs_inverse_pred() {
        let (values, remainder) = engine()
            .neighs(&alice(), vec![pred_knows_inverse()], &make_graph())
            .unwrap();

        assert_eq!(values.len(), 1);
        assert!(values.contains(&(pred_knows_inverse(), carol())));
        // Both :knows and :age are outgoing preds of alice and none were requested directly
        assert!(remainder.contains(&pred_knows_direct()));
        assert!(remainder.contains(&pred_age_direct()));
    }

    // Requesting both a direct and an inverse predicate returns both kinds of arcs;
    // only outgoing preds not explicitly requested as direct end up in remainder.
    #[test]
    fn test_neighs_mixed_direct_and_inverse() {
        let preds = vec![pred_knows_direct(), pred_knows_inverse()];
        let (values, remainder) = engine().neighs(&alice(), preds, &make_graph()).unwrap();

        // outgoing :knows → bob  +  incoming :knows ← carol
        assert_eq!(values.len(), 2);
        assert!(values.contains(&(pred_knows_direct(), bob())));
        assert!(values.contains(&(pred_knows_inverse(), carol())));
        // :age was not requested as a direct pred, so it goes to remainder
        assert_eq!(remainder.len(), 1);
        assert!(remainder.contains(&pred_age_direct()));
    }

    // When preds is empty the values list is empty and every outgoing pred is in the remainder.
    #[test]
    fn test_neighs_empty_preds() {
        let (values, remainder) = engine().neighs(&alice(), vec![], &make_graph()).unwrap();

        assert!(values.is_empty());
        assert!(remainder.contains(&pred_knows_direct()));
        assert!(remainder.contains(&pred_age_direct()));
    }
}
