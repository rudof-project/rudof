use super::dependency_graph::{DependencyGraph, PosNeg};
use super::shape_expr::ShapeExpr;
use super::shape_label::ShapeLabel;
use crate::ir::cache::{CACHE_VERSION, CacheFormat, CacheHeader, CacheReaderMode};
use crate::ir::extend_alternative::{ExtendAlternative, cross_merge};
use crate::ir::external_resolver::ExternalShapeResolverRegistry;
use crate::ir::inheritance_graph::InheritanceGraph;
use crate::ir::map_state::MapState;
use crate::ir::node_constraint::NodeConstraint;
use crate::ir::sem_act::SemAct;
use crate::ir::semantic_actions_registry::SemanticActionsRegistry;
use crate::ir::shape::Shape;
use crate::ir::shape_expr_info::ShapeExprInfo;
use crate::ir::source_idx::SourceIdx;
use crate::{CResult, SchemaIRError, ShapeExprLabel, ShapeLabelIdx, ast::Schema as SchemaJson, ir::ast2ir::AST2IR};
use crate::{Expr, Node, Pred, ResolveMethod};
use prefixmap::{IriRef, PrefixMap};
use rudof_iri::IriS;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::{Read, Write};
use std::sync::Arc;
use std::sync::Mutex;
// use tracing::trace;

type Result<A> = std::result::Result<A, Box<SchemaIRError>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SchemaIR {
    labels_idx_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    idx_labels_map: HashMap<ShapeLabelIdx, ShapeLabel>,
    shapes: HashMap<ShapeLabelIdx, ShapeExprInfo>,
    shape_label_counter: usize,
    sources_map: HashMap<IriS, SourceIdx>,
    sources: HashMap<SourceIdx, IriS>,
    sources_counter: usize,
    prefixmap: PrefixMap,
    local_shapes_counter: usize,
    total_shapes_counter: usize,
    imported_schemas: Vec<IriS>,
    dependency_graph: DependencyGraph,
    inheritance_graph: InheritanceGraph,
    abstract_shapes: HashSet<ShapeLabelIdx>,
    #[serde(skip)]
    semantic_actions_registry: Arc<SemanticActionsRegistry>,
    start_acts: Vec<SemAct>,
}

impl SchemaIR {
    pub fn new(registry: SemanticActionsRegistry) -> SchemaIR {
        SchemaIR {
            labels_idx_map: HashMap::new(),
            idx_labels_map: HashMap::new(),
            shape_label_counter: 0,
            sources_map: HashMap::new(),
            sources: HashMap::new(),
            sources_counter: 0,
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
            total_shapes_counter: 0,
            local_shapes_counter: 0,
            imported_schemas: Vec::new(),
            dependency_graph: DependencyGraph::new(),
            inheritance_graph: InheritanceGraph::new(),
            abstract_shapes: HashSet::new(),
            semantic_actions_registry: Arc::new(registry),
            start_acts: Vec::new(),
        }
    }

    pub fn set_map_state(&mut self, map_state: &mut MapState) {
        self.semantic_actions_registry.set_map_state(map_state);
    }

    pub fn set_start_actions(&mut self, start_acts: Vec<SemAct>) {
        self.start_acts = start_acts;
    }

    pub fn start_acts(&self) -> &Vec<SemAct> {
        &self.start_acts
    }

    pub fn set_default_base_prefixes(&mut self, default_base: &IriS) {
        self.prefixmap.set_default_base(&Some(default_base.clone()));
    }

    /// Return the live `Arc<Mutex<MapState>>` from the registered `MapActionExtension`, if any.
    ///
    /// All validation closures compiled into the RBE table share this same Arc (because
    /// `SemanticActionsRegistry::clone` clones the `Arc`s by reference, not by value), so
    /// locking it after validation yields the fully-populated map state.
    pub fn get_map_state_arc(&self) -> Option<Arc<Mutex<MapState>>> {
        self.semantic_actions_registry.get_map_state_arc()
    }

    pub fn set_prefixmap(&mut self, prefixmap: Option<PrefixMap>) {
        self.prefixmap = prefixmap.clone().unwrap_or_default();
    }

    pub fn add_abstract_shape(&mut self, idx: ShapeLabelIdx) {
        self.abstract_shapes.insert(idx);
    }

