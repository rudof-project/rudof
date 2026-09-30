//! Static checks of a ShExMap schema pair, before any data.
//!
//! Given the input schema and the output schema, [`analyse`] says whether every output
//! repetition has a well-formed iteration scope, whether every variable the output reads is
//! bound in the input where it can be read from, and which bindings go unused -- the
//! questions materialization would otherwise answer at run time, one graph at a time.
//!
//! A variable's **site** is the chain of repeated constraints and groups of the input schema
//! that enclose its `%Map` code: it is bound once per iteration of the innermost one, or once
//! for the whole tree when the chain is empty.  An output repetition's **iteration scope** is
//! the deepest site among the variables its body reads, nested repetitions counting through
//! their parent site; it is ill-formed when two of those sites lie in unrelated lists.  A
//! constraint may read a variable whose site is an ancestor of (or equal to) the scope it is
//! evaluated at; a deeper site means "which one?", and an unrelated one means the value is
//! not in scope.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::ast::{Schema, ShapeExpr, ShapeExprLabel, TripleExpr};
use crate::shexmap::error::ShExMapError;
use crate::shexmap::functions::{self, Prefixes};
use crate::shexmap::schema::{
    SchemaIndex, Tc, code_of, is_repeated, is_repetition, key, label_of, label_text, shape_expr_kind, sub_expressions,
};

/// The keys of the enclosing repeated expressions, outermost first.
pub type Site = Vec<usize>;

/// What [`analyse`] found.  `errors` make the pair not mappable; `warnings` do not.
/// Messages write IRIs with the schemas' prefixes.
#[derive(Debug, Clone, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    /// Input variable -> site.
    pub bound: BTreeMap<String, Site>,
    /// Variables the output reads.
    pub read: BTreeSet<String>,
    /// Repeated expression key -> name.
    pub labels: HashMap<usize, String>,
    pub prefixes: Prefixes,
}

impl Report {
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn where_(&self, site: &[usize]) -> String {
        if site.is_empty() {
            "the root".to_string()
        } else {
            format!(
                "each {}",
                site.iter()
                    .map(|k| self.labels.get(k).map_or("?", String::as_str))
                    .collect::<Vec<_>>()
                    .join(" / ")
            )
        }
    }

    /// `text` with every namespace the schemas declare written as its prefix.
    pub fn short(&self, text: &str) -> String {
        let mut entries: Vec<(&String, &String)> = self.prefixes.iter().collect();
        entries.sort_by_key(|(_, ns)| std::cmp::Reverse(ns.len()));
        let mut out = text.to_string();
        for (prefix, ns) in entries {
            if !ns.is_empty() {
                out = out.replace(ns, &format!("{prefix}:"));
            }
        }
        out
    }

    fn error(&mut self, message: String) {
        let m = self.short(&message);
        self.errors.push(m);
    }

    fn warn(&mut self, message: String) {
        let m = self.short(&message);
        self.warnings.push(m);
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() && self.warnings.is_empty() {
            return f.write_str("ok: the schemas map coherently");
        }
        let mut lines: Vec<String> = self.errors.iter().map(|e| format!("error: {e}")).collect();
        lines.extend(self.warnings.iter().map(|w| format!("warning: {w}")));
        f.write_str(&lines.join("\n"))
    }
}

/// Check that `output` can be materialized coherently from what `input` binds.
///
/// * `input_start`, `output_start`: shape labels; default the schemas' start shapes
/// * `static_vars`: variable IRIs that will be supplied as static variables
pub fn analyse(
    input: &Schema,
    output: &Schema,
    input_start: Option<&ShapeExprLabel>,
    output_start: Option<&ShapeExprLabel>,
    static_vars: &[String],
) -> Result<Report, ShExMapError> {
    let inp_index = SchemaIndex::new(input);
    let out_index = SchemaIndex::new(output);
    let mut prefixes = out_index.prefixes.clone();
    for (k, v) in &inp_index.prefixes {
        prefixes.entry(k.clone()).or_insert_with(|| v.clone());
    }
    let mut report = Report {
        prefixes,
        ..Report::default()
    };
    let inp_start = inp_index.start_expr(input_start)?;
    let mut inp = Input {
        index: &inp_index,
        report: &mut report,
        seen: HashSet::new(),
    };
    inp.shape_expr(&inp_start, &[]);
    let out_start = out_index.start_expr(output_start)?;
    let statics: HashSet<String> = static_vars.iter().cloned().collect();
    let mut out = Output {
        index: &out_index,
        report: &mut report,
        statics: statics.clone(),
        scopes: HashMap::new(),
        ill_formed: HashSet::new(),
        active: HashSet::new(),
    };
    out.shape_expr(&out_start, &[]);
    let bound = report.bound.clone();
    for (v, site) in &bound {
        if !report.read.contains(v) {
            let w = report.where_(site);
            report.warn(format!("{v} is bound (at {w}) but the output never reads it"));
        }
    }
    let mut unread: Vec<&String> = statics.iter().filter(|s| !report.read.contains(*s)).collect();
    unread.sort();
    for v in unread {
        report.warn(format!("static variable {v} is never read"));
    }
    Ok(report)
}

