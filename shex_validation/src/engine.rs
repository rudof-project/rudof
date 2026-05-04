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
use prefixmap::PrefixMap;
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
use shex_ast::ir::preds::Preds;
use shex_ast::ir::schema_ir::SchemaIR;
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

    pub(crate) fn validate_pending<R>(&mut self, rdf: &R, schema: &SchemaIR) -> Result<()>
    where
        R: NeighsRDF + QueryRDF,
    {
        while let Some(atom) = self.pop_pending() {
            match atom.clone() {
                Atom::Pos((node, idx)) => {
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
        // Implements algorithm presented in page 14 of this paper:
        // https://labra.weso.es/publication/2017_semantics-validation-shapes-schemas/
        trace!(
            "Proving {node}@{label} with hyp: [{}]",
            hyp.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );
        hyp.push((node.clone(), *label));
        let hyp_as_set: HashSet<(Node, ShapeLabelIdx)> =
            hyp.iter().map(|(n, l)| (n.clone(), *l)).collect::<HashSet<_>>();
        let mut matched = HashSet::new();
        let candidates = self.dep(node, label, schema, rdf)?;
        let cleaned_candidates: HashSet<_> = candidates.difference(&hyp_as_set).cloned().collect();
        for (n1, l1) in cleaned_candidates {
            // TODO: Change structure to collect errors and reasons instead of using a HashSet
            match self.prove(&n1, &l1, hyp, schema, rdf)? {
                Either::Right(rs) => {
                    debug!(
                        "Proved {n1}@{l1} while proving {node}@{label}: {}",
                        rs.iter().map(|r| format!("{r}")).join(", ")
                    );
                    matched.insert((n1.clone(), l1));
                },
                Either::Left(errors) => {
                    debug!(
                        "Failed to prove {n1}@{l1} while proving {node}@{label}: {}",
                        errors.iter().map(|e| format!("{e}")).join(", ")
                    );
                    // Should we collect errors here?
                },
            }
        }
        let mut typing: HashSet<_> = matched.union(&hyp_as_set).cloned().collect();
        let result = self.check_node_idx(node, label, schema, rdf, &mut typing)?;
        hyp.pop();
        debug!(
            "{} {node}@{label} with result: {}, hyp: [{}]",
            if result.is_right() { "Proved" } else { "Failed to prove" },
            show_result(
                &result,
                &rdf.prefixmap().unwrap_or_default(),
                schema,
                self.config.width()
            )?,
            hyp.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );
        Ok(result)
    }

    pub(crate) fn check_node_idx<R>(
        &self,
        node: &Node,
        idx: &ShapeLabelIdx,
        schema: &SchemaIR,
        rdf: &R,
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        trace!(
            "Checking {node}@{idx}, typing: [{}]",
            typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
        );
        if let Some(info) = schema.find_shape_idx(idx) {
            if schema.is_abstract(idx) {
                let descendants = schema.descendants(idx);
                if descendants.is_empty() {
                    return Err(ValidatorError::AbstractShapeNoDescendants { idx: *idx });
                }
                let descendants_result = self.check_descendants(node, idx, descendants, schema, rdf, typing)?;
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
                tracing::debug!(
                    "Result of {node}@{idx} is: {}",
                    show_result(
                        &result,
                        &rdf.prefixmap().unwrap_or_default(),
                        schema,
                        self.config.width()
                    )?,
                );
                if result.is_right() {
                    Ok(result)
                } else {
                    let descendants = schema.descendants(idx);
                    if descendants.is_empty() {
                        return Ok(result);
                    }
                    // If the shape has descendants, check them too
                    let descendants_result = self.check_descendants(node, idx, descendants, schema, rdf, typing)?;
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
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        let mut errors_collection = Vec::new();
        for desc in descendants {
            match self.check_node_idx(node, &desc, schema, rdf, typing)? {
                Either::Left(errors) => {
                    trace!(
                        "Descendant {desc} failed for node {node}\nErrors: {}\nTyping: {}",
                        errors.iter().map(|err| format!("{err}")).join(", "),
                        typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
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
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
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
                            trace!(
                                "OR branch {e} failed for node {node}\nErrors: {}\nTyping: {}",
                                errors.iter().map(|err| format!("{err}")).join(", "),
                                typing.iter().map(|(n, l)| format!("{n}@{l}")).join(", ")
                            );
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
                debug!("External shape expression encountered for node {node} with shape {se}");
                pass(Reason::External { node: node.clone() })
            },
            ShapeExpr::Ref { idx } => self.check_node_ref(node, idx, typing),
            ShapeExpr::Empty => pass(Reason::Empty { node: node.clone() }),
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
    }

    pub(crate) fn check_node_shape<R>(
        &self,
        idx: &ShapeLabelIdx,
        node: &Node,
        shape: &Shape,
        _schema: &SchemaIR,
        rdf: &R,
        typing: &mut Typing,
    ) -> Result<ValidationResult>
    where
        R: QueryRDF + NeighsRDF,
    {
        trace!("check_node_shape: node = {node}, shape = {idx} [No extends]");
        let (values, reminder) = self.neighs(node, shape.preds(), rdf)?;
        let values_ctx = values
            .iter()
            .map(|(p, v)| (p.clone(), v.clone(), SemanticActionContext::triple(node, p, v)))
            .collect::<Vec<_>>();
        if shape.is_closed() && !reminder.is_empty() {
            return fail(ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(reminder),
                declared: Preds::new(shape.preds().into_iter().collect()),
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
        typing: &mut HashSet<(Node, ShapeLabelIdx)>,
    ) -> Result<ValidationResult>
    where
        R: NeighsRDF + QueryRDF,
    {
        trace!("check_node_shape_extends: node={node}, shape={idx}");
        let preds_extends = Vec::from_iter(schema.get_preds_extends(idx));
        trace!(
            "Predicates in this shape with extends: [{}]",
            preds_extends.iter().map(|p| p.to_string()).join(", ")
        );
        let (values, reminder) = self.neighs(node, preds_extends, rdf)?;

        if shape.is_closed() && !reminder.is_empty() {
            debug!(
                "Closed shape {idx} with extends has remainder preds: [{}]",
                reminder.iter().map(|p| p.to_string()).join(", ")
            );
            return fail(ValidatorError::ClosedShapeWithRemainderPreds {
                remainder: Preds::new(reminder),
                declared: Preds::new(shape.preds().into_iter().collect()),
            });
        }
        if !reminder.is_empty() {
            debug!(
                "Shape {idx} has extra preds: [{}] but is not closed",
                reminder.iter().map(|p| p.to_string()).join(", ")
            );
        }
        debug!(
            "Neighs of {node} [{}]",
            values.iter().map(|(p, v)| format!("{p} {v}")).join(", ")
        );
        let triple_exprs = schema.get_triple_exprs(idx).unwrap();
        debug!(
            "Candidate triple exprs of {node}:\n{}",
            triple_exprs
                .iter()
                .map(|(maybe_label, te)| format!(
                    "   {} -> [{}]",
                    maybe_label.map(|l| l.to_string()).unwrap_or("_?".to_string()),
                    te.iter().map(|p| p.show_rbe_simplified()).join("\n")
                ))
                .join("\n")
        );
        let values_ctx = values
            .iter()
            .map(|(p, v)| (p.clone(), v.clone(), SemanticActionContext::triple(node, p, v)))
            .collect::<Vec<_>>();

        let parts_iter = crate::partitions_iter(&values_ctx, &triple_exprs);
        let mut parts_peekable = parts_iter.peekable();
        if let Some(parts) = parts_peekable.peek() {
            debug!(
                "Some partition found for node {node} and shape {idx}, showing the first one:\n{}",
                parts
                    .iter()
                    .enumerate()
                    .map(|(npart, partition)| format!(" Part {npart}: {}\n", show_partition(partition)))
                    .join("\n")
            );
            let mut errors_in_partitions = Vec::new();
            for (npart, partition) in parts_peekable.enumerate() {
                let partition_display = create_partitions_display(&partition);
                debug!("Partition {npart}: {}", partition_display);
                let mut ok_partition = true;
                let mut errors_in_loop = Vec::new();
                let mut reasons_in_loop = Vec::new();
                for (maybe_label, rbes, neighs_subset) in partition.iter() {
                    let result = check_exprs_neigh(rbes, neighs_subset, node, shape, idx, typing)?;
                    match result {
                        Either::Right(reasons) => {
                            debug!(
                                " Part {npart}| Success component {}: neighs {}",
                                maybe_label.map(|l| l.to_string()).unwrap_or("[]".to_string()),
                                neighs_subset.iter().map(|(p, v, _ctx)| format!("{p} {v}")).join(", ")
                            );
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
                            debug!(
                                " Part {npart}| Failed component {}: neighs {}",
                                maybe_label.map(|l| l.to_string()).unwrap_or("[]".to_string()),
                                neighs_subset.iter().map(|(p, v, _ctx)| format!("{p} {v}")).join(", ")
                            );
                            // We could collect errors here to provide more information about why the partition failed, but for now we just
                            // indicate that the partition failed without going into details about the failure of each triple expr
                            break;
                        },
                    }
                }
                if ok_partition {
                    debug!(" Part {npart}| Partition succeeded",);
                    return pass(Reason::ShapeExtends {
                        node: node.clone(),
                        shape: Box::new(shape.clone()),
                        reasons: Reasons::new(reasons_in_loop),
                    });
                } else {
                    debug!(" Part {npart}| Partition failed",);
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
            debug!(" Failed shape {idx}. All partitions failed",);
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
    /// - remainder is a list of predicates from the neighbours of the node that are not in the given list of predicates
    pub(crate) fn neighs<S>(&self, node: &Node, preds: Vec<Pred>, rdf: &S) -> Result<Neighs>
    where
        S: QueryRDF + NeighsRDF,
    {
        let node = self.get_rdf_node(node, rdf);
        let list: Vec<_> = preds.iter().map(|pred| pred.iri().clone().into()).collect();
        if let Ok(subject) = S::term_as_subject(&node) {
            let (outgoing_arcs, reminder) = rdf
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
            let mut reminder_preds = Vec::new();
            for r in reminder {
                // TODO: We should probably be able to avoid this clone
                let iri_r = self.cnv_iri::<S>(r.clone());
                reminder_preds.push(iri_r);
            }
            Ok((result, reminder_preds))
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
    typing: &Typing,
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
    typing: &Typing,
) -> Result<ValidationResult> {
    trace!(
        "Checking expr {} with neighs: [{}]",
        expr,
        neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", ")
    );
    let result_iter = expr.matches(neighs.to_vec())?;
    let mut result_iter = result_iter.peekable();
    if result_iter.peek().is_none() {
        debug!(
            "expr {expr} produced no candidates for neighs: [{}]",
            neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", ")
        );
        return fail(ValidatorError::NoMatchesFound {
            node: Box::new(node.clone()),
            shape: Box::new(shape.clone()),
            idx: *idx,
        });
    }
    let mut errors = Vec::new();
    for result in result_iter {
        trace!(
            "Result of {expr} with neighs: {}: {:?}",
            neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", "),
            result
        );
        match result {
            Ok(pending_values) => {
                if !pending_values.is_empty() {
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
                        return pass(Reason::Shape {
                            node: node.clone(),
                            shape: Box::new(shape.clone()),
                            idx: *idx,
                        });
                    } else {
                        trace!("Failed pending values: {:?}", failed_pending);
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
                debug!("Result with error: {err}");
                return fail(ValidatorError::RbeError(err));
            },
        }
    }
    // If we reach this point, all results have been processed and all of them have pending values that are not in typing, so the shape failed
    // We can collect all the failed pending values from all the results and return them as errors
    debug!(
        "expr failed {expr} with neighs: [{}]. No matching found. Errors: [{}]",
        neighs.iter().map(|(p, o, _ctx)| format!("{p} {o}")).join(", "),
        errors.iter().map(|e| format!("{e}")).join(", ")
    );
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
}

fn create_partitions_display(ps: &[PartitionInfo]) -> PartitionsDisplay {
    let partitions_display: Vec<PartitionDisplay> = ps
        .iter()
        .map(|(maybe_label, rbes, neighs_subset)| PartitionDisplay::new(*maybe_label, rbes, neighs_subset))
        .collect();
    PartitionsDisplay::new(&partitions_display)
}
