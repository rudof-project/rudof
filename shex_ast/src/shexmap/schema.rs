//! A ShEx schema (AST) indexed the way ShExMap walks it: shapes by label, labelled triple
//! expressions, extensions, and the Map actions and cardinalities of triple constraints.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use prefixmap::{IriRef, PrefixMap};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;

use crate::ast::{
    NodeConstraint, ObjectValue, Schema, SemAct, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, TripleExpr,
    TripleExprLabel, ValueSetValue,
};
use crate::ir::actions::semantic_action_context::SemanticActionContext;
use crate::ir::ast2ir::compile_node_constraint;
use crate::shexmap::error::ShExMapError;
use crate::shexmap::functions::Prefixes;
use crate::{Cond, Node};

/// The Map extension's IRI, as the ShExMap examples write it.
pub const MAP_EXTENSION: &str = "http://shex.io/extensions/Map/#";

/// Whether a semantic action's name is the Map extension (with or without the trailing `#`).
pub fn is_map_action(act: &SemAct) -> bool {
    match act.name() {
        IriRef::Iri(iri) => iri.as_str().trim_end_matches('#') == MAP_EXTENSION.trim_end_matches('#'),
        IriRef::Prefixed { .. } => false,
    }
}

/// The Map actions of a triple constraint (or any expression carrying semantic actions).
pub fn map_actions(acts: Option<&Vec<SemAct>>) -> Vec<&SemAct> {
    acts.map(|v| v.iter().filter(|a| is_map_action(a)).collect())
        .unwrap_or_default()
}

/// The code of a semantic action, empty when it has none.
pub fn code_of(act: &SemAct) -> String {
    act.code().unwrap_or_default()
}

/// A stable identity for an expression of the indexed schema (its address).
pub fn key<T>(x: &T) -> usize {
    std::ptr::from_ref(x).addr()
}

/// A view of a triple constraint: the fields ShExMap reads, with the predicate as an IRI.
#[derive(Debug, Clone, Copy)]
pub struct Tc<'a> {
    pub expr: &'a TripleExpr,
    pub predicate: &'a IriRef,
    pub inverse: bool,
    pub value_expr: Option<&'a ShapeExpr>,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub sem_acts: Option<&'a Vec<SemAct>>,
}

impl<'a> Tc<'a> {
    pub fn of(expr: &'a TripleExpr) -> Option<Tc<'a>> {
        match expr {
            TripleExpr::TripleConstraint {
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                ..
            } => Some(Tc {
                expr,
                predicate,
                inverse: inverse.unwrap_or(false),
                value_expr: value_expr.as_deref(),
                min: *min,
                max: *max,
                sem_acts: sem_acts.as_ref(),
            }),
            _ => None,
        }
    }

    pub fn predicate_iri(&self) -> IriS {
        iri_of(self.predicate)
    }

    pub fn predicate_str(&self) -> String {
        match self.predicate {
            IriRef::Iri(iri) => iri.as_str().to_string(),
            IriRef::Prefixed { prefix, local } => format!("{prefix}:{local}"),
        }
    }

    pub fn map_actions(&self) -> Vec<&'a SemAct> {
        map_actions(self.sem_acts)
    }

    /// Whether the value expression can bind anything: anything but a node constraint.
    pub fn references_shape(&self) -> bool {
        references_shape(self.value_expr)
    }
}

/// An IRI reference as an IRI; a prefixed name that was never resolved is kept as text.
pub fn iri_of(iri: &IriRef) -> IriS {
    match iri {
        IriRef::Iri(iri) => iri.clone(),
        IriRef::Prefixed { prefix, local } => IriS::new_unchecked(&format!("{prefix}:{local}")),
    }
}

/// Whether a value expression can bind anything: anything but a node constraint.
pub fn references_shape(se: Option<&ShapeExpr>) -> bool {
    matches!(se, Some(se) if !matches!(se, ShapeExpr::NodeConstraint(_)))
}