/// (variables bound or read by a constraint's Map codes, id() arguments) -- id() is not a
/// variable code; its arguments are returned apart.
fn variables(tc: &Tc<'_>, prefixes: &Prefixes) -> (Vec<String>, Vec<String>) {
    let mut plain = Vec::new();
    let mut keys = Vec::new();
    for act in tc.map_actions() {
        let code = code_of(act);
        if functions::is_key_code(&code) {
            if let Ok(args) = functions::key_arguments(&code, prefixes) {
                keys.extend(args.into_iter().filter(|a| a != functions::NODE_ARGUMENT));
            }
        } else if functions::is_function_call(&code) {
            if let Ok(vars) = functions::variables_of(&code, prefixes) {
                plain.extend(vars);
            }
        } else if let Ok(v) = functions::expand_variable(&code, prefixes) {
            plain.push(v);
        }
    }
    (plain, keys)
}

fn has_node_argument(tc: &Tc<'_>, prefixes: &Prefixes) -> bool {
    tc.map_actions().iter().any(|a| {
        let code = code_of(a);
        functions::is_key_code(&code)
            && functions::key_arguments(&code, prefixes)
                .is_ok_and(|args| args.iter().any(|x| x == functions::NODE_ARGUMENT))
    })
}

/// Where the input schema binds each variable.
struct Input<'a> {
    index: &'a SchemaIndex,
    report: &'a mut Report,
    seen: HashSet<usize>,
}

impl<'a> Input<'a> {
    fn shape_expr(&mut self, se: &'a ShapeExpr, site: &[usize]) {
        if let ShapeExpr::Ref(label) = se {
            if let Some(decl) = self.index.declaration(label) {
                for o in self.index.extension_candidates(decl) {
                    self.shape_expr(&o.shape_expr, site);
                }
            }
            return;
        }
        if !self.seen.insert(key(se)) {
            return;
        }
        match se {
            ShapeExpr::Shape(shape) => {
                for p in self.index.shape_parts(shape) {
                    if let Some(w) = &p.expression {
                        self.expression(&w.te, site);
                    }
                }
            },
            ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    self.shape_expr(&w.se, site);
                }
            },
            _ => {},
        }
    }

    fn expression(&mut self, expr: &'a TripleExpr, site: &[usize]) {
        let Some(expr) = self.index.resolve(expr) else { return };
        let mut site: Site = site.to_vec();
        if is_repeated(expr) {
            self.report.labels.insert(key(expr), label_of(expr, self.index));
            site.push(key(expr));
        }
        if let Some(tc) = Tc::of(expr) {
            let (plain, _) = variables(&tc, &self.report.prefixes);
            for v in plain {
                let at = self
                    .report
                    .bound
                    .entry(v.clone())
                    .or_insert_with(|| site.clone())
                    .clone();
                if at != site {
                    let (a, b) = (self.report.where_(&at), self.report.where_(&site));
                    self.report.error(format!(
                        "{v} is bound at two places of the input schema, {a} and {b}; a variable must have one binding site"
                    ));
                }
            }
            if let (true, Some(ve)) = (tc.references_shape(), tc.value_expr) {
                self.shape_expr(ve, &site);
            }
        } else {
            for e in sub_expressions(expr) {
                self.expression(e, &site);
            }
        }
    }
}

/// Where the output schema reads each variable, and what each repetition iterates.
struct Output<'a> {
    index: &'a SchemaIndex,
    report: &'a mut Report,
    statics: HashSet<String>,
    scopes: HashMap<usize, Option<Site>>,
    ill_formed: HashSet<usize>,
    active: HashSet<usize>,
}

