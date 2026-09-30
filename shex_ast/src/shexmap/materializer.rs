//! Materialization by iteration scopes: build RDF conforming to an output schema from bindings.
//!
//! The binding tree is read as a **scope tree** ([`crate::shexmap::scopes`]): a scope's own
//! bindings, and one list of iteration scopes per repeated constraint or group of the input.
//! Nothing is flattened and nothing is consumed:
//!
//! * A constraint with a Map variable **reads** it from the scope it is evaluated at, or from
//!   the nearest ancestor scope that binds it.  Reading marks nothing, so a parent's binding
//!   can be read in every item, and twice in one.  A variable bound nowhere on that chain
//!   fails the constraint.
//! * A **repetition** in the output schema iterates one list of the input: the deepest list
//!   its body's variables are bound at (nested repetitions count through their parent list).
//!   Its body runs once per iteration of that list, in list order, at that iteration's
//!   scope; an iteration whose body fails is skipped; the count of successful iterations
//!   must reach the minimum, and the repetition stops at the maximum.  A body whose
//!   variables are all bound at or above the current scope runs once, there.  A body whose
//!   variables live in two lists neither of which contains the other is ill-formed, and
//!   says so.
//! * **Choices** (`OneOf`, `ShapeOr`, an optional constraint, a shape with extensions)
//!   yield alternatives, per scope; the alternatives of the parts of a group or conjunction
//!   combine by product, pruned to the best `max_accepts` as they combine.
//! * Every complete alternative is an [`Accept`]; [`Materializer::materialize`] returns the
//!   one that read the most distinct bindings (ties: most triples, then schema order), or
//!   whichever `prefer` ranks first, and keeps the rest in [`Materializer::accepts`].
//! * A value a Map code produces must **satisfy the constraint's value expression** when
//!   that is a node constraint (datatype, value set, facets, node kind).  A plain literal
//!   whose lexical form is valid for the constraint's datatype is retyped to it, so what
//!   `regex()` and `hashmap()` produce can land in a typed constraint; any other mismatch
//!   fails the constraint like an unbound variable, so an optional or another disjunct can
//!   take over (`check_values: false` turns this off).
//! * A shape-valued constraint that also carries a Map variable **names** its node: the
//!   bound value is the node and the nested shape is materialized on it.  One that carries
//!   `%Map:{ id(v:a, v:b) %}` is **keyed**: its node is a function of the shape and the
//!   values read for the key, so two constraints, or two iterations, with the same key make
//!   the same node and their arcs merge -- grouping by value, and a shared reference.  A
//!   key of one IRI or blank node is that node itself; `id(@node)` reuses the input node
//!   the enclosing iteration matched, so a schema maps a graph onto itself node for node.
//!   Otherwise the node is a blank node determined by the root, the constraint, and the
//!   scope (its `@node` when it is an iteration, else its path), so two runs over the same
//!   bindings agree, and materializing the same root twice adds nothing.
//!
//! `EXTENDS` in the output schema materializes the extended shapes' constraints on the same
//! node, and a reference to a shape that has extensions may materialize any non-abstract one
//! of them.

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};

use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::{Object, literal::ConcreteLiteral};
use rudof_rdf::rdf_core::{Any, BuildRDF, NeighsRDF};

use crate::ast::{NodeConstraint, Schema, ShapeExpr, ShapeExprLabel, TripleExpr};
use crate::shexmap::bindings::{BindingTree, Bindings, bind_all_indexed};
use crate::shexmap::error::{Failure, ShExMapError};
use crate::shexmap::functions::{self, KeyTerm, Prefixes};
use crate::shexmap::schema::{
    SchemaIndex, Tc, code_of, iri_of, is_repetition, key, label_of, label_text, min_max, shape_expr_kind, single_value,
    sub_expressions,
};
use crate::shexmap::scopes::{ListPath, ScopeId, ScopeTree};
use crate::shexmap::term::{MapTriple, digest, is_plain_literal, is_resource, n3, typed_literal};

/// A `(scope path, variable)` address of a binding read.
pub type Read = (Vec<usize>, String);

/// How the object of an output triple arose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceKind {
    /// A Map variable's value.
    Variable(String),
    /// A Map function code's value.
    Code(String),
    /// A value the schema names (`[ex:a]`).
    Constant,
    /// A link to a minted node.
    Structural,
    /// A node a variable names, whose nested shape follows.
    Named(String),
    /// An `id()` node.
    Keyed(String),
}

/// The provenance of one output triple: the output constraint, the input scope it was
/// evaluated at (its path and the node it matched), the bindings and static variables read
/// for it, and how the object arose -- the correspondence between output and input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub predicate: String,
    pub scope: Vec<usize>,
    pub node: Option<Object>,
    pub reads: Vec<Read>,
    pub statics: Vec<String>,
    pub kind: SourceKind,
}

/// A complete materialization.
#[derive(Debug, Clone)]
pub struct Accept {
    pub triples: Vec<MapTriple>,
    /// Distinct bindings read.
    pub consumed: usize,
    /// Always 0: nothing is passed over any more (kept for shex.js's report shape).
    pub skipped: usize,
    /// The `(scope path, variable)` addresses read.
    pub used: BTreeSet<Read>,
    /// One record per triple.
    pub provenance: Vec<Source>,
}

/// What the last run found, beyond its triples.
#[derive(Debug, Clone, Default)]
pub struct Report {
    /// Failures on variables the bindings do not hold at all.
    pub unbound_variables: Vec<Failure>,
    pub unused_statics: Vec<String>,
    pub alternatives: usize,
    pub exploration_truncated: bool,
    pub configs_pruned: usize,
}

/// One way to materialize something: its triples, where each came from, and the bindings
/// it read.
#[derive(Debug, Clone, Default)]
struct Alt {
    quads: Vec<MapTriple>,
    reads: BTreeSet<Read>,
    sources: Vec<Source>,
}

impl Alt {
    fn then(&self, other: &Alt) -> Alt {
        let mut quads = self.quads.clone();
        quads.extend(other.quads.iter().cloned());
        let mut reads = self.reads.clone();
        reads.extend(other.reads.iter().cloned());
        let mut sources = self.sources.clone();
        sources.extend(other.sources.iter().cloned());
        Alt { quads, reads, sources }
    }