    /// Returns a clone of the prefix map used in the schema.
    /// Cloning the prefix map is necessary to avoid unintended
    /// side effects from modifying the prefix map outside of the schema IR,
    /// since the prefix map is shared across the entire schema and is used
    /// to resolve prefixed names in shape labels and shape expressions.
    pub fn prefixmap(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    /// Returns true if the shape label corresponding to the given index is marked as abstract in the schema, false otherwise.
    pub fn is_abstract(&self, idx: &ShapeLabelIdx) -> bool {
        self.abstract_shapes.contains(idx)
    }

    pub fn set_local_shapes_counter(&mut self, counter: usize) {
        self.local_shapes_counter = counter;
    }

    pub fn set_imported_schemas(&mut self, imported_schemas: Vec<IriS>) {
        self.imported_schemas = imported_schemas
    }

    pub fn increment_total_shapes(&mut self, new_counter: usize) {
        self.total_shapes_counter += new_counter
    }

    /// Returns the number of shape labels defined in the schema,
    /// which corresponds to the number of shape expressions that have a label (i.e., are not anonymous).
    /// This does not include shapes that are imported from other schemas,
    /// but only those that are defined locally in this schema.
    pub fn shapes_counter(&self) -> usize {
        self.shape_label_counter
    }

    /// Returns the source IRI corresponding to the given source
    pub fn get_source(&self, source_idx: &SourceIdx) -> Option<&IriS> {
        self.sources.get(source_idx)
    }

    /// Returns the total number of shapes in the schema,
    /// including both local and imported shapes.
    pub fn total_shapes_count(&self) -> usize {
        self.total_shapes_counter
    }

    /// Adds a shape expression to the schema IR, associating it with the given shape label and source IRI.
    /// Returns the index of the added shape expression.
    pub fn add_shape(&mut self, shape_label: ShapeLabel, se: ShapeExpr, source_iri: &IriS) -> ShapeLabelIdx {
        let idx = ShapeLabelIdx::from(self.shape_label_counter);
        self.labels_idx_map.insert(shape_label.clone(), idx);
        self.idx_labels_map.insert(idx, shape_label.clone());
        let source_idx = self.new_source_idx(source_iri);
        self.shapes
            .insert(idx, ShapeExprInfo::new(Some(shape_label.clone()), se, source_idx));
        self.shape_label_counter += 1;
        idx
    }

    /// Returns the shape expression corresponding to the given shape label, if it exists.
    pub fn get_shape_expr(&self, shape_label: &ShapeLabel) -> Option<&ShapeExpr> {
        if let Some(idx) = self.find_shape_label_idx(shape_label) {
            self.shapes.get(idx).map(|info| info.expr())
        } else {
            None
        }
    }

    pub fn local_shapes_count(&self) -> usize {
        self.local_shapes_counter
    }

    /// Returns the list of IRIs of the schemas imported by this schema
    pub fn imported_schemas(&self) -> &Vec<IriS> {
        &self.imported_schemas
    }

    /// Returns the parents of the shape expression corresponding to the given index
    pub fn parents(&self, idx: &ShapeLabelIdx) -> Vec<ShapeLabelIdx> {
        self.inheritance_graph.parents(idx)
    }

    /// Returns the descendants of the shape expression corresponding to the given index
    pub fn descendants(&self, idx: &ShapeLabelIdx) -> Vec<ShapeLabelIdx> {
        self.inheritance_graph.descendants(idx)
    }

    pub fn set_semantic_actions_registry(&mut self, registry: Arc<SemanticActionsRegistry>) {
        self.semantic_actions_registry = registry;
    }

    /// Returns a map of shape label indices to the triple expressions of the shape expressions that extend the shape expression corresponding to the given index,
    /// including the shape expression itself (with key None).
    pub fn get_triple_exprs(&self, idx: &ShapeLabelIdx) -> Option<HashMap<Option<ShapeLabelIdx>, Vec<Expr>>> {
        if let Some(info) = self.find_shape_idx(idx) {
            let mut result = HashMap::new();
            let current_exprs = info.expr().get_triple_exprs(self);
            result.insert(None, current_exprs);
            // trace!("Checking parents of {idx}: {:?}", self.parents(idx));
            for e in &self.parents(idx) {
                let shape_expr = self.find_shape_idx(e).unwrap();
                let exprs = shape_expr.expr().get_triple_exprs(self);
                result.insert(Some(*e), exprs);
            }
            Some(result)
        } else {
            None
        }
    }

    /// Get the list of node constraints, the main shape and the rest of shape expressions.
    /// Shape expression references are dereferenced first, so that a parent declared as
    /// `<B> @<A>` behaves like `<A>` when it is extended.
    pub fn get_main_shape_constraints(
        &self,
        idx: &ShapeLabelIdx,
    ) -> Option<(Vec<NodeConstraint>, Option<Shape>, Vec<ShapeExpr>)> {
        let idx = &self.dereference(idx);
        if let Some(info) = self.find_shape_idx(idx) {
            let mut ncs = Vec::new();
            let mut first_shape = None;
            let mut rest_shapes = Vec::new();
            match info.expr() {
                ShapeExpr::Shape(shape) => Some((ncs, Some(*shape.clone()), rest_shapes)),
                ShapeExpr::ShapeAnd { exprs } => {
                    for e in exprs {
                        if let Some(se) = self.find_shape_idx(e) {
                            match se.expr() {
                                ShapeExpr::Shape(s) => {
                                    if first_shape.is_none() {
                                        first_shape = Some(*s.clone());
                                    } else {
                                        rest_shapes.push(ShapeExpr::Shape(s.clone()));
                                    }
                                },
                                ShapeExpr::NodeConstraint(nc) => {
                                    ncs.push(*nc.clone());
                                },
                                _ => {
                                    rest_shapes.push(se.expr().clone());
                                },
                            }
                        }
                    }
                    Some((ncs, first_shape, rest_shapes))
                },
                _ => None,
            }
        } else {
            None
        }
    }

    /// Resolves the shape expression at `idx` into its extend-alternatives: one per choice
    /// of branch for every `ShapeOr` reachable through references, `ShapeAnd`s and the
    /// extends chains of the shapes encountered. Always returns at least one alternative.
    ///
    /// Within a `ShapeAnd`, the first conjunct that yields bucket shapes plays the "main"
    /// role (cf. [`Self::get_main_shape_constraints`]); every other conjunct becomes a
    /// constraint of each alternative. A `Shape` is a bucket, combined with the cross
    /// product of its extends-parents' alternatives. `NodeConstraint`, `ShapeNot` and
    /// `External` are constraints. See `docs/src/internals/feasibility-model.md` §3.
    pub fn extend_alternatives(&self, idx: &ShapeLabelIdx) -> Vec<ExtendAlternative> {
        let mut path = HashSet::new();
        self.extend_alternatives_rec(idx, &mut path)
    }

    fn extend_alternatives_rec(
        &self,
        idx: &ShapeLabelIdx,
        path: &mut HashSet<ShapeLabelIdx>,
    ) -> Vec<ExtendAlternative> {
        let idx = self.dereference(idx);
        if !path.insert(idx) {
            // Cycle through references/extends: rejected by schema analysis elsewhere;
            // contribute a neutral alternative so resolution terminates.
            return vec![ExtendAlternative::default()];
        }
        let result = match self.find_shape_idx(&idx).map(|info| info.expr()) {
            Some(ShapeExpr::Shape(shape)) => {
                let mut acc = vec![ExtendAlternative::with_bucket(idx)];
                for parent in shape.extends() {
                    let parent_alts = self.extend_alternatives_rec(parent, path);
                    acc = cross_merge(acc, parent_alts);
                }
                acc
            },
            Some(ShapeExpr::ShapeOr { exprs }) => exprs
                .iter()
                .flat_map(|e| self.extend_alternatives_rec(e, path))
                .collect(),
            Some(ShapeExpr::ShapeAnd { exprs }) => {
                let mut main_alts: Option<Vec<ExtendAlternative>> = None;
                let mut constraints = Vec::new();
                for e in exprs {
                    if main_alts.is_none() {
                        let alts = self.extend_alternatives_rec(e, path);
                        if alts.iter().any(|a| !a.bucket_shapes().is_empty()) {
                            main_alts = Some(alts);
                            continue;
                        }
                    }
                    constraints.push(self.dereference(e));
                }
                let base = main_alts.unwrap_or_else(|| vec![ExtendAlternative::default()]);
                let conjunct_constraints = ExtendAlternative::with_constraints(constraints);
                base.into_iter().map(|a| a.merge(&conjunct_constraints)).collect()
            },
            Some(ShapeExpr::Empty) | None => vec![ExtendAlternative::default()],
            // NodeConstraint, ShapeNot, External (Ref is impossible after dereference)
            Some(_) => vec![ExtendAlternative::with_constraint(idx)],
        };
        path.remove(&idx);
        result
    }

    /// Follows shape expression references until a non-reference shape expression is found.
    /// Cycle-guarded: on a reference cycle, returns the last index before revisiting.
    fn dereference(&self, idx: &ShapeLabelIdx) -> ShapeLabelIdx {
        let mut current = *idx;
        let mut seen = HashSet::new();
        while seen.insert(current) {
            match self.find_shape_idx(&current).map(|info| info.expr()) {
                Some(ShapeExpr::Ref { idx: next }) => current = *next,
                _ => break,
            }
        }
        current
    }

    pub fn get_preds_extends(&self, idx: &ShapeLabelIdx) -> HashSet<Pred> {
        let mut preds = HashSet::new();
        if let Some(info) = self.find_shape_idx(idx) {
            preds.extend(info.expr().preds(self));
            for e in &self.parents(idx) {
                if let Some(parent_info) = self.find_shape_idx(e) {
                    preds.extend(parent_info.expr().preds(self));
                }
            }
        }
        preds
    }

    pub fn count_extends(&self) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        for (_, _, shape_expr) in self.shapes() {
            let extends_counter = match shape_expr {
                ShapeExpr::Shape(shape) => Some(shape.extends().len()),
                _ => None,
            };

            if let Some(ec) = extends_counter {
                match result.entry(ec) {
                    Entry::Occupied(mut v) => {
                        let r = v.get_mut();
                        *r += 1;
                    },
                    Entry::Vacant(vac) => {
                        vac.insert(1);
                    },
                }
            }
        }
        result
    }

