//! Collect ShExMap bindings from RDF that conforms to an input schema.
//!
//! **Binding trees** use the structure shex.js prints and reads.  A *scope* (what one node's
//! match binds) is an object mapping variable IRIs to terms, or, when the match has repeated
//! parts, an array whose first element is that object (possibly empty) and whose other
//! elements are *lists*; a list has one element per iteration of a repeated constraint or
//! group, each element a scope, so everything bound from one reading of a blood pressure
//! stays together.  A list's elements are uniform: all objects, or all arrays (a lone object
//! among arrays is written as `[object]`), so that a reader never has to guess whether an
//! array is a scope or a list.  Every repeated constraint or group of the shape gets its
//! list, in schema order, empty when nothing matched, so a list's position says which
//! expression it came from whatever the data.  A non-repeated nested shape merges into the
//! scope that matched it.  An element made by a repeated *shape-valued* constraint also
//! records the node the nested shape matched under the reserved key `"@node"`
//! ([`NODE_KEY`]; the subject for an inverse constraint).  It is not a variable:
//! [`BindingTree::frames`] leaves it out and no output constraint can read it.  It says
//! which input node an iteration came from, which is what tells two groups apart when their
//! bindings are alike.  [`Bindings`] wraps a tree, and its JSON is shex.js's.
//!
//! **Collecting** partitions each node's neighbourhood by an exhaustive search in schema
//! order (larger matches first), following the value expressions of the constraints, so the
//! search is also the conformance check.  When several partitions -- or several ways to
//! satisfy nested shapes -- lead to different bindings, the input schema is ambiguous for
//! mapping: [`bind`] returns the first and records how many there were, [`bind_all`] returns
//! them all, and `strict` turns ambiguity into an error.  Which parse is first follows the
//! order the neighbourhood's triples are tried in: by predicate, then by value, blank nodes
//! ordered by the arcs they carry rather than by the labels the parser gave them, so two
//! runs over the same document agree.
//!
//! EXTENDS is followed: a shape's neighbourhood is shared between its own expression and
//! the shapes it extends, and a reference to a shape with extensions binds through the most
//! specific extension the node satisfies.  Inverse triple constraints (`^p`) bind the
//! subject of the matched triple.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use rudof_iri::IriS;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::Triple as _;
use serde_json::{Map, Value};

use crate::ast::{Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr};
use crate::shexmap::error::ShExMapError;
use crate::shexmap::functions;
use crate::shexmap::schema::{
    SchemaIndex, Tc, cardinality, code_of, iri_of, is_repeated, key, label_text, sub_expressions,
};
use crate::shexmap::term::{MapTriple, is_resource, n3, term_from_json, term_to_json};

/// In an iteration's bindings: the node the nested shape matched.
pub const NODE_KEY: &str = "@node";

/// Distinct binding trees to collect before giving up counting.
pub const MAX_ALTERNATIVES: usize = 20;

/// Partitions of one neighbourhood to try.
pub const MAX_PARTITIONS: usize = 10_000;

/// A frame: the bindings one output constraint sees at once (see [`BindingTree::frames`]).
pub type Frame = BTreeMap<String, Object>;

// -- binding trees ------------------------------------------------------------------------

/// A scope of a binding tree (see the module documentation): its own bindings (`@node`
/// included, when recorded) and one list of iterations per repeated expression.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BindingTree {
    pub own: BTreeMap<String, Object>,
    pub lists: Vec<Vec<BindingTree>>,
}

impl BindingTree {
    /// The tree as shex.js's JSON: the object alone, or `[object, list, ...]`, each list's
    /// elements uniform (all objects or all arrays).
    pub fn to_json(&self) -> Value {
        let own = Value::Object(
            self.own
                .iter()
                .map(|(k, v)| (k.clone(), term_to_json(v)))
                .collect::<Map<String, Value>>(),
        );
        if self.lists.is_empty() {
            return own;
        }
        let mut arr = vec![own];
        for list in &self.lists {
            let wrap = list.iter().any(|it| !it.lists.is_empty());
            arr.push(Value::Array(
                list.iter()
                    .map(|it| {
                        let j = it.to_json();
                        if wrap && !j.is_array() {
                            Value::Array(vec![j])
                        } else {
                            j
                        }
                    })
                    .collect(),
            ));
        }
        Value::Array(arr)
    }

    /// A tree from JSON, accepting the two older layouts shex.js and PyShEx wrote: a root
    /// without own bindings written as its one list, and a nested scope written as a sibling.
    pub fn from_json(value: &Value) -> Result<BindingTree, ShExMapError> {
        match value {
            Value::Object(_) => scope_from_json(value),
            Value::Array(a) if is_scope_array(a) => scope_from_json(value),
            Value::Array(a) => {
                if a.iter().all(Value::is_object) {
                    // L1: shex.js's root, written as its one list
                    return Ok(BindingTree {
                        own: BTreeMap::new(),
                        lists: vec![iterations_from_json(a)?],
                    });
                }
                if a.iter().all(Value::is_array) {
                    if a.iter().all(|e| e.as_array().is_some_and(|x| is_scope_array(x))) {
                        return Ok(BindingTree {
                            own: BTreeMap::new(),
                            lists: vec![iterations_from_json(a)?],
                        });
                    }
                    if a.iter()
                        .all(|e| e.as_array().is_some_and(|x| x.iter().all(Value::is_object)))
                    {
                        // PyShEx before 2026-09-27: the root's lists without its object
                        let mut lists = Vec::new();
                        for e in a {
                            lists.push(iterations_from_json(e.as_array().expect("checked above"))?);
                        }
                        return Ok(BindingTree {
                            own: BTreeMap::new(),
                            lists,
                        });
                    }
                }
                Err(ShExMapError::binding_tree(format!("not a scope: {}", short(value))))
            },
            _ => Err(ShExMapError::binding_tree(format!("not a scope: {}", short(value)))),
        }
    }