    fn rank(&self) -> (usize, usize) {
        (self.reads.len(), self.quads.len())
    }
}

type Guard = (&'static str, usize, Vec<usize>, Object);

/// Options of a [`Materializer`].
pub struct Options {
    /// Variable values available everywhere (shex.js's `staticVars`).
    pub static_vars: HashMap<String, Object>,
    /// Extra prefixes for ShExMap variable names, beside the schema's.
    pub prefixes: Prefixes,
    /// Comparator over accepts: `Less` when the first is better.
    pub prefer: Option<Box<dyn Fn(&Accept, &Accept) -> Ordering>>,
    /// Cap on the iterations of one repetition; `None` for none.
    pub max_repeat: Option<usize>,
    /// Cap on nested shape calls (recursive schemas).
    pub max_call_depth: usize,
    /// How many alternatives to keep, at every level and at the end.
    pub max_accepts: usize,
    /// Drop optional subshapes that read no binding.
    pub require_bindings_in_subshapes: bool,
    /// Check bound values against node constraints, retyping plain literals whose lexical
    /// form fits the datatype.
    pub check_values: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            static_vars: HashMap::new(),
            prefixes: Prefixes::new(),
            prefer: None,
            max_repeat: None,
            max_call_depth: 50,
            max_accepts: 20,
            require_bindings_in_subshapes: false,
            check_values: true,
        }
    }
}

impl std::fmt::Debug for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Options")
            .field("static_vars", &self.static_vars)
            .field("prefixes", &self.prefixes)
            .field("prefer", &self.prefer.is_some())
            .field("max_repeat", &self.max_repeat)
            .field("max_call_depth", &self.max_call_depth)
            .field("max_accepts", &self.max_accepts)
            .field("require_bindings_in_subshapes", &self.require_bindings_in_subshapes)
            .field("check_values", &self.check_values)
            .finish()
    }
}

/// Materialize instances of an output schema from ShExMap bindings.
pub struct Materializer {
    index: SchemaIndex,
    options: Options,
    /// The alternatives of the last run, best first.
    pub accepts: Vec<Accept>,
    /// Which of them was returned.
    pub chosen: Option<usize>,
    pub last_report: Report,
    start_expr: Option<ShapeExpr>,
    root: Object,
}

/// One run of the evaluator over one binding tree: the schema by reference (so that
/// expressions keep their identity), the scope tree, and the search's bookkeeping.
struct Run<'a> {
    index: &'a SchemaIndex,
    options: &'a Options,
    tree: ScopeTree,
    root: Object,
    lists: HashMap<usize, Option<ListPath>>,
    tc_index: HashMap<usize, usize>,
    active: Vec<(Guard, String)>,
    failures: Vec<Failure>,
    referenced: HashSet<String>,
    dropped: usize,
}

fn unbounded_or(max: Option<i32>) -> Option<usize> {
    match max {
        None => Some(1),
        Some(-1) => None,
        Some(m) => Some(usize::try_from(m).unwrap_or(0)),
    }
}

impl Materializer {
    pub fn new(schema: &Schema, options: Options) -> Materializer {
        let mut index = SchemaIndex::new(schema);
        for (k, v) in &options.prefixes {
            index.prefixes.insert(k.clone(), v.clone());
        }
        Materializer {
            index,
            options,
            accepts: Vec::new(),
            chosen: None,
            last_report: Report::default(),
            start_expr: None,
            root: Object::bnode("root".to_string()),
        }
    }

    /// The prefixes ShExMap variable names use (the schema's, plus the options').
    pub fn prefixes(&self) -> &Prefixes {
        &self.index.prefixes
    }

    /// The provenance of the chosen materialization, parallel to its triples.
    pub fn provenance(&self) -> Option<&[Source]> {
        self.chosen.map(|i| self.accepts[i].provenance.as_slice())
    }

    /// The root the last run built.
    pub fn root(&self) -> &Object {
        &self.root
    }

    // -- running ----------------------------------------------------------------------------

    /// The triples of the best materialization of `start` (default: the schema's start)
    /// rooted at `root` (default: a new blank node).  The alternatives are kept in
    /// [`accepts`](Self::accepts), best first; the returned one is [`chosen`](Self::chosen).
    ///
    /// Fails when nothing materializes, or the bindings are not a scope tree, or a repetition
    /// reads variables from unrelated lists.
    pub fn materialize(
        &mut self,
        bindings: &BindingTree,
        root: Option<&Object>,
        start: Option<&ShapeExprLabel>,
    ) -> Result<Vec<MapTriple>, ShExMapError> {
        let tree = ScopeTree::new(bindings)?;
        self.root = root
            .cloned()
            .unwrap_or_else(|| Object::bnode(format!("root{}", digest([bindings.canonical().as_str()]))));
        self.start_expr = Some(self.index.start_expr(start)?);
        let label = start.map_or_else(|| "START".to_string(), label_text);
        let mut run = Run {
            index: &self.index,
            options: &self.options,
            tree,
            root: self.root.clone(),
            lists: HashMap::new(),
            tc_index: HashMap::new(),
            active: Vec::new(),
            failures: Vec::new(),
            referenced: HashSet::new(),
            dropped: 0,
        };
        let se = self.start_expr.as_ref().expect("set above");
        let root_scope = run.tree.root();
        let root_node = self.root.clone();
        let results = run.shape_expr(se, root_scope, &root_node, 0, Some(&label))?;
        let mut ranked: Vec<Alt> = results;
        ranked.sort_by_key(|a| std::cmp::Reverse(a.rank())); // stable: schema order among ties
        let mut seen: HashSet<BTreeSet<MapTriple>> = HashSet::new();
        let mut accepts: Vec<Accept> = Vec::new();
        for r in ranked {
            let (triples, provenance) = distinct(&r.quads, &r.sources);
            let k: BTreeSet<MapTriple> = triples.iter().cloned().collect();
            if !seen.insert(k) {
                continue;
            }
            accepts.push(Accept {
                triples,
                consumed: r.reads.len(),
                skipped: 0,
                used: r.reads,
                provenance,
            });
        }
        let report = run.report(accepts.len());
        let failures = std::mem::take(&mut run.failures);
        drop(run);
        self.accepts = accepts;
        self.last_report = report;
        if self.accepts.is_empty() {
            return Err(ShExMapError::Materialization {
                msg: "nothing materializes the shape from these bindings".to_string(),
                failures,
            });
        }
        let mut best = 0;
        if let Some(prefer) = &self.options.prefer {
            for i in 1..self.accepts.len() {
                if prefer(&self.accepts[i], &self.accepts[best]) == Ordering::Less {
                    best = i;
                }
            }
        }
        self.chosen = Some(best);
        Ok(self.accepts[best].triples.clone())
    }