/// `(min, max)` with defaults applied; an unbounded max is `None`.
pub fn cardinality(expr: &TripleExpr) -> (usize, Option<usize>) {
    let (min, max) = min_max(expr);
    let min = usize::try_from(min.unwrap_or(1)).unwrap_or(0);
    let max = match max {
        None => Some(1),
        Some(-1) => None,
        Some(m) => Some(usize::try_from(m).unwrap_or(0)),
    };
    (min, max)
}

pub fn min_max(expr: &TripleExpr) -> (Option<i32>, Option<i32>) {
    match expr {
        TripleExpr::EachOf { min, max, .. }
        | TripleExpr::OneOf { min, max, .. }
        | TripleExpr::TripleConstraint { min, max, .. } => (*min, *max),
        TripleExpr::Ref(_) => (None, None),
    }
}

/// A repeated expression: one whose max is not exactly one (`*`, `+`, `{2,5}`, `{0,1}` ...).
pub fn is_repeated(expr: &TripleExpr) -> bool {
    matches!(min_max(expr).1, Some(m) if m != 1)
}

/// Any cardinality but exactly one, as the materializer treats it (`?` included).
pub fn is_repetition(expr: &TripleExpr) -> bool {
    let (min, max) = min_max(expr);
    !(matches!(min, None | Some(1)) && matches!(max, None | Some(1)))
}

/// The sub-expressions of a group, or none.
pub fn sub_expressions(expr: &TripleExpr) -> Vec<&TripleExpr> {
    match expr {
        TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
            expressions.iter().map(|w| &w.te).collect()
        },
        _ => Vec::new(),
    }
}

/// The name of an expression's kind, for messages.
pub fn kind_name(expr: &TripleExpr) -> &'static str {
    match expr {
        TripleExpr::EachOf { .. } => "EachOf",
        TripleExpr::OneOf { .. } => "OneOf",
        TripleExpr::TripleConstraint { .. } => "TripleConstraint",
        TripleExpr::Ref(_) => "TripleExprRef",
    }
}

pub fn shape_expr_kind(se: &ShapeExpr) -> &'static str {
    match se {
        ShapeExpr::ShapeOr { .. } => "ShapeOr",
        ShapeExpr::ShapeAnd { .. } => "ShapeAnd",
        ShapeExpr::ShapeNot { .. } => "ShapeNot",
        ShapeExpr::NodeConstraint(_) => "NodeConstraint",
        ShapeExpr::Shape(_) => "Shape",
        ShapeExpr::External => "ShapeExternal",
        ShapeExpr::Ref(_) => "ShapeRef",
    }
}

/// A short name for a triple expression: its first predicate(s).
pub fn label_of(expr: &TripleExpr, index: &SchemaIndex) -> String {
    let mut found: Vec<String> = Vec::new();
    let mut stack = vec![expr];
    while let Some(e) = stack.pop() {
        if found.len() >= 2 {
            break;
        }
        let Some(e) = index.resolve(e) else { continue };
        if let Some(tc) = Tc::of(e) {
            found.push(format!("{}{}", if tc.inverse { "^" } else { "" }, tc.predicate_str()));
        } else {
            stack.extend(sub_expressions(e).into_iter().rev());
        }
    }
    match found.len() {
        0 => "(group)".to_string(),
        1 => found.remove(0),
        _ => format!("({})", found.join(", ")),
    }
}

/// The key of a shape label: its IRI, `_:label`, or `START`.
pub fn label_key(label: &ShapeExprLabel) -> String {
    match label {
        ShapeExprLabel::IriRef { value } => iri_of(value).as_str().to_string(),
        ShapeExprLabel::BNode { value } => format!("_:{value}"),
        ShapeExprLabel::Start => "START".to_string(),
    }
}

pub fn label_text(label: &ShapeExprLabel) -> String {
    match label {
        ShapeExprLabel::IriRef { value } => format!("<{}>", iri_of(value).as_str()),
        other => label_key(other),
    }
}