    /// The tree flattened to the frame sequence, as shex.js's `normalizeBindingTree` does.
    ///
    /// A binding whose variable occurs once under a list level (a patient's name beside the
    /// list of their readings) is copied into every frame the sibling lists produce; `@node`
    /// is left out.
    pub fn frames(&self) -> Vec<Frame> {
        walk_scope(self).0
    }

    /// Every variable bound anywhere in the tree.
    pub fn variables(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        self.collect_variables(&mut out);
        out
    }

    fn collect_variables(&self, out: &mut BTreeSet<String>) {
        out.extend(self.own.keys().filter(|k| !k.starts_with('@')).cloned());
        for list in &self.lists {
            for it in list {
                it.collect_variables(out);
            }
        }
    }

    /// Every scope of the tree that recorded `@node`.
    pub fn nodes(&self) -> Vec<&BindingTree> {
        let mut out = Vec::new();
        self.collect_nodes(&mut out);
        out
    }

    fn collect_nodes<'a>(&'a self, out: &mut Vec<&'a BindingTree>) {
        if self.own.contains_key(NODE_KEY) {
            out.push(self);
        }
        for list in &self.lists {
            for it in list {
                it.collect_nodes(out);
            }
        }
    }

    /// A canonical text of the tree (JSON with sorted keys), for comparisons.
    pub fn canonical(&self) -> String {
        self.to_json().to_string()
    }
}

fn short(v: &Value) -> String {
    let s = v.to_string();
    if s.len() < 120 { s } else { format!("{}...", &s[..117]) }
}

fn is_scope_array(a: &[Value]) -> bool {
    !a.is_empty() && a[0].is_object() && a[1..].iter().all(Value::is_array)
}

fn own_from_json(obj: &Map<String, Value>) -> Result<BTreeMap<String, Object>, ShExMapError> {
    let mut own = BTreeMap::new();
    for (k, v) in obj {
        own.insert(k.clone(), term_from_json(v)?);
    }
    Ok(own)
}

fn scope_from_json(value: &Value) -> Result<BindingTree, ShExMapError> {
    let (obj, rest): (&Map<String, Value>, &[Value]) = match value {
        Value::Object(m) => (m, &[]),
        Value::Array(a) => match a.first() {
            Some(Value::Object(m)) => (m, &a[1..]),
            _ => return Err(ShExMapError::binding_tree(format!("not a scope: {}", short(value)))),
        },
        _ => return Err(ShExMapError::binding_tree(format!("not a scope: {}", short(value)))),
    };
    let mut lists = Vec::new();
    for list in rest {
        let Value::Array(items) = list else {
            return Err(ShExMapError::binding_tree(format!(
                "a scope's lists must be arrays: {}",
                short(list)
            )));
        };
        lists.push(iterations_from_json(items)?);
    }
    Ok(BindingTree {
        own: own_from_json(obj)?,
        lists,
    })
}

fn iterations_from_json(items: &[Value]) -> Result<Vec<BindingTree>, ShExMapError> {
    items.iter().map(iteration_from_json).collect()
}

fn iteration_from_json(elt: &Value) -> Result<BindingTree, ShExMapError> {
    match elt {
        Value::Object(_) => scope_from_json(elt),
        Value::Array(a) if is_scope_array(a) => scope_from_json(elt), // also L2: a nested scope as a sibling
        Value::Array(a) if a.iter().all(Value::is_object) => Ok(BindingTree {
            // shex.js, a nested scope with no own bindings whose one list was unwrapped
            own: BTreeMap::new(),
            lists: vec![iterations_from_json(a)?],
        }),
        _ => Err(ShExMapError::binding_tree(format!("not an iteration: {}", short(elt)))),
    }
}

type Walked = (Vec<Frame>, bool, HashMap<String, usize>);

fn walk_scope(scope: &BindingTree) -> Walked {
    let own: Frame = scope
        .own
        .iter()
        .filter(|(k, _)| k.as_str() != NODE_KEY)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let counts: HashMap<String, usize> = own.keys().map(|k| (k.clone(), 1)).collect();
    let leaf = (vec![own], true, counts);
    if scope.lists.is_empty() {
        return leaf;
    }
    let mut kids = vec![leaf];
    kids.extend(scope.lists.iter().map(|l| walk_list(l)));
    combine(kids)
}

fn walk_list(list: &[BindingTree]) -> Walked {
    combine(list.iter().map(walk_scope).collect())
}

fn combine(kids: Vec<Walked>) -> Walked {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (_, _, c) in &kids {
        for (k, n) in c {
            *counts.entry(k.clone()).or_insert(0) += n;
        }
    }
    if kids.iter().all(|(_, leaf, _)| *leaf) {
        // a plain sequence of frames
        return (kids.into_iter().flat_map(|(fs, _, _)| fs).collect(), false, counts);
    }
    let mut shared: Frame = BTreeMap::new();
    let mut ordered: Vec<(Vec<Frame>, bool)> = Vec::new();
    for (frames, leaf, _) in kids {
        if leaf {
            let mut rest: Frame = BTreeMap::new();
            if let Some(first) = frames.into_iter().next() {
                for (k, v) in first {
                    if counts.get(&k) == Some(&1) {
                        shared.insert(k, v);
                    } else {
                        rest.insert(k, v);
                    }
                }
            }
            if !rest.is_empty() {
                ordered.push((vec![rest], true));
            }
        } else {
            ordered.push((frames, false));
        }
    }
    let mut out: Vec<Frame> = Vec::new();
    for (frames, leaf) in ordered {
        for f in frames {
            if leaf {
                out.push(f);
            } else {
                let mut merged = shared.clone();
                merged.extend(f);
                out.push(merged);
            }
        }
    }
    if out.is_empty() && !shared.is_empty() {
        // a scope whose lists are all empty still has its own bindings
        out.push(shared);
    }
    (out, false, counts)
}