    pub fn populate_from_schema_json(
        &mut self,
        schema_json: &SchemaJson,
        external_resolvers: &ExternalShapeResolverRegistry,
        resolve_method: &ResolveMethod,
        base: &Option<IriS>,
    ) -> Result<()> {
        // Reuse the registry that was already set up on this SchemaIR (cloning it shares the
        // same Arc<dyn SemanticActionExtension> instances, so closures compiled below hold the
        // same Arc<Mutex<MapState>> that callers can later read back via get_map_state_arc).
        let registry = self.semantic_actions_registry.clone();
        let mut compiler = AST2IR::with_registry(resolve_method, registry);
        // `AST2IR::compile` applies `external_resolvers` to the root and imported schemas.
        compiler.compile(schema_json, &schema_json.source_iri(), base, self, external_resolvers)?;
        if let Some(base) = base {
            self.set_default_base_prefixes(base);
        }
        Ok(())
    }

    pub fn find_ref(&self, se_ref: &ShapeExprLabel) -> CResult<ShapeLabelIdx> {
        let shape_label = match se_ref {
            ShapeExprLabel::IriRef { value } => match value {
                IriRef::Iri(iri) => {
                    let label = ShapeLabel::iri(iri.clone());
                    Ok::<ShapeLabel, SchemaIRError>(label)
                },
                IriRef::Prefixed { prefix, local } => {
                    let iri = self.prefixmap.resolve_prefix_local(prefix, local).map_err(|err| {
                        SchemaIRError::PrefixedNotFound {
                            prefix: prefix.clone(),
                            local: local.clone(),
                            err: Box::new(err),
                        }
                    })?;
                    Ok::<ShapeLabel, SchemaIRError>(ShapeLabel::iri(iri))
                },
            },
            ShapeExprLabel::BNode { value } => {
                let label = ShapeLabel::from_bnode((*value).clone());
                Ok(label)
            },
            ShapeExprLabel::Start => Ok(ShapeLabel::Start),
        }?;
        match self.labels_idx_map.get(&shape_label) {
            Some(idx) => Ok(*idx),
            None => Err(Box::new(SchemaIRError::LabelNotFound { shape_label })),
        }
    }