    /// Materialize into `graph`, replacing what the output schema currently holds at `root`:
    /// the schema, used as an input schema on `graph` at `root`, says which triples it
    /// governs there; those no longer produced are removed, the new ones added, and
    /// everything else is left alone.  Returns `(added, removed)`.
    ///
    /// Whether the graph holds an earlier materialization is decided by the predicates the
    /// output schema's start shape constrains: a triple at `root` on one of them means there
    /// is something to replace, and it must then conform.
    pub fn update<G: NeighsRDF + BuildRDF>(
        &mut self,
        graph: &mut G,
        bindings: &BindingTree,
        root: Option<&Object>,
        start: Option<&ShapeExprLabel>,
    ) -> Result<(Vec<MapTriple>, Vec<MapTriple>), ShExMapError> {
        let new: BTreeSet<MapTriple> = self.materialize(bindings, root, start)?.into_iter().collect();
        let root = self.root.clone();
        let mut current: BTreeSet<MapTriple> = BTreeSet::new();
        if self.holds_something(graph, &root, start)? {
            match bind_all_indexed(&self.index, graph, &root, start, self.options.max_accepts) {
                Ok(all) => {
                    for b in all {
                        current.extend(b.matched);
                    }
                },
                Err(ShExMapError::Validation { .. }) => {
                    return Err(ShExMapError::materialization(format!(
                        "{} holds something that does not conform to the output schema, so what to replace \
                         cannot be told",
                        n3(&root)
                    )));
                },
                Err(e) => return Err(e),
            }
        }
        let removed: Vec<MapTriple> = current.difference(&new).cloned().collect();
        let mut added = Vec::new();
        for t in &new {
            if !contains(graph, t)? {
                added.push(t.clone());
            }
        }
        for t in &removed {
            remove_triple(graph, t)?;
        }
        for t in &new {
            add_triple(graph, t)?;
        }
        Ok((added, removed))
    }