fn triple_expr_label_key(label: &TripleExprLabel) -> String {
    match label {
        TripleExprLabel::IriRef { value } => iri_of(value).as_str().to_string(),
        TripleExprLabel::BNode { value } => format!("_:{value}"),
    }
}

fn expr_label(expr: &TripleExpr) -> Option<&TripleExprLabel> {
    match expr {
        TripleExpr::EachOf { id, .. } | TripleExpr::OneOf { id, .. } | TripleExpr::TripleConstraint { id, .. } => {
            id.as_ref()
        },
        TripleExpr::Ref(_) => None,
    }
}

/// The one value a node constraint names (`[ex:a]` or `["x"]`), if any.
pub fn single_value(se: Option<&ShapeExpr>) -> Option<Object> {
    let Some(ShapeExpr::NodeConstraint(nc)) = se else {
        return None;
    };
    let values = nc.values()?;
    if values.len() != 1 {
        return None;
    }
    match &values[0] {
        ValueSetValue::ObjectValue(ObjectValue::IriRef(iri)) => Some(Object::iri(iri_of(iri))),
        ValueSetValue::ObjectValue(ObjectValue::Literal(lit)) => Some(Object::literal(lit.clone())),
        _ => None, // stems, ranges and languages do not name one value
    }
}

/// The schema, indexed.  Expressions are addressed by [`key`] into this structure, so it
/// must outlive every key taken from it.
pub struct SchemaIndex {
    pub shapes: Vec<ShapeDecl>,
    pub start: Option<ShapeExpr>,
    pub prefixes: Prefixes,
    pub prefixmap: PrefixMap,
    pub base: Option<IriS>,
    by_label: HashMap<String, usize>,
    triple_exprs: HashMap<String, TripleExpr>,
    children: HashMap<String, Vec<usize>>,
    conds: RefCell<HashMap<usize, Option<Cond>>>,
}