    pub fn find_label(&self, label: &ShapeLabel) -> Option<(&ShapeLabelIdx, &ShapeExpr)> {
        self.find_shape_label_idx(label)
            .and_then(|idx| self.shapes.get(idx).map(|info| (idx, info.expr())))
    }

    pub fn find_shape_label_idx(&self, label: &ShapeLabel) -> Option<&ShapeLabelIdx> {
        self.labels_idx_map.get(label)
    }

    pub fn find_shape_idx(&self, idx: &ShapeLabelIdx) -> Option<&ShapeExprInfo> {
        self.shapes.get(idx)
    }

    pub fn shape_label_from_idx(&self, idx: &ShapeLabelIdx) -> Option<&ShapeLabel> {
        self.shapes.get(idx).and_then(|info| info.label()).or(None)
    }

    pub fn new_index(&mut self, source_iri: &IriS) -> ShapeLabelIdx {
        let idx = ShapeLabelIdx::from(self.shape_label_counter);
        self.shape_label_counter += 1;
        let source_idx = self.new_source_idx(source_iri);
        self.shapes
            .insert(idx, ShapeExprInfo::new(None, ShapeExpr::Empty, source_idx));
        idx
    }

    fn new_source_idx(&mut self, source_iri: &IriS) -> SourceIdx {
        let source_idx = self.sources_map.entry(source_iri.clone()).or_insert_with(|| {
            let idx = SourceIdx::new(self.sources_counter);
            self.sources.insert(idx, source_iri.clone());
            self.sources_counter += 1;
            idx
        });
        *source_idx
    }

    pub fn existing_labels(&self) -> Vec<&ShapeLabel> {
        self.labels_idx_map.keys().collect()
    }

    pub fn shapes(&self) -> impl Iterator<Item = (&ShapeLabel, &IriS, &ShapeExpr)> {
        self.shapes.values().filter_map(|info| {
            info.label().map(|label| {
                let source = self.get_source(info.source_idx()).unwrap();
                (label, source, info.expr())
            })
        })
    }