    fn holds_something<G: NeighsRDF>(
        &self,
        graph: &G,
        root: &Object,
        start: Option<&ShapeExprLabel>,
    ) -> Result<bool, ShExMapError> {
        let se = self.index.start_expr(start)?;
        let mut seen = HashSet::new();
        for (pred, inverse) in self.own_predicates(&se, &mut seen) {
            let term = G::object_as_term(root);
            let p: G::IRI = pred.into();
            let found = if inverse {
                graph
                    .triples_matching(&Any, &p, &term)
                    .map_err(ShExMapError::rdf)?
                    .next()
                    .is_some()
            } else {
                match G::term_as_subject(&term) {
                    Ok(s) => graph
                        .triples_matching(&s, &p, &Any)
                        .map_err(ShExMapError::rdf)?
                        .next()
                        .is_some(),
                    Err(_) => false,
                }
            };
            if found {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// `(predicate, inverse)` of the constraints on the shape's own node, extensions included.
    fn own_predicates(&self, se: &ShapeExpr, seen: &mut HashSet<usize>) -> BTreeSet<(IriS, bool)> {
        let mut out = BTreeSet::new();
        match se {
            ShapeExpr::Ref(label) => {
                if let Some(decl) = self.index.declaration(label) {
                    for o in self.index.extension_candidates(decl) {
                        out.extend(self.own_predicates(&o.shape_expr, seen));
                    }
                }
            },
            ShapeExpr::Shape(shape) => {
                if !seen.insert(key(se)) {
                    return out;
                }
                let mut stack: Vec<&TripleExpr> = self
                    .index
                    .shape_parts(shape)
                    .iter()
                    .filter_map(|p| p.expression.as_ref().map(|w| &w.te))
                    .collect();
                while let Some(e) = stack.pop() {
                    let Some(e) = self.index.resolve(e) else { continue };
                    if let Some(tc) = Tc::of(e) {
                        out.insert((tc.predicate_iri(), tc.inverse));
                    } else {
                        stack.extend(sub_expressions(e));
                    }
                }
            },
            ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => {
                if !seen.insert(key(se)) {
                    return out;
                }
                for w in shape_exprs {
                    out.extend(self.own_predicates(&w.se, seen));
                }
            },
            _ => {},
        }
        out
    }
}

impl<'a> Run<'a> {
    fn report(&self, found: usize) -> Report {
        let mut available: HashSet<String> = self.options.static_vars.keys().cloned().collect();
        available.extend(self.tree.variables());
        let mut seen = HashSet::new();
        let mut unbound = Vec::new();
        for f in &self.failures {
            if let Some(v) = &f.variable
                && !available.contains(v)
                && seen.insert((v.clone(), f.predicate.clone()))
            {
                unbound.push(f.clone());
            }
        }
        let mut unused: Vec<String> = self
            .options
            .static_vars
            .keys()
            .filter(|s| !self.referenced.contains(*s))
            .cloned()
            .collect();
        unused.sort();
        Report {
            unbound_variables: unbound,
            unused_statics: unused,
            alternatives: found,
            exploration_truncated: self.dropped > 0,
            configs_pruned: self.dropped,
        }
    }

    // -- shape expressions ------------------------------------------------------------------

    fn shape_expr(
        &mut self,
        se: &'a ShapeExpr,
        scope: ScopeId,
        subject: &Object,
        depth: usize,
        label: Option<&str>,
    ) -> Result<Vec<Alt>, ShExMapError> {
        let index = self.index;
        if let ShapeExpr::Ref(lbl) = se {
            let decl = index.declaration(lbl).ok_or_else(|| {
                ShExMapError::schema(format!("Shape {} is not defined in the output schema", label_text(lbl)))
            })?;
            let mut options: Vec<&'a ShapeExpr> = index
                .extension_candidates(decl)
                .into_iter()
                .map(|o| &o.shape_expr)
                .collect();
            if options.is_empty() {
                return Err(ShExMapError::schema(format!(
                    "Shape {} is abstract and nothing extends it",
                    label_text(lbl)
                )));
            }
            options.reverse(); // the declaration itself first, then its extensions
            let text = label_text(lbl);
            let guard: Guard = ("ref", key(decl), self.tree.scope(scope).path.clone(), subject.clone());
            return self.guarded(guard, &text, |m| {
                let mut out = Vec::new();
                for o in options {
                    out.extend(m.shape_expr(o, scope, subject, depth, Some(&text))?);
                }
                Ok(m.prune(out))
            });
        }
        let label_owned = label.map_or_else(|| format!("(inline {})", shape_expr_kind(se)), str::to_string);
        let path = self.tree.scope(scope).path.clone();
        match se {
            ShapeExpr::Shape(shape) => {
                let parts: Vec<Option<&'a TripleExpr>> = index
                    .shape_parts(shape)
                    .iter()
                    .map(|p| p.expression.as_ref().map(|w| &w.te))
                    .collect();
                self.guarded(("shape", key(se), path, subject.clone()), &label_owned, |m| {
                    let mut alts = Vec::new();
                    for e in parts {
                        alts.push(match e {
                            Some(e) => m.expression(e, scope, subject, depth)?,
                            None => vec![Alt::default()],
                        });
                    }
                    Ok(m.all(alts))
                })
            },
            ShapeExpr::ShapeAnd { shape_exprs } => {
                let parts: Vec<&'a ShapeExpr> = shape_exprs
                    .iter()
                    .filter(|w| !matches!(w.se, ShapeExpr::NodeConstraint(_)))
                    .map(|w| &w.se)
                    .collect();
                self.guarded(("and", key(se), path, subject.clone()), &label_owned, |m| {
                    let mut alts = Vec::new();
                    for p in parts {
                        alts.push(m.shape_expr(p, scope, subject, depth, None)?);
                    }
                    Ok(m.all(alts))
                })
            },
            ShapeExpr::ShapeOr { shape_exprs } => {
                let parts: Vec<&'a ShapeExpr> = shape_exprs
                    .iter()
                    .filter(|w| !matches!(w.se, ShapeExpr::NodeConstraint(_)))
                    .map(|w| &w.se)
                    .collect();
                self.guarded(("or", key(se), path, subject.clone()), &label_owned, |m| {
                    let mut out = Vec::new();
                    for p in parts {
                        out.extend(m.shape_expr(p, scope, subject, depth, None)?);
                    }
                    Ok(m.prune(out))
                })
            },
            ShapeExpr::NodeConstraint(_) => Ok(vec![Alt::default()]),
            ShapeExpr::ShapeNot { .. } | ShapeExpr::External => Err(ShExMapError::schema(format!(
                "{} synthesis is not supported",
                shape_expr_kind(se)
            ))),
            ShapeExpr::Ref(_) => unreachable!("handled above"),
        }
    }

    /// Evaluate, refusing a shape expression that reaches itself at the same scope and
    /// subject through references alone (`<A> @<B> OR ...`, `<B> @<A> OR ...`).
    fn guarded<F>(&mut self, guard: Guard, label: &str, run: F) -> Result<Vec<Alt>, ShExMapError>
    where
        F: FnOnce(&mut Run<'a>) -> Result<Vec<Alt>, ShExMapError>,
    {
        if let Some(i) = self.active.iter().position(|(g, _)| *g == guard) {
            let chain: Vec<&str> = self.active[i..].iter().map(|(_, l)| l.as_str()).collect();
            return Err(ShExMapError::schema(format!(
                "cycle in shape expressions: {} -> {label}",
                chain.join(" -> ")
            )));
        }
        self.active.push((guard, label.to_string()));
        let result = run(self);
        self.active.pop();
        result
    }

    // -- triple expressions -----------------------------------------------------------------

    fn expression(
        &mut self,
        expr: &'a TripleExpr,
        scope: ScopeId,
        subject: &Object,
        depth: usize,
    ) -> Result<Vec<Alt>, ShExMapError> {
        let expr = self.resolved(expr)?;
        let (min, max) = min_max(expr);
        let min = usize::try_from(min.unwrap_or(1)).unwrap_or(0);
        let max = unbounded_or(max);
        if min == 1 && max == Some(1) {
            return self.once(expr, scope, subject, depth, false);
        }
        self.repetition(expr, min, max, scope, subject, depth)
    }

    /// A triple expression, an inclusion (`&label`) resolved to what it names.
    fn resolved(&self, expr: &'a TripleExpr) -> Result<&'a TripleExpr, ShExMapError> {
        self.index
            .resolve(expr)
            .ok_or_else(|| ShExMapError::schema("a triple expression inclusion names no expression"))
    }

    /// The alternatives for one occurrence of `expr`, cardinality aside.
    fn once(
        &mut self,
        expr: &'a TripleExpr,
        scope: ScopeId,
        subject: &Object,
        depth: usize,
        skippable: bool,
    ) -> Result<Vec<Alt>, ShExMapError> {
        if let Some(tc) = Tc::of(expr) {
            return self.tc(&tc, scope, subject, depth, skippable);
        }
        match expr {
            TripleExpr::EachOf { .. } => {
                let mut alts = Vec::new();
                for e in sub_expressions(expr) {
                    alts.push(self.expression(e, scope, subject, depth)?);
                }
                Ok(self.all(alts))
            },
            TripleExpr::OneOf { .. } => {
                let mut out = Vec::new();
                for e in sub_expressions(expr) {
                    out.extend(self.expression(e, scope, subject, depth)?);
                }
                Ok(self.prune(out))
            },
            _ => Err(ShExMapError::schema("unexpected triple expression")),
        }
    }

    fn repetition(
        &mut self,
        expr: &'a TripleExpr,
        min: usize,
        max: Option<usize>,
        scope: ScopeId,
        subject: &Object,
        depth: usize,
    ) -> Result<Vec<Alt>, ShExMapError> {
        let list_path = self.list_path(expr)?;
        let skippable = min == 0;
        let depth_here = self.tree.scope(scope).depth();
        let iterates = match &list_path {
            Some(lp) if lp.len() > depth_here => lp.clone(),
            _ => {
                // the body reads nothing below this scope: it runs once, here
                if min > 1 {
                    return Ok(Vec::new());
                }
                let body = self.once(expr, scope, subject, depth, skippable)?;
                return Ok(if body.is_empty() {
                    if min == 0 { vec![Alt::default()] } else { Vec::new() }
                } else {
                    body
                });
            },
        };
        let cap = match (max, self.options.max_repeat) {
            (Some(m), Some(r)) => Some(m.min(r)),
            (Some(m), None) => Some(m),
            (None, r) => r,
        };
        let mut results = vec![Alt::default()];
        let mut count = 0usize;
        for item in self.tree.descendants_at(scope, &iterates) {
            if cap.is_some_and(|c| count >= c) {
                break;
            }
            let body = self.once(expr, item, subject, depth, skippable)?;
            if body.is_empty() {
                continue; // this item does not fit the body: skip it
            }
            results = self.all(vec![results, body]);
            count += 1;
        }
        Ok(if count >= min { results } else { Vec::new() })
    }

    fn fail(
        &mut self,
        predicate: &str,
        variable: Option<String>,
        code: Option<String>,
        error: Option<String>,
        scope: Option<Vec<usize>>,
    ) {
        self.failures.push(Failure {
            predicate: predicate.to_string(),
            variable,
            code,
            error,
            scope,
        });
    }

    fn triple_for(&mut self, tc: &Tc<'_>, subject: &Object, obj: &Object) -> Option<MapTriple> {
        let pred = tc.predicate_iri();
        if tc.inverse {
            if !is_resource(obj) {
                self.fail(
                    &tc.predicate_str(),
                    None,
                    None,
                    Some("literal subject of inverse".to_string()),
                    None,
                );
                return None;
            }
            Some(MapTriple::new(obj.clone(), pred, subject.clone()))
        } else {
            Some(MapTriple::new(subject.clone(), pred, obj.clone()))
        }
    }

    fn source(&self, tc: &Tc<'_>, scope: ScopeId, reads: Vec<Read>, statics: Vec<String>, kind: SourceKind) -> Source {
        let s = self.tree.scope(scope);
        Source {
            predicate: tc.predicate_str(),
            scope: s.path.clone(),
            node: self.tree.nearest_node(scope).cloned(),
            reads,
            statics,
            kind,
        }
    }

    /// Read a variable at a scope: statics first, then the scope chain.  Records the read.
    fn get(
        &mut self,
        scope: ScopeId,
        v: &str,
        reads: &mut BTreeSet<Read>,
        statics: &mut Vec<String>,
    ) -> Option<Object> {
        self.referenced.insert(v.to_string());
        if let Some(val) = self.options.static_vars.get(v) {
            statics.push(v.to_string());
            return Some(val.clone());
        }
        let (value, at) = self.tree.lookup(scope, v)?;
        let value = value.clone();
        reads.insert((self.tree.scope(at).path.clone(), v.to_string()));
        Some(value)
    }

    fn tc(
        &mut self,
        tc: &Tc<'a>,
        scope: ScopeId,
        subject: &Object,
        depth: usize,
        skippable: bool,
    ) -> Result<Vec<Alt>, ShExMapError> {
        let pred_str = tc.predicate_str();
        let acts: Vec<String> = tc.map_actions().iter().map(|a| code_of(a)).collect();
        if acts.iter().any(|c| functions::is_key_code(c)) {
            return self.keyed(tc, &acts, scope, subject, depth, skippable);
        }
        let scope_path = self.tree.scope(scope).path.clone();
        if !acts.is_empty() {
            let mut reads: BTreeSet<Read> = BTreeSet::new();
            let mut quads = Vec::new();
            let mut objects = Vec::new();
            let mut sources = Vec::new();
            for code in &acts {
                let before = reads.clone();
                let mut statics_read = Vec::new();
                let prefixes = self.index.prefixes.clone();
                let (value, kind) = if functions::is_function_call(code) {
                    let mut getter = |v: &str| self.get(scope, v, &mut reads, &mut statics_read);
                    let value = match functions::lower(code, &mut getter, &prefixes) {
                        Ok(v) => v,
                        Err(e) => {
                            self.fail(&pred_str, None, Some(code.clone()), Some(e.to_string()), None);
                            return Ok(Vec::new());
                        },
                    };
                    let Some(value) = value else {
                        self.fail(&pred_str, None, Some(code.clone()), Some("unbound".to_string()), None);
                        return Ok(Vec::new());
                    };
                    (value, SourceKind::Code(code.trim().to_string()))
                } else {
                    let var = match functions::expand_variable(code, &prefixes) {
                        Ok(v) => v,
                        Err(e) => {
                            self.fail(&pred_str, None, Some(code.clone()), Some(e.to_string()), None);
                            return Ok(Vec::new());
                        },
                    };
                    let Some(value) = self.get(scope, &var, &mut reads, &mut statics_read) else {
                        self.fail(&pred_str, Some(var), None, None, Some(scope_path.clone()));
                        return Ok(Vec::new());
                    };
                    (value, SourceKind::Variable(var))
                };
                let value = if self.options.check_values {
                    match self.checked(tc, &value)? {
                        Some(v) => v,
                        None => {
                            self.fail(
                                &pred_str,
                                None,
                                Some(code.clone()),
                                Some("the value does not satisfy the value expression".to_string()),
                                None,
                            );
                            return Ok(Vec::new());
                        },
                    }
                } else {
                    value
                };
                let Some(t) = self.triple_for(tc, subject, &value) else {
                    return Ok(Vec::new());
                };
                quads.push(t);
                objects.push(value);
                let new_reads: Vec<Read> = reads.difference(&before).cloned().collect();
                sources.push(self.source(tc, scope, new_reads, statics_read, kind));
            }
            if let (true, Some(ve)) = (
                tc.references_shape() && objects.len() == 1 && is_resource(&objects[0]),
                tc.value_expr,
            ) {
                // the variable names the node: materialize the nested shape on it
                if depth >= self.options.max_call_depth {
                    self.fail(&pred_str, None, None, Some("exceeded max_call_depth".to_string()), None);
                    return Ok(Vec::new());
                }
                if let SourceKind::Variable(v) = &sources[0].kind {
                    sources[0].kind = SourceKind::Named(v.clone());
                }
                let head = Alt { quads, reads, sources };
                let node = objects[0].clone();
                let nested = self.shape_expr(ve, scope, &node, depth + 1, None)?;
                return Ok(nested.iter().map(|n| head.then(n)).collect());
            }
            return Ok(vec![Alt { quads, reads, sources }]);
        }

        if let Some(constant) = single_value(tc.value_expr) {
            let Some(t) = self.triple_for(tc, subject, &constant) else {
                return Ok(Vec::new());
            };
            let src = self.source(tc, scope, Vec::new(), Vec::new(), SourceKind::Constant);
            return Ok(vec![Alt {
                quads: vec![t],
                reads: BTreeSet::new(),
                sources: vec![src],
            }]);
        }

        if let (true, Some(ve)) = (tc.references_shape(), tc.value_expr) {
            if depth >= self.options.max_call_depth {
                self.fail(&pred_str, None, None, Some("exceeded max_call_depth".to_string()), None);
                return Ok(Vec::new());
            }
            let node = self.mint(tc, scope, depth);
            let Some(link) = self.triple_for(tc, subject, &node) else {
                return Ok(Vec::new());
            };
            let src = self.source(tc, scope, Vec::new(), Vec::new(), SourceKind::Structural);
            let mut out = Vec::new();
            for n in self.shape_expr(ve, scope, &node, depth + 1, None)? {
                if skippable && n.reads.is_empty() && (n.quads.is_empty() || self.options.require_bindings_in_subshapes)
                {
                    continue; // an optional island nothing asked for
                }
                let head = Alt {
                    quads: vec![link.clone()],
                    reads: BTreeSet::new(),
                    sources: vec![src.clone()],
                };
                out.push(head.then(&n));
            }
            return Ok(out);
        }

        let what = tc.value_expr.map_or("any value", shape_expr_kind);
        self.fail(
            &pred_str,
            None,
            None,
            Some(format!("cannot synthesize {what} without a Map action")),
            None,
        );
        Ok(Vec::new())
    }

    /// A shape-valued constraint with `id(...)`: the node is a function of the key.
    fn keyed(
        &mut self,
        tc: &Tc<'a>,
        acts: &[String],
        scope: ScopeId,
        subject: &Object,
        depth: usize,
        skippable: bool,
    ) -> Result<Vec<Alt>, ShExMapError> {
        let pred = tc.predicate_str();
        if acts.len() > 1 {
            return Err(ShExMapError::schema(format!(
                "id() must be the only Map code on the constraint at {pred}"
            )));
        }
        let Some(ve) = tc.value_expr.filter(|_| tc.references_shape()) else {
            return Err(ShExMapError::schema(format!(
                "id() at {pred} needs a shape-valued constraint: it names a node, not a value"
            )));
        };
        let code = acts[0].clone();
        let terms = functions::key_terms(&code, &self.index.prefixes)?;
        let mut reads: BTreeSet<Read> = BTreeSet::new();
        let mut statics_read: Vec<String> = Vec::new();
        let scope_path = self.tree.scope(scope).path.clone();
        let mut values: Vec<Object> = Vec::new();
        for term in terms {
            let value = match term {
                KeyTerm::Node => match self.tree.nearest_node(scope) {
                    Some(n) => Some(n.clone()),
                    None => {
                        self.fail(
                            &pred,
                            None,
                            Some(code.clone()),
                            Some("no @node in scope".to_string()),
                            None,
                        );
                        return Ok(Vec::new());
                    },
                },
                KeyTerm::Template(t) => {
                    let mut missing: Option<String> = None;
                    let mut getter = |v: &str| {
                        let hit = self.get(scope, v, &mut reads, &mut statics_read);
                        if hit.is_none() {
                            missing = Some(v.to_string());
                        }
                        hit
                    };
                    let v = t.expand(&mut getter);
                    if v.is_none() {
                        self.fail(&pred, missing, Some(code.clone()), None, Some(scope_path.clone()));
                        return Ok(Vec::new());
                    }
                    v
                },
                KeyTerm::Variable(v) => {
                    let hit = self.get(scope, &v, &mut reads, &mut statics_read);
                    if hit.is_none() {
                        self.fail(&pred, Some(v), Some(code.clone()), None, Some(scope_path.clone()));
                        return Ok(Vec::new());
                    }
                    hit
                },
            };
            values.push(value.expect("checked above"));
        }
        let node = if values.len() == 1 && is_resource(&values[0]) {
            values[0].clone()
        } else {
            let shape = match ve {
                ShapeExpr::Ref(lbl) => label_text(lbl),
                _ => {
                    let n = self.tc_ordinal(tc);
                    format!("inline{n}")
                },
            };
            let lexicals: Vec<String> = values.iter().map(n3).collect();
            let mut parts = vec![shape.as_str()];
            parts.extend(lexicals.iter().map(String::as_str));
            Object::bnode(format!("k{}", digest(parts)))
        };
        if depth >= self.options.max_call_depth {
            self.fail(&pred, None, None, Some("exceeded max_call_depth".to_string()), None);
            return Ok(Vec::new());
        }
        let Some(link) = self.triple_for(tc, subject, &node) else {
            return Ok(Vec::new());
        };
        let src = self.source(
            tc,
            scope,
            reads.iter().cloned().collect(),
            statics_read,
            SourceKind::Keyed(code.trim().to_string()),
        );
        let mut out = Vec::new();
        for n in self.shape_expr(ve, scope, &node, depth + 1, None)? {
            if skippable
                && n.reads.is_empty()
                && reads.is_empty()
                && (n.quads.is_empty() || self.options.require_bindings_in_subshapes)
            {
                continue;
            }
            let head = Alt {
                quads: vec![link.clone()],
                reads: reads.clone(),
                sources: vec![src.clone()],
            };
            out.push(head.then(&n));
        }
        Ok(out)
    }

    fn tc_ordinal(&mut self, tc: &Tc<'_>) -> usize {
        let next = self.tc_index.len();
        *self.tc_index.entry(key(tc.expr)).or_insert(next)
    }

    // -- value expressions ------------------------------------------------------------------

    /// `value` if it satisfies the constraint's value expression, a retyped copy of a plain
    /// literal whose lexical form fits the expression's datatype, or `None`.
    fn checked(&self, tc: &Tc<'_>, value: &Object) -> Result<Option<Object>, ShExMapError> {
        let Some(se) = tc.value_expr.and_then(|se| self.resolve_se(se)) else {
            return Ok(Some(value.clone())); // nothing to check: no expression at all
        };
        if !self.is_node_constraint(se) {
            return Ok(Some(value.clone())); // a shape: the nested materialization is the check
        }
        if self.satisfies(se, value)? {
            return Ok(Some(value.clone()));
        }
        if let (Some(dt), Object::Literal(lit)) = (self.datatype_of(se), value)
            && is_plain_literal(value)
        {
            let raw = ConcreteLiteral::lit_datatype(&lit.lexical_form(), &prefixmap::IriRef::iri(dt.clone()));
            let valid = match raw.into_checked_literal() {
                Ok(ConcreteLiteral::WrongDatatypeLiteral { .. }) | Err(_) => false,
                Ok(_) => true,
            };
            if valid {
                let candidate = Object::literal(typed_literal(&lit.lexical_form(), &dt));
                if self.satisfies(se, &candidate)? {
                    return Ok(Some(candidate));
                }
            }
        }
        Ok(None)
    }

    fn resolve_se<'s>(&'s self, mut se: &'s ShapeExpr) -> Option<&'s ShapeExpr> {
        for _ in 0..100 {
            match se {
                ShapeExpr::Ref(label) => se = &self.index.declaration(label)?.shape_expr,
                other => return Some(other),
            }
        }
        None
    }

    /// A value expression that constrains the node itself, not its arcs.
    fn is_node_constraint(&self, se: &ShapeExpr) -> bool {
        match se {
            ShapeExpr::NodeConstraint(_) => true,
            ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => shape_exprs
                .iter()
                .filter(|w| !matches!(w.se, ShapeExpr::Ref(_)))
                .any(|w| self.is_node_constraint(&w.se)),
            ShapeExpr::ShapeNot { shape_expr } => self.is_node_constraint(&shape_expr.se),
            _ => false,
        }
    }

    /// The datatype a value expression asks for: its own, or the one conjunct that has one.
    fn datatype_of(&self, se: &ShapeExpr) -> Option<IriS> {
        let se = self.resolve_se(se)?;
        match se {
            ShapeExpr::NodeConstraint(nc) => nc.datatype().map(|d| iri_of(&d)),
            ShapeExpr::ShapeAnd { shape_exprs } => {
                let found: Vec<IriS> = shape_exprs.iter().filter_map(|w| self.datatype_of(&w.se)).collect();
                let distinct: HashSet<&IriS> = found.iter().collect();
                if distinct.len() == 1 {
                    found.into_iter().next()
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn satisfies(&self, se: &ShapeExpr, value: &Object) -> Result<bool, ShExMapError> {
        let Some(se) = self.resolve_se(se) else { return Ok(true) };
        match se {
            ShapeExpr::NodeConstraint(nc) => self.satisfies_nc(nc, value),
            ShapeExpr::ShapeAnd { shape_exprs } => {
                for w in shape_exprs {
                    let is_nc = self.resolve_se(&w.se).is_some_and(|p| self.is_node_constraint(p));
                    if is_nc && !self.satisfies(&w.se, value)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    if self.satisfies(&w.se, value)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            },
            ShapeExpr::ShapeNot { shape_expr } => Ok(!self.satisfies(&shape_expr.se, value)?),
            _ => Ok(true), // a shape: the nested materialization is the check
        }
    }

    fn satisfies_nc(&self, nc: &NodeConstraint, value: &Object) -> Result<bool, ShExMapError> {
        self.index.satisfies_nc(nc, value)
    }

    /// The node for a shape-valued constraint without a key: determined by the root, the
    /// constraint, its call depth, and the scope -- the input node the scope matched when it
    /// is an iteration, else its path -- so a materialization is reproducible and two runs
    /// over the same input agree.
    fn mint(&mut self, tc: &Tc<'_>, scope: ScopeId, depth: usize) -> Object {
        let n = self.tc_ordinal(tc);
        let s = self.tree.scope(scope);
        let where_ = match &s.node {
            Some(node) => n3(node),
            None => format!(
                "p{}",
                s.path.iter().map(ToString::to_string).collect::<Vec<_>>().join("_")
            ),
        };
        let root = n3(&self.root);
        let n = n.to_string();
        let depth = depth.to_string();
        Object::bnode(format!(
            "m{}",
            digest([root.as_str(), n.as_str(), depth.as_str(), where_.as_str()])
        ))
    }

    // -- combining alternatives -------------------------------------------------------------

    /// Every combination of one alternative per part, pruned as it grows.
    fn all(&mut self, parts: Vec<Vec<Alt>>) -> Vec<Alt> {
        let mut acc = vec![Alt::default()];
        for alternatives in parts {
            if alternatives.is_empty() {
                return Vec::new();
            }
            let mut next = Vec::new();
            for a in &acc {
                for b in &alternatives {
                    next.push(a.then(b));
                }
            }
            acc = self.prune(next);
        }
        acc
    }

    fn prune(&mut self, results: Vec<Alt>) -> Vec<Alt> {
        if results.len() <= self.options.max_accepts {
            return results;
        }
        let mut order: Vec<usize> = (0..results.len()).collect();
        order.sort_by(|&i, &j| results[j].rank().cmp(&results[i].rank()).then(i.cmp(&j)));
        order.truncate(self.options.max_accepts);
        self.dropped += results.len() - order.len();
        order.sort_unstable();
        let mut kept: Vec<Option<Alt>> = results.into_iter().map(Some).collect();
        order.into_iter().filter_map(|i| kept[i].take()).collect()
    }

    // -- which list a repetition iterates ---------------------------------------------------

    fn list_path(&mut self, expr: &'a TripleExpr) -> Result<Option<ListPath>, ShExMapError> {
        let k = key(expr);
        if let Some(found) = self.lists.get(&k) {
            return Ok(found.clone());
        }
        self.lists.insert(k, None); // a recursive repetition counts as scope-free while computed
        let found = self.analyse(expr)?;
        self.lists.insert(k, found.clone());
        Ok(found)
    }

    /// The list the repetition `expr` iterates: the deepest list its body's variables are
    /// bound at, nested repetitions counting through their parent list.
    fn analyse(&mut self, expr: &'a TripleExpr) -> Result<Option<ListPath>, ShExMapError> {
        let mut direct: BTreeSet<String> = BTreeSet::new();
        let mut nested: Vec<&'a TripleExpr> = Vec::new();
        let mut seen = HashSet::new();
        self.collect(expr, &mut direct, &mut nested, &mut seen, true);
        let mut candidates: Vec<(ListPath, String)> = Vec::new();
        for v in &direct {
            if let Some(lp) = self.tree.bound_at.get(v)
                && !candidates.iter().any(|(c, _)| c == lp)
            {
                candidates.push((lp.clone(), v.clone()));
            }
        }
        for r in &nested {
            if let Some(lp) = self.list_path(r)?
                && !lp.is_empty()
            {
                let parent = lp[..lp.len() - 1].to_vec();
                if !candidates.iter().any(|(c, _)| *c == parent) {
                    candidates.push((parent, format!("the repetition at {}", label_of(r, self.index))));
                }
            }
        }
        if candidates.is_empty() {
            return Ok(None);
        }
        let (deepest, deepest_why) = candidates
            .iter()
            .max_by_key(|(lp, _)| lp.len())
            .cloned()
            .expect("non-empty");
        for (lp, why) in &candidates {
            if deepest.len() < lp.len() || deepest[..lp.len()] != lp[..] {
                return Err(ShExMapError::materialization(format!(
                    "the repetition at {} reads from unrelated lists: {why} is bound at {lp:?} and {deepest_why} at {deepest:?}",
                    label_of(expr, self.index)
                )));
            }
        }
        Ok(Some(deepest))
    }

    /// Variables read directly in `expr`'s body and the repetitions directly inside it.
    fn collect(
        &self,
        expr: &'a TripleExpr,
        direct: &mut BTreeSet<String>,
        nested: &mut Vec<&'a TripleExpr>,
        seen: &mut HashSet<usize>,
        top: bool,
    ) {
        let Some(expr) = self.index.resolve(expr) else { return };
        if !seen.insert(key(expr)) {
            return;
        }
        if !top && is_repetition(expr) {
            nested.push(expr);
            return;
        }
        if let Some(tc) = Tc::of(expr) {
            for act in tc.map_actions() {
                if let Ok(vars) = functions::code_variables(&code_of(act), &self.index.prefixes) {
                    direct.extend(vars.into_iter().filter(|v| v != functions::NODE_ARGUMENT));
                }
            }
            if let (true, Some(ve)) = (tc.references_shape(), tc.value_expr) {
                self.collect_shape(ve, direct, nested, seen);
            }
        } else {
            for e in sub_expressions(expr) {
                self.collect(e, direct, nested, seen, false);
            }
        }
    }

    fn collect_shape(
        &self,
        se: &'a ShapeExpr,
        direct: &mut BTreeSet<String>,
        nested: &mut Vec<&'a TripleExpr>,
        seen: &mut HashSet<usize>,
    ) {
        if let ShapeExpr::Ref(label) = se {
            if let Some(decl) = self.index.declaration(label) {
                for o in self.index.extension_candidates(decl) {
                    self.collect_shape(&o.shape_expr, direct, nested, seen);
                }
            }
            return;
        }
        if !seen.insert(key(se)) {
            return;
        }
        match se {
            ShapeExpr::Shape(shape) => {
                for p in self.index.shape_parts(shape) {
                    if let Some(w) = &p.expression {
                        self.collect(&w.te, direct, nested, seen, false);
                    }
                }
            },
            ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    self.collect_shape(&w.se, direct, nested, seen);
                }
            },
            _ => {},
        }
    }
}

/// The quads without repeats, and each one's first source.
fn distinct(quads: &[MapTriple], sources: &[Source]) -> (Vec<MapTriple>, Vec<Source>) {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    let mut prov = Vec::new();
    for (q, s) in quads.iter().zip(sources) {
        if seen.insert(q.clone()) {
            out.push(q.clone());
            prov.push(s.clone());
        }
    }
    (out, prov)
}

fn contains<G: NeighsRDF>(graph: &G, t: &MapTriple) -> Result<bool, ShExMapError> {
    let s = G::term_as_subject(&G::object_as_term(&t.subject)).map_err(ShExMapError::rdf)?;
    let p: G::IRI = t.predicate.clone().into();
    let o = G::object_as_term(&t.object);
    graph.contains(&s, &p, &o).map_err(ShExMapError::rdf)
}

/// Add a triple to a graph being built.
pub fn add_triple<G: BuildRDF>(graph: &mut G, t: &MapTriple) -> Result<(), ShExMapError> {
    let s = G::Subject::try_from(t.subject.clone())
        .map_err(|_| ShExMapError::rdf(format!("{} cannot be a subject", n3(&t.subject))))?;
    let p: G::IRI = t.predicate.clone().into();
    let o: G::Term = t.object.clone().into();
    graph.add_triple(s, p, o).map_err(ShExMapError::rdf)
}

fn remove_triple<G: BuildRDF>(graph: &mut G, t: &MapTriple) -> Result<(), ShExMapError> {
    let s = G::Subject::try_from(t.subject.clone())
        .map_err(|_| ShExMapError::rdf(format!("{} cannot be a subject", n3(&t.subject))))?;
    let p: G::IRI = t.predicate.clone().into();
    let o: G::Term = t.object.clone().into();
    graph.remove_triple(s, p, o).map_err(ShExMapError::rdf)
}

/// Materialize `root` as an instance of the output schema from `bindings`, into a new graph.
///
/// A convenience over [`Materializer`] (which also exposes the alternative materializations).
pub fn materialize<G: BuildRDF>(
    schema: &Schema,
    bindings: &Bindings,
    root: Option<&Object>,
    start: Option<&ShapeExprLabel>,
    options: Options,
) -> Result<G, ShExMapError> {
    let mut graph = G::empty();
    let mut m = Materializer::new(schema, options);
    for (prefix, ns) in m.prefixes() {
        graph.add_prefix(prefix, &IriS::new_unchecked(ns));
    }
    for t in m.materialize(&bindings.tree, root, start)? {
        add_triple(&mut graph, &t)?;
    }
    Ok(graph)
}