/// A binding tree, how many distinct ones the input allowed, and the triples the schema
/// matched to make it (`matched`: the subgraph it selected).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bindings {
    pub tree: BindingTree,
    pub alternatives: usize,
    pub matched: BTreeSet<MapTriple>,
}

impl Bindings {
    pub fn new(tree: BindingTree) -> Bindings {
        Bindings {
            tree,
            alternatives: 1,
            matched: BTreeSet::new(),
        }
    }

    pub fn ambiguous(&self) -> bool {
        self.alternatives > 1
    }

    pub fn to_json(&self) -> Value {
        self.tree.to_json()
    }

    pub fn from_json(value: &Value) -> Result<Bindings, ShExMapError> {
        Ok(Bindings::new(BindingTree::from_json(value)?))
    }

    /// The tree flattened to frames (see [`BindingTree::frames`]).
    pub fn frames(&self) -> Vec<Frame> {
        self.tree.frames()
    }

    pub fn variables(&self) -> BTreeSet<String> {
        self.tree.variables()
    }

    /// The bindings as JSON text, shex.js's format.
    pub fn dumps(&self, pretty: bool) -> String {
        if pretty {
            serde_json::to_string_pretty(&self.to_json())
        } else {
            serde_json::to_string(&self.to_json())
        }
        .expect("JSON values serialize")
    }

    pub fn loads(text: &str) -> Result<Bindings, ShExMapError> {
        let value: Value =
            serde_json::from_str(text).map_err(|e| ShExMapError::binding_tree(format!("bindings JSON: {e}")))?;
        Bindings::from_json(&value)
    }
}

// -- extraction -----------------------------------------------------------------------------

/// One scope while collecting: the bindings made while matching one node, one list of
/// iteration records per repeated constraint or group that matched under it, and the triples
/// matched here.
#[derive(Debug, Clone, Default)]
struct Record {
    vars: BTreeMap<String, Object>,
    lists: Vec<Vec<Record>>,
    triples: BTreeSet<MapTriple>,
}

impl Record {
    fn merged(&self, other: &Record) -> Record {
        let mut vars = self.vars.clone();
        vars.extend(other.vars.iter().map(|(k, v)| (k.clone(), v.clone())));
        let mut lists = self.lists.clone();
        lists.extend(other.lists.iter().cloned());
        let mut triples = self.triples.clone();
        triples.extend(other.triples.iter().cloned());
        Record { vars, lists, triples }
    }

    /// The triples matched here and in every nested scope.
    fn all_triples(&self) -> BTreeSet<MapTriple> {
        let mut out = self.triples.clone();
        for list in &self.lists {
            for r in list {
                out.extend(r.all_triples());
            }
        }
        out
    }

    fn is_empty(&self) -> bool {
        self.vars.keys().all(|k| k == NODE_KEY) && self.lists.iter().all(|l| l.iter().all(Record::is_empty))
    }

    fn tree(&self) -> BindingTree {
        BindingTree {
            own: self.vars.clone(),
            lists: self
                .lists
                .iter()
                .map(|l| l.iter().map(Record::tree).collect())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Out,
    In,
}

/// A triple of the neighbourhood, with the direction it was reached in.
#[derive(Debug, Clone)]
struct Dt {
    dir: Direction,
    triple: MapTriple,
}

#[derive(Debug, Clone)]
enum Item<'a> {
    /// A triple constraint matched one triple (an index into the neighbourhood).
    Tc { tc: Tc<'a>, at: usize },
    /// One iteration of a repeated group.
    Group { expr: &'a TripleExpr, items: Vec<Item<'a>> },
}

type Avail = BTreeSet<usize>;
type Visit<'k, 'a> = &'k mut dyn FnMut(&[Item<'a>], &Avail) -> Result<bool, ShExMapError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Want {
    /// Whether some partition exists.
    Exists,
    /// Every partition's bindings, up to the limit.
    All,
}

struct Extractor<'a, R: NeighsRDF> {
    index: &'a SchemaIndex,
    rdf: &'a R,
    limit: usize,
    active: RefCell<HashSet<(Object, usize)>>,
    checking: RefCell<HashSet<(Object, usize)>>,
    sat_cache: RefCell<HashMap<(Object, usize), bool>>,
    key_cache: RefCell<HashMap<(Object, u8), String>>,
}

fn value_of<'a>(tc: &Tc<'a>, t: &'a MapTriple) -> &'a Object {
    if tc.inverse { &t.subject } else { &t.object }
}

/// The k-element subsets of `0..n` in lexicographic order.
struct Combinations {
    n: usize,
    k: usize,
    current: Vec<usize>,
    done: bool,
}