impl<'a> Output<'a> {
    fn shape_expr(&mut self, se: &'a ShapeExpr, scope: &[usize]) {
        if let ShapeExpr::Ref(label) = se {
            let Some(decl) = self.index.declaration(label) else {
                self.report.error(format!(
                    "the output schema references {}, which it does not define",
                    label_text(label)
                ));
                return;
            };
            let options = self.index.extension_candidates(decl);
            if options.is_empty() {
                self.report
                    .error(format!("{} is abstract and nothing extends it", label_text(label)));
            }
            for o in options {
                self.shape_expr(&o.shape_expr, scope);
            }
            return;
        }
        let k = key(se);
        if !self.active.insert(k) {
            return; // a recursive shape: the first pass covered it
        }
        match se {
            ShapeExpr::Shape(shape) => {
                for p in self.index.shape_parts(shape) {
                    if let Some(w) = &p.expression {
                        self.expression(&w.te, scope);
                    }
                }
            },
            ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    if !matches!(w.se, ShapeExpr::NodeConstraint(_)) {
                        self.shape_expr(&w.se, scope);
                    }
                }
            },
            ShapeExpr::NodeConstraint(_) => {},
            other => self
                .report
                .error(format!("{} cannot be materialized", shape_expr_kind(other))),
        }
        self.active.remove(&k);
    }

    fn expression(&mut self, expr: &'a TripleExpr, scope: &[usize]) {
        let Some(expr) = self.index.resolve(expr) else { return };
        let body_scope: Site = if is_repetition(expr) {
            let iterates = self.iteration_scope(expr);
            if self.ill_formed.contains(&key(expr)) {
                return; // its structure is the error; its reads would only repeat it
            }
            match iterates {
                Some(it) if it.len() > scope.len() => it,
                _ => scope.to_vec(),
            }
        } else {
            scope.to_vec()
        };
        if let Some(tc) = Tc::of(expr) {
            self.constraint(&tc, &body_scope);
        } else {
            for e in sub_expressions(expr) {
                self.expression(e, &body_scope);
            }
        }
    }

    fn constraint(&mut self, tc: &Tc<'a>, scope: &[usize]) {
        let prefixes = self.report.prefixes.clone();
        let (plain, keys) = variables(tc, &prefixes);
        let acts = tc.map_actions();
        let pred = tc.predicate_str();
        if !keys.is_empty() || acts.iter().any(|a| functions::is_key_code(&code_of(a))) {
            if acts.len() > 1 {
                self.report.error(format!("id() must be the only Map code on {pred}"));
            }
            if !tc.references_shape() {
                self.report
                    .error(format!("id() on {pred} needs a shape-valued constraint"));
            }
            if has_node_argument(tc, &prefixes) && scope.is_empty() {
                self.report.error(format!(
                    "id(@node) on {pred} is read at the root, where no input iteration provides a node"
                ));
            }
        }
        for v in plain.iter().chain(keys.iter()) {
            self.read_variable(v, &pred, scope);
        }
        if let (true, Some(ve)) = (tc.references_shape(), tc.value_expr) {
            self.shape_expr(ve, scope);
        }
    }

    fn read_variable(&mut self, v: &str, pred: &str, scope: &[usize]) {
        self.report.read.insert(v.to_string());
        if self.statics.contains(v) {
            return;
        }
        let Some(site) = self.report.bound.get(v).cloned() else {
            self.report
                .error(format!("{pred} reads {v}, which the input schema never binds"));
            return;
        };
        if scope.len() >= site.len() && scope[..site.len()] == site[..] {
            return; // bound here or above: readable
        }
        let (at, here) = (self.report.where_(&site), self.report.where_(scope));
        if site.len() >= scope.len() && site[..scope.len()] == scope[..] {
            self.report.error(format!(
                "{pred} reads {v}, bound once per {at}, from {here}: which one?  A repetition over it is needed"
            ));
        } else {
            self.report.error(format!(
                "{pred} reads {v}, bound at {at}, from {here}, which is not below it"
            ));
        }
    }

    fn iteration_scope(&mut self, expr: &'a TripleExpr) -> Option<Site> {
        let k = key(expr);
        if let Some(found) = self.scopes.get(&k) {
            return found.clone();
        }
        self.scopes.insert(k, None); // a recursive repetition counts as scope-free while computed
        let mut direct: BTreeSet<String> = BTreeSet::new();
        let mut nested: Vec<&'a TripleExpr> = Vec::new();
        let mut seen = HashSet::new();
        self.collect(expr, &mut direct, &mut nested, &mut seen, true);
        let mut candidates: Vec<(Site, String)> = Vec::new();
        for v in &direct {
            if let Some(site) = self.report.bound.get(v)
                && !candidates.iter().any(|(c, _)| c == site)
            {
                candidates.push((site.clone(), v.clone()));
            }
        }
        for r in &nested {
            if let Some(sub) = self.iteration_scope(r)
                && !sub.is_empty()
            {
                let parent = sub[..sub.len() - 1].to_vec();
                if !candidates.iter().any(|(c, _)| *c == parent) {
                    candidates.push((parent, format!("the repetition over {}", label_of(r, self.index))));
                }
            }
        }
        let mut result: Option<Site> = None;
        if let Some((deepest, deepest_why)) = candidates.iter().max_by_key(|(s, _)| s.len()).cloned() {
            result = Some(deepest.clone());
            for (site, why) in &candidates {
                if deepest.len() < site.len() || deepest[..site.len()] != site[..] {
                    let (a, b) = (self.report.where_(site), self.report.where_(&deepest));
                    let name = label_of(expr, self.index);
                    self.report.error(format!(
                        "the repetition over {name} reads from unrelated lists: {why} is bound at {a} and {deepest_why} at {b}"
                    ));
                    self.ill_formed.insert(k);
                    self.report.read.extend(direct.iter().cloned());
                    break;
                }
            }
        }
        self.scopes.insert(k, result.clone());
        result
    }

    fn collect(
        &mut self,
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
            let (plain, keys) = variables(&tc, &self.report.prefixes);
            direct.extend(plain);
            direct.extend(keys);
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
        &mut self,
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