impl SchemaIndex {
    pub fn new(schema: &Schema) -> SchemaIndex {
        let shapes = schema.shapes().unwrap_or_default();
        let prefixmap = schema.prefixmap().unwrap_or_default();
        let prefixes: Prefixes = prefixmap
            .iter()
            .map(|(alias, iri)| (alias.clone(), iri.as_str().to_string()))
            .collect();
        let mut by_label = HashMap::new();
        let mut children: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, decl) in shapes.iter().enumerate() {
            by_label.insert(label_key(&decl.id), i);
            for shape in shapes_in(&decl.shape_expr) {
                for base in shape.extends() {
                    children.entry(label_key(base)).or_default().push(i);
                }
            }
        }
        let mut triple_exprs = HashMap::new();
        for decl in &shapes {
            collect_triple_expr_labels(&decl.shape_expr, &mut triple_exprs);
        }
        if let Some(start) = schema.start() {
            collect_triple_expr_labels(&start, &mut triple_exprs);
        }
        SchemaIndex {
            shapes,
            start: schema.start(),
            prefixes,
            prefixmap,
            base: schema.base(),
            by_label,
            triple_exprs,
            children,
            conds: RefCell::new(HashMap::new()),
        }
    }

    /// The declaration for a shape label, if the schema has one.
    pub fn declaration(&self, label: &ShapeExprLabel) -> Option<&ShapeDecl> {
        self.by_label.get(&label_key(label)).map(|&i| &self.shapes[i])
    }

    /// The shape expression a `start=` or `&label` argument names: the schema's start or a label.
    pub fn start_expr(&self, start: Option<&ShapeExprLabel>) -> Result<ShapeExpr, ShExMapError> {
        match start {
            Some(label) if !matches!(label, ShapeExprLabel::Start) => Ok(ShapeExpr::Ref(label.clone())),
            _ => self
                .start
                .clone()
                .ok_or_else(|| ShExMapError::schema("the schema has no start shape; name one")),
        }
    }

    /// A triple expression, an inclusion (`&label`) resolved to what it names.
    pub fn resolve<'a>(&'a self, expr: &'a TripleExpr) -> Option<&'a TripleExpr> {
        match expr {
            TripleExpr::Ref(label) => self.triple_exprs.get(&triple_expr_label_key(label)),
            other => Some(other),
        }
    }

    /// What satisfying `decl` can mean, most specific first: its non-abstract extensions
    /// (deepest first), then the declaration itself unless it is abstract.
    pub fn extension_candidates<'a>(&'a self, decl: &'a ShapeDecl) -> Vec<&'a ShapeDecl> {
        let mut found: Vec<&ShapeDecl> = Vec::new();
        let mut seen: HashSet<usize> = HashSet::new();
        self.descendants(&label_key(&decl.id), &mut seen, &mut found);
        found.reverse();
        if !decl.is_abstract {
            found.push(decl);
        }
        found
    }

    fn descendants<'a>(&'a self, label: &str, seen: &mut HashSet<usize>, out: &mut Vec<&'a ShapeDecl>) {
        if let Some(kids) = self.children.get(label) {
            for &i in kids {
                if seen.insert(i) {
                    let decl = &self.shapes[i];
                    if !decl.is_abstract {
                        out.push(decl);
                    }
                    self.descendants(&label_key(&decl.id), seen, out);
                }
            }
        }
    }

    /// The shapes whose triple expressions share a node's neighbourhood when it matches
    /// `shape`: those of everything `shape` EXTENDS (transitively), then `shape` itself.
    pub fn shape_parts<'a>(&'a self, shape: &'a Shape) -> Vec<&'a Shape> {
        let mut seen = HashSet::new();
        let mut parts = Vec::new();
        self.parts_into(shape, &mut seen, &mut parts);
        parts
    }

    fn parts_into<'a>(&'a self, shape: &'a Shape, seen: &mut HashSet<String>, parts: &mut Vec<&'a Shape>) {
        for base in shape.extends() {
            let k = label_key(base);
            if !seen.insert(k) {
                continue;
            }
            if let Some(decl) = self.declaration(base) {
                for s in shapes_in(&decl.shape_expr) {
                    self.parts_into(s, seen, parts);
                }
            }
        }
        parts.push(shape);
    }

    /// The triple constraints of a shape's own expression (not of the shapes nested in them).
    pub fn own_tcs<'a>(&'a self, shape: &'a Shape) -> Vec<Tc<'a>> {
        let mut out = Vec::new();
        let mut stack: Vec<&TripleExpr> = shape.expression.iter().map(|w| &w.te).collect();
        while let Some(e) = stack.pop() {
            let Some(e) = self.resolve(e) else { continue };
            if let Some(tc) = Tc::of(e) {
                out.push(tc);
            } else {
                stack.extend(sub_expressions(e));
            }
        }
        out
    }

    /// The repeated constraints and groups directly under `exprs` (not inside another
    /// repeated expression), in schema order: one list each in a scope's tree.
    pub fn repeated_expressions<'a>(&'a self, exprs: &[&'a TripleExpr]) -> Vec<&'a TripleExpr> {
        let mut out = Vec::new();
        for e in exprs {
            self.repeated_into(e, &mut out);
        }
        out
    }

    fn repeated_into<'a>(&'a self, expr: &'a TripleExpr, out: &mut Vec<&'a TripleExpr>) {
        let Some(e) = self.resolve(expr) else { return };
        if is_repeated(e) {
            out.push(e);
        } else {
            for sub in sub_expressions(e) {
                self.repeated_into(sub, out);
            }
        }
    }

    /// Whether `value` satisfies a node constraint.
    pub fn satisfies_nc(&self, nc: &NodeConstraint, value: &Object) -> Result<bool, ShExMapError> {
        let k = key(nc);
        if !self.conds.borrow().contains_key(&k) {
            let cond = compile_node_constraint(nc, &self.prefixmap, &self.base)
                .map_err(|e| ShExMapError::schema(format!("node constraint: {e}")))?;
            self.conds.borrow_mut().insert(k, Some(cond));
        }
        let conds = self.conds.borrow();
        let Some(Some(cond)) = conds.get(&k) else {
            return Ok(true);
        };
        let node = Node::new(value.clone());
        Ok(cond.matches(&node, &SemanticActionContext::default()).is_ok())
    }

    /// Every triple constraint reachable from a shape expression.
    pub fn triple_constraints<'a>(&'a self, se: &'a ShapeExpr) -> Vec<Tc<'a>> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        self.tcs_of_shape_expr(se, &mut seen, &mut out);
        out
    }

    fn tcs_of_shape_expr<'a>(&'a self, se: &'a ShapeExpr, seen: &mut HashSet<usize>, out: &mut Vec<Tc<'a>>) {
        if !seen.insert(key(se)) {
            return;
        }
        match se {
            ShapeExpr::Shape(shape) => {
                if let Some(w) = &shape.expression {
                    self.tcs_of_expr(&w.te, seen, out);
                }
            },
            ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    self.tcs_of_shape_expr(&w.se, seen, out);
                }
            },
            ShapeExpr::ShapeNot { shape_expr } => self.tcs_of_shape_expr(&shape_expr.se, seen, out),
            ShapeExpr::Ref(label) => {
                if let Some(decl) = self.declaration(label) {
                    self.tcs_of_shape_expr(&decl.shape_expr, seen, out);
                }
            },
            ShapeExpr::NodeConstraint(_) | ShapeExpr::External => {},
        }
    }

    fn tcs_of_expr<'a>(&'a self, expr: &'a TripleExpr, seen: &mut HashSet<usize>, out: &mut Vec<Tc<'a>>) {
        let Some(e) = self.resolve(expr) else { return };
        if !seen.insert(key(e)) {
            return;
        }
        if let Some(tc) = Tc::of(e) {
            out.push(tc);
            if let Some(ve) = tc.value_expr {
                self.tcs_of_shape_expr(ve, seen, out);
            }
        } else {
            for sub in sub_expressions(e) {
                self.tcs_of_expr(sub, seen, out);
            }
        }
    }
}