impl Combinations {
    fn new(n: usize, k: usize) -> Combinations {
        Combinations {
            n,
            k,
            current: (0..k).collect(),
            done: k > n,
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        let out = self.current.clone();
        // advance: find the rightmost index that can move right
        let mut i = self.k;
        loop {
            if i == 0 {
                self.done = true;
                break;
            }
            i -= 1;
            if self.current[i] < self.n - self.k + i {
                self.current[i] += 1;
                for j in i + 1..self.k {
                    self.current[j] = self.current[j - 1] + 1;
                }
                break;
            }
        }
        Some(out)
    }
}

fn product(alternatives: &[Vec<Record>], limit: usize) -> Vec<Record> {
    let mut acc = vec![Record::default()];
    for alts in alternatives {
        if alts.is_empty() {
            return Vec::new();
        }
        let mut next = Vec::new();
        'outer: for a in &acc {
            for b in alts {
                next.push(a.merged(b));
                if next.len() >= limit {
                    break 'outer;
                }
            }
        }
        acc = next;
    }
    acc
}

fn dedupe(records: Vec<Record>, limit: usize) -> Vec<Record> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for r in records {
        if seen.insert(r.tree().canonical()) {
            out.push(r);
            if out.len() >= limit {
                break;
            }
        }
    }
    out
}