    // Returns a map of predicates to shape label indices that reference the given index
    pub fn references(&self, idx: &ShapeLabelIdx) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        let visited = HashSet::new();
        self.references_visited(idx, visited)
    }

    //
    pub fn references_visited(
        &self,
        idx: &ShapeLabelIdx,
        mut visited: HashSet<ShapeLabelIdx>,
    ) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        if let Some(info) = self.find_shape_idx(idx) {
            match info.expr() {
                ShapeExpr::Ref { idx } => {
                    if visited.contains(idx) {
                        // If we have already visited this index, we return an empty map to avoid infinite recursion
                        return HashMap::new();
                    }
                    visited.insert(*idx);
                    self.references_visited(idx, visited)
                },
                _ => info.expr().references(self),
            }
        } else {
            HashMap::new()
        }
    }

    #[allow(dead_code)]
    fn cnv_closed(closed: &Option<bool>) -> bool {
        match closed {
            None => false,
            Some(closed) => *closed,
        }
    }

    #[allow(dead_code)]
    fn cnv_extra(&self, extra: &Option<Vec<IriRef>>) -> CResult<Vec<IriS>> {
        extra
            .as_ref()
            .map(|extra| {
                extra
                    .iter()
                    .map(|iri| self.cnv_iri_ref(iri))
                    .collect::<CResult<Vec<_>>>()
            })
            .unwrap_or(Ok(vec![]))
    }

    fn cnv_iri_ref(&self, iri_ref: &IriRef) -> Result<IriS> {
        let iri_s = (*iri_ref).clone().into();
        Ok(iri_s)
    }

    pub fn get_shape_label_idx(&self, shape_label: &ShapeLabel) -> Result<ShapeLabelIdx> {
        match self.labels_idx_map.get(shape_label) {
            Some(shape_label_idx) => Ok(*shape_label_idx),
            None => Err(Box::new(SchemaIRError::ShapeLabelNotFound {
                shape_label: shape_label.clone(),
            })),
        }
    }

    pub fn replace_shape(&mut self, idx: &ShapeLabelIdx, se: ShapeExpr) {
        self.shapes.entry(*idx).and_modify(|info| info.set_expr(se));
    }

    pub fn show_label(&self, label: &ShapeLabel) -> String {
        match label {
            ShapeLabel::Iri(iri) => self.prefixmap.qualify(iri),
            ShapeLabel::BNode(bnode) => format!("{bnode}"),
            ShapeLabel::Start => "START".to_string(),
        }
    }

    pub fn neg_cycles(&self) -> Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>> {
        let dep_graph = self.dependency_graph();
        dep_graph.neg_cycles()
    }

    /// This is used to detect cycles that involve negations in the schema
    /// A well formed schema should not have any cyclic reference that involve a negation
    pub fn has_neg_cycle(&self) -> bool {
        let dep_graph = self.dependency_graph();
        // trace!("Dependency graph: {dep_graph}");
        dep_graph.has_neg_cycle()
    }

    /// Returns the dependency graph.
    pub fn dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }

    pub(crate) fn build_dependency_graph(&mut self) {
        let mut dep_graph = DependencyGraph::new();
        for (idx, info) in self.shapes.iter() {
            let mut visited = Vec::new();
            info.expr()
                .add_edges(*idx, &mut dep_graph, PosNeg::pos(), self, &mut visited);
        }
        self.dependency_graph = dep_graph
    }

    pub(crate) fn build_inheritance_graph(&mut self) {
        let mut inheritance_graph = InheritanceGraph::new();
        for (idx, info) in self.shapes.iter() {
            match info.expr() {
                ShapeExpr::Shape(shape) => {
                    for e in shape.extends() {
                        inheritance_graph.add_edge(*idx, *e);
                    }
                },
                _ => continue,
            }
        }
        self.inheritance_graph = inheritance_graph
    }

    pub fn dependencies(&self) -> Vec<(ShapeLabel, PosNeg, ShapeLabel)> {
        let mut deps = Vec::new();
        for (source, posneg, target) in self.dependency_graph().all_edges() {
            match (self.shape_label_from_idx(&source), self.shape_label_from_idx(&target)) {
                (Some(source_label), Some(target_label)) => {
                    deps.push((source_label.clone(), posneg, target_label.clone()));
                },
                _ => {
                    // We ignore dependencies between shapes that have no labels
                },
            }
        }
        deps
    }

    pub fn show_idx(&self, idx: &ShapeLabelIdx) -> String {
        if let Some(label) = self.shape_label_from_idx(idx) {
            self.show_label(label)
        } else {
            format!("_:{}", idx)
        }
    }

    /// Returns a string representation of the shape expression corresponding to the given index, including the label if it exists.
    pub fn show_shape_idx(&self, idx: &ShapeLabelIdx, width: usize) -> String {
        let mut result = String::new();
        let idx_resolved = self.resolve_shape_ref(idx);
        if let Some(info) = self.find_shape_idx(&idx_resolved) {
            match info.label() {
                Some(label) => {
                    result.push_str(&self.show_label(label));
                },
                None => {
                    result.push_str(&self.show_shape_expr(info.expr(), width).to_string());
                },
            }
        } else {
            result.push_str(format!("ShapeLabelIdx {idx} not found").as_str());
        }
        result
    }

    pub fn show_shape_expr(&self, se: &ShapeExpr, width: usize) -> String {
        match se {
            ShapeExpr::ShapeOr { exprs } => format!(
                "({})",
                exprs
                    .iter()
                    .map(|e| self.show_shape_idx(e, width))
                    .collect::<Vec<_>>()
                    .join(" OR ")
            ),
            ShapeExpr::ShapeAnd { exprs } => format!(
                "({})",
                exprs
                    .iter()
                    .map(|e| self.show_shape_idx(e, width))
                    .collect::<Vec<_>>()
                    .join(" AND ")
            ),
            ShapeExpr::ShapeNot { expr } => format!("NOT ({})", self.show_shape_idx(expr, width)),
            ShapeExpr::NodeConstraint(nc) => format!("{nc}"),
            ShapeExpr::Shape(shape) => self.show_shape(shape, width),
            ShapeExpr::External {} => "EXTERNAL".to_string(),
            ShapeExpr::Ref { idx } => format!("@{}", self.show_idx(idx)),
            ShapeExpr::Empty => "{}".to_string(),
        }
    }

    pub fn show_shape(&self, shape: &Shape, width: usize) -> String {
        let extends = if shape.extends().is_empty() {
            "".to_string()
        } else {
            format!(
                " {}",
                shape
                    .extends()
                    .iter()
                    .map(|e| format!("EXTENDS @{}", self.show_idx(e)))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        let closed = if shape.is_closed() { " CLOSED " } else { "" };
        let extra = if shape.extra().is_empty() {
            "".to_string()
        } else {
            format!(
                " EXTRA {}",
                shape
                    .extra()
                    .iter()
                    .map(|e| self.prefixmap.qualify(e.iri()))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        let show_pred = |p: &Pred| self.prefixmap.qualify(p.iri());
        let show_cond = |node: &Node| node.show_qualified(&self.prefixmap()).to_string();
        let rbe = shape.triple_expr().show_rbe_table(show_pred, show_cond, width);
        format!("{extends}{closed}{extra} {{{rbe}}}")
    }

    pub fn format_cycle_details(
        &self,
        cycle_edges: &[(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)],
    ) -> (Vec<String>, Vec<String>) {
        use std::collections::HashSet;

        if cycle_edges.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let mut labeled_shapes = HashSet::new();
        for (_, _, shapes) in cycle_edges {
            for shape_idx in shapes {
                if self.shape_label_from_idx(shape_idx).is_some() {
                    labeled_shapes.insert(*shape_idx);
                }
            }
        }

        let mut shapes_list: Vec<_> = labeled_shapes
            .iter()
            .map(|idx| {
                if let Some(label) = self.shape_label_from_idx(idx) {
                    self.show_label(label)
                } else {
                    format!("@{}", idx)
                }
            })
            .collect();
        shapes_list.sort();

        let mut edges = Vec::new();
        for (from_idx, to_idx, shapes_in_path) in cycle_edges {
            let from_resolved = self.resolve_shape_ref(from_idx);
            let to_resolved = self.resolve_shape_ref(to_idx);

            let from_str = if let Some(label) = self.shape_label_from_idx(&from_resolved) {
                self.show_label(label)
            } else {
                format!("@{}", from_resolved)
            };

            let to_str = if let Some(label) = self.shape_label_from_idx(&to_resolved) {
                self.show_label(label)
            } else {
                format!("@{}", to_resolved)
            };

            let mut path_parts = vec![from_str.clone()];

            for shape_idx in shapes_in_path {
                let resolved = self.resolve_shape_ref(shape_idx);
                if resolved == from_resolved || resolved == to_resolved {
                    continue;
                }

                let shape_str = if let Some(label) = self.shape_label_from_idx(&resolved) {
                    self.show_label(label)
                } else {
                    format!("@{}", resolved)
                };
                path_parts.push(shape_str);
            }

            path_parts.push(to_str.clone());

            let first_part = format!("{} <--[NOT]-- {}", path_parts[0], path_parts[1]);
            let positive_path = path_parts[2..path_parts.len()].join(" <-- ");

            let path = format!("{} <-- {}", first_part, positive_path);

            edges.push(path);
        }

        (shapes_list, edges)
    }

    fn resolve_shape_ref(&self, idx: &ShapeLabelIdx) -> ShapeLabelIdx {
        if self.shape_label_from_idx(idx).is_some() {
            return *idx;
        }

        if let Some(info) = self.find_shape_idx(idx) {
            match info.expr() {
                ShapeExpr::Ref { idx: ref_idx } => self.resolve_shape_ref(ref_idx),
                ShapeExpr::ShapeNot { expr } => self.resolve_shape_ref(expr),
                ShapeExpr::ShapeAnd { exprs } => {
                    for e in exprs {
                        let resolved = self.resolve_shape_ref(e);
                        if self.shape_label_from_idx(&resolved).is_some() {
                            return resolved;
                        }
                    }
                    *idx
                },
                ShapeExpr::ShapeOr { exprs } => {
                    for e in exprs {
                        let resolved = self.resolve_shape_ref(e);
                        if self.shape_label_from_idx(&resolved).is_some() {
                            return resolved;
                        }
                    }
                    *idx
                },
                _ => *idx,
            }
        } else {
            *idx
        }
    }

    pub fn semantic_actions_registry(&self) -> &SemanticActionsRegistry {
        &self.semantic_actions_registry
    }

    /// Returns a shared handle to the semantic actions registry.
    pub fn semantic_actions_registry_arc(&self) -> Arc<SemanticActionsRegistry> {
        self.semantic_actions_registry.clone()
    }

    pub fn write<W: Write>(&self, mut w: W, fmt: CacheFormat) -> Result<()> {
        let header = CacheHeader::new(fmt, self.has_neg_cycle());
        header.write_to(&mut w)?;
        fmt.write_to(self, &mut w)?;
        Ok(())
    }

    pub fn read<R: Read>(r: R, registry: SemanticActionsRegistry, reader_mode: CacheReaderMode) -> Result<Self> {
        let mut reader = std::io::BufReader::new(r);

        let header = CacheHeader::read_from(&mut reader)?;
        if header.version != CACHE_VERSION {
            return Err(Box::new(SchemaIRError::CacheReadError {
                msg: format!(
                    "Incompatible cache version: found {}, expected {}",
                    header.version, CACHE_VERSION
                ),
            }));
        }

        if reader_mode.is_strict() && header.has_neg_cycle {
            return Err(Box::new(SchemaIRError::CacheReadError {
                msg: "Cache built for a schema with negation cycles cannot be loaded in Strict mode".to_string(),
            }));
        }

        let fmt = header.body_format().map_err(Box::new)?;
        let mut ir = fmt.read_from(&mut reader)?;
        ir.set_semantic_actions_registry(Arc::new(registry));

        Ok(ir)
    }
}

impl Display for SchemaIR {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.sources_counter > 1 {
            writeln!(dest, "Sources:")?;
            for (idx, iri) in self.sources.iter() {
                writeln!(dest, "{idx} -> {iri}")?;
            }
        } else if self.sources_counter == 1 {
            write!(dest, "Source: ")?;
            if let Some((idx, iri)) = self.sources.iter().next() {
                writeln!(dest, "{idx} -> {iri}")?;
            }
        } else {
            writeln!(dest, "No sources")?;
        }
        if !self.start_acts().is_empty() {
            writeln!(
                dest,
                "Start actions: [{}]",
                self.start_acts()
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        writeln!(dest, "SchemaIR with {} shapes", self.shape_label_counter)?;
        writeln!(dest, "Labels to indexes:")?;
        for (label, idx) in self.labels_idx_map.iter() {
            let label = self.show_label(label);
            writeln!(dest, "{label} -> {idx}")?;
        }
        writeln!(dest, "Indexes to Shape Expressions:")?;
        for (idx, info) in self.shapes.iter() {
            let label_str = match info.label() {
                None => "".to_string(),
                Some(label) => format!("{} = ", self.show_label(label)),
            };
            writeln!(
                dest,
                "{idx}{} -> {label_str}{}",
                if self.sources_counter > 1 {
                    format!(" (source: {})", info.source_idx())
                } else {
                    "".to_string()
                },
                info.expr()
            )?;
        }
        writeln!(dest, "Dependency graph: {}", self.dependency_graph())?;
        writeln!(dest, "Inheritance graph: {}", self.inheritance_graph)?;
        writeln!(dest, "---end of schema IR")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rudof_iri::iri;

    use super::SchemaIR;
    use crate::ir::external_resolver::ExternalShapeResolverRegistry;
    use crate::{
        Pred, ResolveMethod, ShapeLabelIdx,
        ast::Schema as SchemaJson,
        ir::{semantic_actions_registry::SemanticActionsRegistry, shape_label::ShapeLabel},
    };

    #[test]
    fn test_find_component() {
        let str = r#"{
            "@context": "http://www.w3.org/ns/shex.jsonld",
            "type": "Schema",
            "shapes": [
                {
                    "type": "ShapeDecl",
                    "id": "http://a.example/S1",
                    "shapeExpr": {
                        "type": "Shape",
                        "expression": {
                            "type": "TripleConstraint",
                            "predicate": "http://a.example/p1"
                        }
                    }
                }
            ]
        }"#;
        let schema_json: SchemaJson = serde_json::from_str::<SchemaJson>(str).unwrap();
        let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
        ir.populate_from_schema_json(
            &schema_json,
            &ExternalShapeResolverRegistry::default(),
            &ResolveMethod::default(),
            &None,
        )
        .unwrap();
        let s1_label: ShapeLabel = ShapeLabel::iri(iri!("http://a.example/S1"));
        let s1 = ir
            .shape_label_from_idx(&ir.get_shape_label_idx(&s1_label).unwrap())
            .unwrap();
        assert_eq!(s1, &s1_label);
    }

    #[test]
    fn test_ir_references() {
        let str = r#"{ "type": "Schema",
            "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
            "type": "Shape",
            "expression": {
            "type": "EachOf",
            "expressions": [{
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/T"
            },
            {
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/U"
            }
          ]
        }
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/T",
      "shapeExpr": {
        "type": "Shape"
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/U",
      "shapeExpr": {
        "type": "Shape"
      }
    }
  ],
  "@context": "http://www.w3.org/ns/shex.jsonld"
}"#;
        let schema: SchemaJson = serde_json::from_str(str).unwrap();
        let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
        ir.populate_from_schema_json(
            &schema,
            &ExternalShapeResolverRegistry::default(),
            &ResolveMethod::default(),
            &None,
        )
        .unwrap();
        println!("Schema IR: {ir}");
        let s: ShapeLabel = ShapeLabel::iri(iri!("http://example.org/S"));
        let idx = ir.get_shape_label_idx(&s).unwrap();
        let references = ir.references(&idx);
        let expected: HashMap<Pred, Vec<ShapeLabelIdx>> = vec![(
            Pred::new_unchecked("http://example.org/p"),
            vec![
                ShapeLabelIdx::from(1), // T
                ShapeLabelIdx::from(2), // U
            ],
        )]
        .into_iter()
        .collect();
        assert_eq!(references, expected);
    }

    #[test]
    fn test_ir_references_and() {
        let str = r#"{
  "type": "Schema",
  "shapes": [
    {
      "type": "ShapeDecl",
      "id": "http://example.org/S",
      "shapeExpr": {
        "type": "ShapeAnd",
        "shapeExprs": [
          {
            "type": "Shape",
            "expression": {
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/T"
            }
          },
          {
            "type": "Shape",
            "expression": {
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/U"
            }
          }
        ]
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/T",
      "shapeExpr": {
        "type": "Shape"
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/U",
      "shapeExpr": {
        "type": "Shape"
      }
    }
  ],
  "@context": "http://www.w3.org/ns/shex.jsonld"
}"#;
        let schema: SchemaJson = serde_json::from_str(str).unwrap();
        let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
        ir.populate_from_schema_json(
            &schema,
            &ExternalShapeResolverRegistry::default(),
            &ResolveMethod::default(),
            &None,
        )
        .unwrap();
        let s: ShapeLabel = ShapeLabel::iri(iri!("http://example.org/S"));
        let idx = ir.get_shape_label_idx(&s).unwrap();
        println!("Schema IR: {ir}");
        println!("Idx: {idx}");
        let references = ir.references(&idx);
        let expected: HashMap<Pred, Vec<ShapeLabelIdx>> = vec![(
            Pred::new_unchecked("http://example.org/p"),
            vec![
                ShapeLabelIdx::from(1), // T
                ShapeLabelIdx::from(2), // U
            ],
        )]
        .into_iter()
        .collect();
        assert_eq!(references, expected);
    }

    fn round_trip_schema_json(name: &str, schema_json_str: &str) {
        let schema: SchemaJson =
            serde_json::from_str(schema_json_str).unwrap_or_else(|e| panic!("[{name}] parse ShEx JSON: {e}"));

        let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
        ir.populate_from_schema_json(
            &schema,
            &ExternalShapeResolverRegistry::default(),
            &ResolveMethod::default(),
            &None,
        )
        .unwrap_or_else(|e| panic!("[{name}] AST -> IR compile: {e}"));

        let config = bincode::config::standard();
        let bytes: Vec<u8> =
            bincode::serde::encode_to_vec(&ir, config).unwrap_or_else(|e| panic!("[{name}] bincode encode: {e}"));
        let (restored, consumed): (SchemaIR, usize) = bincode::serde::decode_from_slice(&bytes, config)
            .unwrap_or_else(|e| panic!("[{name}] bincode decode: {e}"));
        assert_eq!(consumed, bytes.len(), "[{name}] bincode did not consume every byte");

        fn sorted_lines(s: &str) -> Vec<String> {
            let mut lines: Vec<String> = s.lines().map(str::to_string).collect();
            lines.sort();
            lines
        }
        assert_eq!(
            sorted_lines(&format!("{ir}")),
            sorted_lines(&format!("{restored}")),
            "[{name}] Display differs after bincode round-trip",
        );

        assert_eq!(
            ir.shape_label_counter, restored.shape_label_counter,
            "[{name}] shape_label_counter changed",
        );
        assert_eq!(
            ir.labels_idx_map.len(),
            restored.labels_idx_map.len(),
            "[{name}] label count changed",
        );

        for (label, idx) in ir.labels_idx_map.iter() {
            let restored_idx = restored
                .get_shape_label_idx(label)
                .unwrap_or_else(|_| panic!("[{name}] restored missing label {label}"));
            assert_eq!(*idx, restored_idx, "[{name}] index for {label} differs");
            let a_info = ir.find_shape_idx(idx).unwrap();
            let b_info = restored.find_shape_idx(&restored_idx).unwrap();
            assert_eq!(
                format!("{}", a_info.expr()),
                format!("{}", b_info.expr()),
                "[{name}] shape expression for {label} differs",
            );
        }
    }

    #[test]
    fn schema_ir_bincode_round_trip_and_of_two_shapes() {
        let str = r#"{
          "type": "Schema",
          "@context": "http://www.w3.org/ns/shex.jsonld",
          "shapes": [
            { "type": "ShapeDecl", "id": "http://example.org/S",
              "shapeExpr": { "type": "ShapeAnd", "shapeExprs": [
                { "type": "Shape", "expression": {
                  "type": "TripleConstraint",
                  "predicate": "http://example.org/p",
                  "valueExpr": "http://example.org/T" } },
                { "type": "Shape", "expression": {
                  "type": "TripleConstraint",
                  "predicate": "http://example.org/p",
                  "valueExpr": "http://example.org/U" } }
              ] } },
            { "type": "ShapeDecl", "id": "http://example.org/T",
              "shapeExpr": { "type": "Shape" } },
            { "type": "ShapeDecl", "id": "http://example.org/U",
              "shapeExpr": { "type": "Shape" } }
          ]
        }"#;
        round_trip_schema_json("and_of_two_shapes", str);
    }

    #[test]
    fn schema_ir_bincode_round_trip_value_set_literal() {
        let json = r#"{
          "@context": "http://www.w3.org/ns/shex.jsonld",
          "type": "Schema",
          "shapes": [{
            "type": "ShapeDecl",
            "id": "http://example.org/A",
            "shapeExpr": {
              "type": "Shape",
              "expression": {
                "type": "TripleConstraint",
                "predicate": "http://example.org/p",
                "valueExpr": {
                  "type": "NodeConstraint",
                  "values": [ { "value": "A" } ]
                }
              }
            }
          }]
        }"#;
        round_trip_schema_json("value_set_literal", json);
    }

    #[test]
    fn schema_ir_cache_round_trip_from_shexc() {
        use crate::compact::shex_parser::ShExParser;
        use crate::ir::cache::{CacheFormat, CacheReaderMode};
        use rudof_iri::IriS;
        use std::io::Cursor;

        let shexc = r#"prefix : <http://example.org/>

:A {
  :p [ "A" ] ;
}

:S extends @:A {
  :p [ "S" ]
}
"#;
        let base = IriS::new_unchecked("file:///tmp/extends_basic.shex");
        let schema = ShExParser::parse(shexc, Some(base.clone()), &base).expect("parse ShExC");

        let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
        ir.populate_from_schema_json(
            &schema,
            &ExternalShapeResolverRegistry::default(),
            &ResolveMethod::default(),
            &Some(base),
        )
        .expect("populate IR from ShExC-parsed schema");

        let mut buf = Vec::new();
        ir.write(&mut buf, CacheFormat::Bincode).expect("SchemaIR::write");

        let _restored = SchemaIR::read(
            Cursor::new(buf),
            SemanticActionsRegistry::default(),
            CacheReaderMode::Strict,
        )
        .expect("SchemaIR::read");
    }
}