/// The shapes a shape expression is made of: a shape, or the shapes of a conjunction.
pub fn shapes_in(se: &ShapeExpr) -> Vec<&Shape> {
    match se {
        ShapeExpr::Shape(s) => vec![s],
        ShapeExpr::ShapeAnd { shape_exprs } => shape_exprs.iter().flat_map(|w| shapes_in(&w.se)).collect(),
        _ => Vec::new(),
    }
}

fn collect_triple_expr_labels(se: &ShapeExpr, out: &mut HashMap<String, TripleExpr>) {
    match se {
        ShapeExpr::Shape(shape) => {
            if let Some(w) = &shape.expression {
                collect_expr_labels(&w.te, out);
            }
        },
        ShapeExpr::ShapeAnd { shape_exprs } | ShapeExpr::ShapeOr { shape_exprs } => {
            for w in shape_exprs {
                collect_triple_expr_labels(&w.se, out);
            }
        },
        ShapeExpr::ShapeNot { shape_expr } => collect_triple_expr_labels(&shape_expr.se, out),
        ShapeExpr::NodeConstraint(_) | ShapeExpr::External | ShapeExpr::Ref(_) => {},
    }
}

fn collect_expr_labels(expr: &TripleExpr, out: &mut HashMap<String, TripleExpr>) {
    if let Some(label) = expr_label(expr) {
        out.entry(triple_expr_label_key(label)).or_insert_with(|| expr.clone());
    }
    if let Some(tc) = Tc::of(expr) {
        if let Some(ve) = tc.value_expr {
            collect_triple_expr_labels(ve, out);
        }
    } else {
        for sub in sub_expressions(expr) {
            collect_expr_labels(sub, out);
        }
    }
}