impl<'a, R: NeighsRDF> Extractor<'a, R> {
    fn new(index: &'a SchemaIndex, rdf: &'a R, limit: usize) -> Extractor<'a, R> {
        Extractor {
            index,
            rdf,
            limit: limit.max(1),
            active: RefCell::new(HashSet::new()),
            checking: RefCell::new(HashSet::new()),
            sat_cache: RefCell::new(HashMap::new()),
            key_cache: RefCell::new(HashMap::new()),
        }
    }

    // -- the graph ------------------------------------------------------------------------

    fn outgoing(&self, n: &Object) -> Result<Vec<MapTriple>, ShExMapError> {
        if !is_resource(n) {
            return Ok(Vec::new());
        }
        let term = R::object_as_term(n);
        let subj = R::term_as_subject(&term).map_err(ShExMapError::rdf)?;
        let mut out = Vec::new();
        for t in self.rdf.triples_with_subject(&subj).map_err(ShExMapError::rdf)? {
            let (_, p, o) = t.into_components();
            let pred: IriS = p.into();
            out.push(MapTriple::new(
                n.clone(),
                pred,
                R::term_as_object(&o).map_err(ShExMapError::rdf)?,
            ));
        }
        Ok(out)
    }

    fn incoming(&self, n: &Object) -> Result<Vec<MapTriple>, ShExMapError> {
        let term = R::object_as_term(n);
        let mut out = Vec::new();
        for t in self.rdf.triples_with_object(&term).map_err(ShExMapError::rdf)? {
            let (s, p, _) = t.into_components();
            let pred: IriS = p.into();
            let subject = R::term_as_object(&R::subject_as_term(&s)).map_err(ShExMapError::rdf)?;
            out.push(MapTriple::new(subject, pred, n.clone()));
        }
        Ok(out)
    }

    /// The order candidates are tried in: IRIs and literals by their N3 form, blank nodes by
    /// the arcs they carry (to `depth` levels), so that the order does not depend on the
    /// labels the parser gave them and two runs over the same document agree.
    fn sort_key(&self, value: &Object, depth: u8) -> Result<String, ShExMapError> {
        if !matches!(value, Object::BlankNode(_)) {
            return Ok(n3(value));
        }
        if depth == 0 {
            return Ok("_:".to_string());
        }
        if let Some(hit) = self.key_cache.borrow().get(&(value.clone(), depth)) {
            return Ok(hit.clone());
        }
        let mut arcs: Vec<String> = Vec::new();
        for t in self.outgoing(value)? {
            arcs.push(format!(
                "<{}> {}",
                t.predicate.as_str(),
                self.sort_key(&t.object, depth - 1)?
            ));
        }
        arcs.sort();
        let k = format!("_:[{}]", arcs.join("; "));
        self.key_cache.borrow_mut().insert((value.clone(), depth), k.clone());
        Ok(k)
    }

    // -- conformance ----------------------------------------------------------------------

    /// Whether `n` satisfies a shape expression: the partition search, without collecting.
    fn satisfies(&self, n: &Object, se: &'a ShapeExpr) -> Result<bool, ShExMapError> {
        match se {
            ShapeExpr::Ref(label) => {
                let decl = self.index.declaration(label).ok_or_else(|| {
                    ShExMapError::schema(format!(
                        "the schema references {}, which it does not define",
                        label_text(label)
                    ))
                })?;
                let k = (n.clone(), key(decl));
                if let Some(&hit) = self.sat_cache.borrow().get(&k) {
                    return Ok(hit);
                }
                if self.checking.borrow().contains(&k) {
                    return Ok(true); // a cycle through references: assumed, as validation does
                }
                self.checking.borrow_mut().insert(k.clone());
                let mut result = false;
                for candidate in self.index.extension_candidates(decl) {
                    if self.satisfies(n, &candidate.shape_expr)? {
                        result = true;
                        break;
                    }
                }
                self.checking.borrow_mut().remove(&k);
                self.sat_cache.borrow_mut().insert(k, result);
                Ok(result)
            },
            ShapeExpr::Shape(shape) => {
                let k = (n.clone(), key(se));
                if let Some(&hit) = self.sat_cache.borrow().get(&k) {
                    return Ok(hit);
                }
                if self.checking.borrow().contains(&k) {
                    return Ok(true);
                }
                self.checking.borrow_mut().insert(k.clone());
                let result = self.partitions(n, shape, Want::Exists);
                self.checking.borrow_mut().remove(&k);
                let result = !result?.is_empty();
                self.sat_cache.borrow_mut().insert(k, result);
                Ok(result)
            },
            ShapeExpr::ShapeAnd { shape_exprs } => {
                for w in shape_exprs {
                    if !self.satisfies(n, &w.se)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    if self.satisfies(n, &w.se)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            },
            ShapeExpr::ShapeNot { shape_expr } => Ok(!self.satisfies(n, &shape_expr.se)?),
            ShapeExpr::NodeConstraint(nc) => self.index.satisfies_nc(nc, n),
            ShapeExpr::External => Ok(true),
        }
    }

    // -- shape expressions: each returns the alternative records for node n -----------------

    fn shape_expr(&self, n: &Object, se: &'a ShapeExpr, label: Option<&str>) -> Result<Vec<Record>, ShExMapError> {
        if let ShapeExpr::Ref(lbl) = se {
            // a reference: bind through the most specific match
            let Some(decl) = self.index.declaration(lbl) else {
                return Ok(vec![Record::default()]);
            };
            let text = label_text(lbl);
            for candidate in self.index.extension_candidates(decl) {
                if std::ptr::eq(candidate, decl) || self.satisfies(n, &candidate.shape_expr)? {
                    return self.guarded(n, &candidate.shape_expr, Some(&text));
                }
            }
            return Ok(vec![Record::default()]);
        }
        self.guarded(n, se, label)
    }

    fn guarded(&self, n: &Object, se: &'a ShapeExpr, label: Option<&str>) -> Result<Vec<Record>, ShExMapError> {
        let k = (n.clone(), key(se));
        if self.active.borrow().contains(&k) {
            return Ok(vec![Record::default()]); // recursion: the outer match already collects these
        }
        self.active.borrow_mut().insert(k.clone());
        let result = self.shape_expr_inner(n, se, label);
        self.active.borrow_mut().remove(&k);
        result
    }

    fn shape_expr_inner(
        &self,
        n: &Object,
        se: &'a ShapeExpr,
        label: Option<&str>,
    ) -> Result<Vec<Record>, ShExMapError> {
        match se {
            ShapeExpr::Shape(shape) => {
                let records = self.partitions(n, shape, Want::All)?;
                if records.is_empty() {
                    // either the search gave up (MAX_PARTITIONS) or the node does not conform:
                    // say so, rather than binding nothing here and handing back a hollow tree
                    return Err(ShExMapError::Validation {
                        node: n3(n),
                        details: Some(format!(
                            "no partition of the neighbourhood of {} matches shape {}",
                            n3(n),
                            label.unwrap_or("(inline)")
                        )),
                    });
                }
                Ok(records)
            },
            ShapeExpr::ShapeAnd { shape_exprs } => {
                let mut alternatives = Vec::new();
                for w in shape_exprs {
                    alternatives.push(self.shape_expr(n, &w.se, label)?);
                }
                Ok(dedupe(product(&alternatives, self.limit), self.limit))
            },
            ShapeExpr::ShapeOr { shape_exprs } => {
                for w in shape_exprs {
                    if self.satisfies(n, &w.se)? {
                        return self.shape_expr(n, &w.se, label);
                    }
                }
                Ok(vec![Record::default()])
            },
            // NodeConstraint, ShapeNot, ShapeExternal bind nothing; a Ref is handled above
            _ => Ok(vec![Record::default()]),
        }
    }

    // -- shapes: partition the neighbourhood, then bind each partition ------------------------

    fn partitions(&self, n: &Object, shape: &'a Shape, want: Want) -> Result<Vec<Record>, ShExMapError> {
        let parts = self.index.shape_parts(shape);
        let tcs: Vec<Tc<'a>> = parts.iter().flat_map(|p| self.index.own_tcs(p)).collect();
        let keys: HashSet<(IriS, bool)> = tcs.iter().map(|tc| (tc.predicate_iri(), tc.inverse)).collect();
        let extras: HashSet<IriS> = parts
            .iter()
            .flat_map(|p| p.extra.iter().flatten().map(iri_of))
            .collect();
        let closed = parts.iter().any(|p| p.is_closed());

        let outgoing = self.outgoing(n)?;
        if closed
            && outgoing
                .iter()
                .any(|t| !keys.contains(&(t.predicate.clone(), false)) && !extras.contains(&t.predicate))
        {
            return Ok(Vec::new());
        }
        let mut matchables: Vec<Dt> = outgoing
            .into_iter()
            .filter(|t| keys.contains(&(t.predicate.clone(), false)))
            .map(|triple| Dt {
                dir: Direction::Out,
                triple,
            })
            .collect();
        if keys.iter().any(|(_, inverse)| *inverse) {
            matchables.extend(
                self.incoming(n)?
                    .into_iter()
                    .filter(|t| keys.contains(&(t.predicate.clone(), true)))
                    .map(|triple| Dt {
                        dir: Direction::In,
                        triple,
                    }),
            );
        }
        let mut keyed: Vec<(Direction, IriS, String, Dt)> = Vec::new();
        for dt in matchables {
            let value = if dt.dir == Direction::In {
                &dt.triple.subject
            } else {
                &dt.triple.object
            };
            let k = self.sort_key(value, 2)?;
            keyed.push((dt.dir, dt.triple.predicate.clone(), k, dt));
        }
        keyed.sort_by(|a, b| (a.0, &a.1, &a.2).cmp(&(b.0, &b.1, &b.2)));
        let mut matchables: Vec<Dt> = keyed.into_iter().map(|(_, _, _, dt)| dt).collect();
        matchables.dedup_by(|a, b| a.dir == b.dir && a.triple == b.triple);

        let exprs: Vec<&'a TripleExpr> = parts
            .iter()
            .filter_map(|p| p.expression.as_ref().map(|w| &w.te))
            .collect();
        let repeated = self.index.repeated_expressions(&exprs);
        let all: Avail = (0..matchables.len()).collect();
        let mut results: Vec<Record> = Vec::new();
        let mut tried = 0usize;
        let limit = self.limit;
        let m = &matchables;
        self.parts(n, &exprs, &all, m, &mut |items, rest| {
            tried += 1;
            if tried > MAX_PARTITIONS {
                return Ok(false);
            }
            if !self.valid_remainder(rest, m, &extras, &tcs)? {
                return Ok(true);
            }
            match want {
                Want::Exists => {
                    results.push(Record::default());
                    Ok(false)
                },
                Want::All => {
                    let found = self.bind_items(items, &repeated, m)?;
                    results.extend(found);
                    let kept = dedupe(std::mem::take(&mut results), limit);
                    results = kept;
                    Ok(results.len() < limit)
                },
            }
        })?;
        Ok(results)
    }

    fn valid_remainder(
        &self,
        rest: &Avail,
        m: &[Dt],
        extras: &HashSet<IriS>,
        tcs: &[Tc<'a>],
    ) -> Result<bool, ShExMapError> {
        for &i in rest {
            let dt = &m[i];
            if dt.dir != Direction::Out || !extras.contains(&dt.triple.predicate) {
                return Ok(false);
            }
            // EXTRA may only absorb a triple that no constraint on its predicate accepts
            for tc in tcs {
                if !tc.inverse && tc.predicate_iri() == dt.triple.predicate {
                    let accepts = match tc.value_expr {
                        None => true,
                        Some(ve) => self.satisfies(&dt.triple.object, ve)?,
                    };
                    if accepts {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

    /// Partitions of `avail` among a sequence of triple expressions, each handed to `visit`
    /// with what remains; `visit` returns whether to go on.
    fn parts(
        &self,
        n: &Object,
        exprs: &[&'a TripleExpr],
        avail: &Avail,
        m: &[Dt],
        visit: Visit<'_, 'a>,
    ) -> Result<bool, ShExMapError> {
        if exprs.is_empty() {
            return visit(&[], avail);
        }
        let (first, rest_exprs) = (exprs[0], &exprs[1..]);
        self.match_expr(n, first, avail, m, &mut |items, rest| {
            self.parts(n, rest_exprs, rest, m, &mut |more, rest2| {
                let mut all = items.to_vec();
                all.extend_from_slice(more);
                visit(&all, rest2)
            })
        })
    }

    /// Ways `expr` can match some of `avail`, greediest first.
    fn match_expr(
        &self,
        n: &Object,
        expr: &'a TripleExpr,
        avail: &Avail,
        m: &[Dt],
        visit: Visit<'_, 'a>,
    ) -> Result<bool, ShExMapError> {
        let Some(expr) = self.index.resolve(expr) else {
            return Err(ShExMapError::schema(
                "a triple expression inclusion names no expression",
            ));
        };
        if let Some(tc) = Tc::of(expr) {
            let (min, max) = cardinality(expr);
            let dir = if tc.inverse { Direction::In } else { Direction::Out };
            let pred = tc.predicate_iri();
            let mut candidates: Vec<usize> = Vec::new();
            for &i in avail {
                let dt = &m[i];
                if dt.dir != dir || dt.triple.predicate != pred {
                    continue;
                }
                let ok = match tc.value_expr {
                    None => true,
                    Some(ve) => self.satisfies(value_of(&tc, &dt.triple), ve)?,
                };
                if ok {
                    candidates.push(i);
                }
            }
            let upper = max.map_or(candidates.len(), |mx| mx.min(candidates.len()));
            if upper < min {
                return Ok(true);
            }
            for size in (min..=upper).rev() {
                for combo in Combinations::new(candidates.len(), size) {
                    let chosen: Vec<usize> = combo.iter().map(|&c| candidates[c]).collect();
                    let items: Vec<Item<'a>> = chosen.iter().map(|&at| Item::Tc { tc, at }).collect();
                    let rest: Avail = avail.iter().copied().filter(|i| !chosen.contains(i)).collect();
                    if !visit(&items, &rest)? {
                        return Ok(false);
                    }
                }
            }
            return Ok(true);
        }
        let (min, max) = cardinality(expr);
        self.repeat(n, expr, avail, m, 0, min, max, is_repeated(expr), visit)
    }

    #[allow(clippy::too_many_arguments)]
    fn repeat(
        &self,
        n: &Object,
        expr: &'a TripleExpr,
        avail: &Avail,
        m: &[Dt],
        count: usize,
        min: usize,
        max: Option<usize>,
        repeated: bool,
        visit: Visit<'_, 'a>,
    ) -> Result<bool, ShExMapError> {
        if max.is_none_or(|mx| count < mx) {
            let go_on = self.once(n, expr, avail, m, &mut |one, rest| {
                if rest == avail && count >= min {
                    return Ok(true); // an iteration that matches nothing adds nothing
                }
                let wrapped: Vec<Item<'a>> = if repeated {
                    vec![Item::Group {
                        expr,
                        items: one.to_vec(),
                    }]
                } else {
                    one.to_vec()
                };
                if rest == avail {
                    return visit(&wrapped, rest); // matched nothing but is required: count it once
                }
                self.repeat(n, expr, rest, m, count + 1, min, max, repeated, &mut |more, rest2| {
                    let mut all = wrapped.clone();
                    all.extend_from_slice(more);
                    visit(&all, rest2)
                })
            })?;
            if !go_on {
                return Ok(false);
            }
        }
        if count >= min {
            return visit(&[], avail);
        }
        Ok(true)
    }

    fn once(
        &self,
        n: &Object,
        expr: &'a TripleExpr,
        avail: &Avail,
        m: &[Dt],
        visit: Visit<'_, 'a>,
    ) -> Result<bool, ShExMapError> {
        match expr {
            TripleExpr::EachOf { .. } => {
                let subs = sub_expressions(expr);
                self.parts(n, &subs, avail, m, visit)
            },
            TripleExpr::OneOf { .. } => {
                for sub in sub_expressions(expr) {
                    if !self.match_expr(n, sub, avail, m, visit)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            _ => self.match_expr(n, expr, avail, m, visit),
        }
    }

    // -- binding a partition ----------------------------------------------------------------

    /// Alternative records for one partition's items: each of the scope's `repeated`
    /// expressions makes one list, with an iteration per item (empty when it matched
    /// nothing); a non-repeated constraint binds into the scope itself.
    fn bind_items(
        &self,
        items: &[Item<'a>],
        repeated: &[&'a TripleExpr],
        m: &[Dt],
    ) -> Result<Vec<Record>, ShExMapError> {
        let mut by_expr: HashMap<usize, Vec<&Item<'a>>> = HashMap::new();
        let mut firsts: Vec<&'a TripleExpr> = Vec::new();
        let mut alternatives: Vec<Vec<Record>> = Vec::new();
        for item in items {
            let (expr, grouped) = match item {
                Item::Group { expr, .. } => (*expr, true),
                Item::Tc { tc, .. } => (tc.expr, is_repeated(tc.expr)),
            };
            if grouped {
                let k = key(expr);
                if !by_expr.contains_key(&k) {
                    firsts.push(expr);
                }
                by_expr.entry(k).or_default().push(item);
                continue;
            }
            let Item::Tc { tc, at } = item else { unreachable!() };
            let t = &m[*at].triple;
            let value = value_of(tc, t);
            let lifted = self.lift(tc, value)?;
            if let (true, Some(ve)) = (tc.references_shape(), tc.value_expr) {
                // a nested scope: merge it into this one
                let mut opts = Vec::new();
                for sub in self.shape_expr(value, ve, None)? {
                    if lifted.keys().any(|k| sub.vars.contains_key(k)) {
                        // a name clash: keep it apart
                        opts.push(Record {
                            vars: lifted.clone(),
                            lists: vec![vec![sub]],
                            triples: BTreeSet::from([t.clone()]),
                        });
                    } else {
                        let mut vars = lifted.clone();
                        vars.extend(sub.vars);
                        let mut triples = sub.triples;
                        triples.insert(t.clone());
                        opts.push(Record {
                            vars,
                            lists: sub.lists,
                            triples,
                        });
                    }
                }
                alternatives.push(opts);
            } else {
                alternatives.push(vec![Record {
                    vars: lifted,
                    lists: Vec::new(),
                    triples: BTreeSet::from([t.clone()]),
                }]);
            }
        }
        let mut order: Vec<&'a TripleExpr> = repeated.to_vec();
        for e in firsts {
            if !order.iter().any(|o| key(*o) == key(e)) {
                order.push(e);
            }
        }
        for expr in order {
            let its = by_expr.remove(&key(expr)).unwrap_or_default();
            let mut per_iteration: Vec<Vec<Record>> = Vec::new();
            for it in its {
                per_iteration.push(self.iteration(it, m)?);
            }
            let mut opts: Vec<Record> = Vec::new();
            let mut counters = vec![0usize; per_iteration.len()];
            loop {
                let mut kept = Vec::new();
                let mut dropped = BTreeSet::new();
                for (i, recs) in per_iteration.iter().enumerate() {
                    let r = &recs[counters[i]];
                    if r.is_empty() {
                        dropped.extend(r.triples.iter().cloned()); // an iteration that bound nothing still matched
                    } else {
                        kept.push(r.clone());
                    }
                }
                opts.push(Record {
                    vars: BTreeMap::new(),
                    lists: vec![kept],
                    triples: dropped,
                });
                if opts.len() >= self.limit {
                    break;
                }
                // next combination, odometer style
                let mut i = per_iteration.len();
                loop {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                    counters[i] += 1;
                    if counters[i] < per_iteration[i].len() {
                        break;
                    }
                    counters[i] = 0;
                }
                if i == 0 && (per_iteration.is_empty() || counters.iter().all(|&c| c == 0)) {
                    break;
                }
            }
            alternatives.push(opts);
        }
        Ok(product(&alternatives, self.limit))
    }

    /// Alternative scopes for one iteration of a repeated constraint or group.
    fn iteration(&self, item: &Item<'a>, m: &[Dt]) -> Result<Vec<Record>, ShExMapError> {
        match item {
            Item::Group { expr, items } => {
                let subs = sub_expressions(expr);
                let repeated = self.index.repeated_expressions(&subs);
                let found = self.bind_items(items, &repeated, m)?;
                Ok(if found.is_empty() {
                    vec![Record::default()]
                } else {
                    found
                })
            },
            Item::Tc { tc, at } => {
                let t = &m[*at].triple;
                let value = value_of(tc, t);
                let lifted = self.lift(tc, value)?;
                if let (true, Some(ve)) = (tc.references_shape(), tc.value_expr) {
                    let mut out = Vec::new();
                    for sub in self.shape_expr(value, ve, None)? {
                        let mut vars = BTreeMap::from([(NODE_KEY.to_string(), value.clone())]);
                        vars.extend(lifted.iter().map(|(k, v)| (k.clone(), v.clone())));
                        vars.extend(sub.vars);
                        let mut triples = sub.triples;
                        triples.insert(t.clone());
                        out.push(Record {
                            vars,
                            lists: sub.lists,
                            triples,
                        });
                    }
                    Ok(out)
                } else {
                    Ok(vec![Record {
                        vars: lifted,
                        lists: Vec::new(),
                        triples: BTreeSet::from([t.clone()]),
                    }])
                }
            },
        }
    }

    fn lift(&self, tc: &Tc<'a>, value: &Object) -> Result<BTreeMap<String, Object>, ShExMapError> {
        let mut bound = BTreeMap::new();
        for act in tc.map_actions() {
            let code = code_of(act);
            if functions::is_function_call(&code) {
                bound.extend(functions::lift(&code, value, &self.index.prefixes)?);
            } else {
                bound.insert(functions::expand_variable(&code, &self.index.prefixes)?, value.clone());
            }
        }
        Ok(bound)
    }
}

// -- entry points -------------------------------------------------------------------------

/// Every distinct binding tree the input allows for `focus` (up to `limit`), on a schema
/// indexed already.
pub fn bind_all_indexed<R: NeighsRDF>(
    index: &SchemaIndex,
    rdf: &R,
    focus: &Object,
    start: Option<&ShapeExprLabel>,
    limit: usize,
) -> Result<Vec<Bindings>, ShExMapError> {
    let start_expr = index.start_expr(start)?;
    let extractor = Extractor::new(index, rdf, limit);
    if !extractor.satisfies(focus, &start_expr)? {
        return Err(ShExMapError::Validation {
            node: n3(focus),
            details: None,
        });
    }
    let records = extractor.shape_expr(focus, &start_expr, start.map(label_text).as_deref())?;
    let kept = dedupe(records, limit);
    let count = kept.len();
    Ok(kept
        .into_iter()
        .map(|r| Bindings {
            tree: r.tree(),
            alternatives: count,
            matched: r.all_triples(),
        })
        .collect())
}

/// Check that `focus` conforms to `schema` and return every distinct binding tree the input
/// allows (up to `limit`).
///
/// * `rdf`: the input graph
/// * `schema`: the input schema (its prefixes name the ShExMap variables)
/// * `focus`: the node to start from
/// * `start`: the shape label to match against; the schema's start when `None`
///
/// Fails with [`ShExMapError::Validation`] when `focus` does not conform.
pub fn bind_all<R: NeighsRDF>(
    rdf: &R,
    schema: &Schema,
    focus: &Object,
    start: Option<&ShapeExprLabel>,
    limit: usize,
) -> Result<Vec<Bindings>, ShExMapError> {
    let index = SchemaIndex::new(schema);
    bind_all_indexed(&index, rdf, focus, start, limit)
}

/// Check that `focus` conforms to `schema` and collect its ShExMap bindings.
///
/// When the input conforms in several ways that bind differently, the first (in schema
/// order, larger matches first) is returned and `.alternatives` says how many there were;
/// with `strict` an [`ShExMapError::Ambiguous`] error is returned instead.
pub fn bind<R: NeighsRDF>(
    rdf: &R,
    schema: &Schema,
    focus: &Object,
    start: Option<&ShapeExprLabel>,
    strict: bool,
) -> Result<Bindings, ShExMapError> {
    let mut found = bind_all(rdf, schema, focus, start, MAX_ALTERNATIVES)?;
    if strict && found.len() > 1 {
        return Err(ShExMapError::Ambiguous {
            node: n3(focus),
            alternatives: found,
        });
    }
    Ok(found.remove(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn combinations_enumerate_lexicographically() {
        let all: Vec<Vec<usize>> = Combinations::new(4, 2).collect();
        assert_eq!(
            all,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3], vec![2, 3]]
        );
        assert_eq!(Combinations::new(3, 0).collect::<Vec<_>>(), vec![Vec::<usize>::new()]);
        assert_eq!(Combinations::new(2, 3).count(), 0);
        assert_eq!(Combinations::new(0, 0).count(), 1);
    }

    #[test]
    fn trees_round_trip_and_flatten() {
        let j = json!([
            {"http://v/name": {"value": "Sue"}},
            [
                {"@node": "tag:b0", "http://v/sys": {"value": "110"}},
                {"@node": "tag:b1", "http://v/sys": {"value": "111"}}
            ]
        ]);
        let tree = BindingTree::from_json(&j).unwrap();
        assert_eq!(tree.to_json(), j);
        let frames = tree.frames();
        assert_eq!(frames.len(), 2);
        assert!(
            frames
                .iter()
                .all(|f| f.contains_key("http://v/name") && !f.contains_key("@node"))
        );
        assert_eq!(tree.nodes().len(), 2);
        assert_eq!(tree.variables().len(), 2);
    }

    #[test]
    fn legacy_layouts_are_read() {
        let l1 = json!([{"http://v/a": "http://x"}, {"http://v/a": "http://y"}]);
        let t = BindingTree::from_json(&l1).unwrap();
        assert_eq!(t.lists.len(), 1);
        assert_eq!(t.lists[0].len(), 2);
        let mixed = json!([{"http://v/n": {"value": "x"}}, [[{"http://v/a": "http://x"}, [{"http://v/b": "http://y"}]], {"http://v/a": "http://z"}]]);
        let t = BindingTree::from_json(&mixed).unwrap();
        assert_eq!(t.lists[0][0].lists.len(), 1);
        assert!(t.lists[0][1].lists.is_empty());
        // written back uniformly: the lone object is wrapped
        assert_eq!(t.to_json()[1][1], json!([{"http://v/a": "http://z"}]));
    }

    #[test]
    fn a_scope_with_empty_lists_keeps_its_frame() {
        let t = BindingTree::from_json(&json!([{"http://v/n": {"value": "x"}}, []])).unwrap();
        assert_eq!(t.frames().len(), 1);
    }
}
