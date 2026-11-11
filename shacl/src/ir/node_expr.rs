use crate::ast::{ASTNodeExpr, ASTSchema};
use crate::error::{IRError, NodeExprEvalError};
use crate::ir::node_exprs::{
    Concat, ConformsToShape, Count, Distinct, Empty, Exists, Filter, FindFirst, FlatMap, IfExpression, InstancesOf,
    Intersection, Iri, Limit, List, Literal, MatchAll, Max, Min, NodesMatching, Offset, OrderBy, PathValues, Remove,
    Sum, TripleTerm, Var,
};
use crate::ir::node_expr_register::{NodeExprRegister, Shapes};
use crate::ir::{IRSchema, convert_iri_ref};
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{BuildRDF, NeighsRDF};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// Expressions evaluated for one focus node.
const MAX_EVAL_STEPS: usize = 1_000_000;

/// The node a shape-taking node expression refers its shape by, as a shape id.
fn shape_id(node: IriOrBlankNode) -> Object {
    match node {
        IriOrBlankNode::Iri(iri) => Object::iri(iri),
        IriOrBlankNode::BlankNode(label) => Object::bnode(label),
    }
}

#[derive(Debug, Clone)]
pub enum IRNodeExpr {
    // Constants
    Iri(Iri),
    Literal(Literal),
    TripleTerm(TripleTerm),

    // Basic Expressions
    Empty(Empty),
    Var(Var),
    List(List),
    PathValues(PathValues),
    Exists(Exists),
    IfExpression(IfExpression),

    // List Expressions
    Distinct(Distinct),
    Intersection(Intersection),
    Concat(Concat),
    Remove(Remove),
    Filter(Filter),
    Limit(Limit),
    Offset(Offset),
    OrderBy(OrderBy),

    // Advanced Sequence Operations
    FlatMap(FlatMap),
    FindFirst(FindFirst),
    MatchAll(MatchAll),

    // Aggregation expressions
    Count(Count),
    Min(Min),
    Max(Max),
    Sum(Sum),

    // Miscellaneous Expressions
    InstancesOf(InstancesOf),
    NodesMatching(NodesMatching),
    ConformsToShape(ConformsToShape),
}

impl IRNodeExpr {
    pub fn compile(expr: &ASTNodeExpr, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        // A debug build takes about 19 KiB of stack per level, so 64 levels would not fit in the red zone
        crate::with_stack(|| Self::compile_level(expr, ast, ir))
    }

    fn compile_level(expr: &ASTNodeExpr, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let result = match expr.clone() {
            ASTNodeExpr::Iri(iri) => Self::Iri(Iri::new(convert_iri_ref(iri)?)),
            ASTNodeExpr::Literal(lit) => Self::Literal(Literal::new(lit)),
            ASTNodeExpr::TripleTerm(triple) => Self::TripleTerm(TripleTerm::new(triple)),
            ASTNodeExpr::Empty => Self::Empty(Empty),
            ASTNodeExpr::Var(var) => Self::Var(Var::new(var)),
            ASTNodeExpr::List(list) => Self::List(List::new(list)),
            ASTNodeExpr::PathValues { path, focus_node } => {
                let fnode = if let Some(fnode) = focus_node {
                    Some(IRNodeExpr::compile(fnode.deref(), ast, ir)?)
                } else {
                    None
                };
                Self::PathValues(PathValues::new(path).with_focus_node(fnode))
            },
            ASTNodeExpr::Exists(exists) => {
                let e = IRNodeExpr::compile(exists.deref(), ast, ir)?;
                Self::Exists(Exists::new(e))
            },
            ASTNodeExpr::IfExpression {
                if_condition,
                then,
                else_expression,
            } => {
                let i = IRNodeExpr::compile(if_condition.deref(), ast, ir)?;
                let t = then.as_deref().map(|t| IRNodeExpr::compile(t, ast, ir)).transpose()?;
                let e = else_expression
                    .as_deref()
                    .map(|e| IRNodeExpr::compile(e, ast, ir))
                    .transpose()?;
                Self::IfExpression(IfExpression::new(i).with_then(t).with_else(e))
            },
            ASTNodeExpr::Distinct(distinct) => {
                let d = IRNodeExpr::compile(distinct.as_ref(), ast, ir)?;
                Self::Distinct(Distinct::new(d))
            },
            ASTNodeExpr::Intersection(intersection) => {
                let v = intersection
                    .iter()
                    .map(|e| IRNodeExpr::compile(e, ast, ir))
                    .collect::<Result<Vec<_>, _>>()?;
                Self::Intersection(Intersection::new(v))
            },
            ASTNodeExpr::Concat(concat) => {
                let v = concat
                    .iter()
                    .map(|e| IRNodeExpr::compile(e, ast, ir))
                    .collect::<Result<Vec<_>, _>>()?;
                Self::Concat(Concat::new(v))
            },
            ASTNodeExpr::Remove { nodes, remove } => {
                let r = IRNodeExpr::compile(remove.deref(), ast, ir)?;
                let n = IRNodeExpr::compile(nodes.deref(), ast, ir)?;
                Self::Remove(Remove::new(r, n))
            },
            ASTNodeExpr::Filter { nodes, filter_shape } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let f = ir.register_shape(&shape_id(filter_shape), None, ast)?;
                Self::Filter(Filter::new(f, n))
            },
            ASTNodeExpr::Limit { nodes, limit } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                Self::Limit(Limit::new(limit, n))
            },
            ASTNodeExpr::Offset { nodes, offset } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                Self::Offset(Offset::new(offset, n))
            },
            ASTNodeExpr::OrderBy { nodes, order_by, desc } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let o = IRNodeExpr::compile(order_by.as_ref(), ast, ir)?;
                Self::OrderBy(OrderBy::new(o, n).with_desc(desc))
            },
            ASTNodeExpr::FlatMap { nodes, flat_map } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let f = IRNodeExpr::compile(flat_map.as_ref(), ast, ir)?;
                Self::FlatMap(FlatMap::new(f).with_nodes(Some(n)))
            },
            ASTNodeExpr::FindFirst { nodes, find_first } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let f = ir.register_shape(&shape_id(find_first), None, ast)?;
                Self::FindFirst(FindFirst::new(f, n))
            },
            ASTNodeExpr::MatchAll { nodes, match_all } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let m = ir.register_shape(&shape_id(match_all), None, ast)?;
                Self::MatchAll(MatchAll::new(m).with_nodes(Some(n)))
            },
            ASTNodeExpr::Count(count) => {
                let c = IRNodeExpr::compile(count.as_ref(), ast, ir)?;
                Self::Count(Count::new(c))
            },
            ASTNodeExpr::Min(min) => {
                let m = IRNodeExpr::compile(min.as_ref(), ast, ir)?;
                Self::Min(Min::new(m))
            },
            ASTNodeExpr::Max(max) => {
                let m = IRNodeExpr::compile(max.as_ref(), ast, ir)?;
                Self::Max(Max::new(m))
            },
            ASTNodeExpr::Sum(sum) => {
                let s = IRNodeExpr::compile(sum.as_ref(), ast, ir)?;
                Self::Sum(Sum::new(s))
            },
            ASTNodeExpr::InstancesOf(instances) => Self::InstancesOf(InstancesOf::new(convert_iri_ref(instances)?)),
            ASTNodeExpr::NodesMatching(node) => {
                let idx = ir.register_shape(&shape_id(node), None, ast)?;
                Self::NodesMatching(NodesMatching::new(idx))
            },
            ASTNodeExpr::ConformsToShape { node, shape } => {
                let n = IRNodeExpr::compile(node.as_ref(), ast, ir)?;
                // The shape is only known once the expression is evaluated, so it is resolved there
                let s = IRNodeExpr::compile(shape.as_ref(), ast, ir)?;
                Self::ConformsToShape(ConformsToShape::new(n, s))
            },
        };

        Ok(result)
    }

    /// Writes the expression to `graph`, and returns the term that denotes it: the constant
    /// itself for a constant expression, the head of the list for a list one, and a fresh
    /// blank node for every other one.
    pub fn register<RDF: BuildRDF>(&self, graph: &mut RDF, shapes: &Shapes) -> Result<RDF::Term, IRError> {
        crate::with_stack(|| self.register_level(graph, shapes))
    }

    fn register_level<RDF: BuildRDF>(&self, graph: &mut RDF, shapes: &Shapes) -> Result<RDF::Term, IRError> {
        // A constant denotes itself
        if let Some(constant) = self.as_constant() {
            return Ok(RDF::object_as_term(&constant));
        }

        // A list node expression is the RDF list itself. An empty one is `rdf:nil`, which is
        // read back as the IRI expression: the syntax has no other term for it.
        if let IRNodeExpr::List(l) = self {
            let items = l.list().iter().map(RDF::object_as_term).collect();
            return graph
                .add_rdf_list(items)
                .map_err(|e| IRError::from_rdf_err::<RDF>("add a node expression list", e));
        }

        let mut register = NodeExprRegister::new(graph, shapes)?;
        let term = register.term();

        match self {
            // Written above, before the blank node is taken
            IRNodeExpr::Iri(_) | IRNodeExpr::Literal(_) | IRNodeExpr::TripleTerm(_) | IRNodeExpr::List(_) => {
                unreachable!("a constant or list expression denotes itself")
            },
            // A blank node with no arcs at all
            IRNodeExpr::Empty(_) => Ok(()),
            IRNodeExpr::Var(v) => register.string(ShaclNodeExprVocab::shnex_var(), v.var()),
            IRNodeExpr::PathValues(pv) => {
                register.path(ShaclNodeExprVocab::shnex_path_values(), pv.path())?;
                register.optional_expr(ShaclNodeExprVocab::shnex_focus_node(), pv.focus_nodes())
            },
            IRNodeExpr::Exists(e) => register.expr(ShaclNodeExprVocab::shnex_exists(), e.exists()),
            IRNodeExpr::IfExpression(ie) => {
                register.expr(ShaclNodeExprVocab::shnex_if(), ie.if_condition())?;
                register.optional_expr(ShaclNodeExprVocab::shnex_then(), ie.then())?;
                register.optional_expr(ShaclNodeExprVocab::shnex_else(), ie.else_expression())
            },
            IRNodeExpr::Distinct(d) => register.expr(ShaclNodeExprVocab::shnex_distinct(), d.distinct()),
            IRNodeExpr::Intersection(i) => {
                register.expr_list(ShaclNodeExprVocab::shnex_intersection(), i.intersection())
            },
            IRNodeExpr::Concat(c) => register.expr_list(ShaclNodeExprVocab::shnex_concat(), c.concat()),
            IRNodeExpr::Remove(r) => {
                register.expr(ShaclNodeExprVocab::shnex_remove(), r.remove())?;
                register.nodes(r.nodes())
            },
            IRNodeExpr::Filter(f) => {
                register.shape(ShaclNodeExprVocab::shnex_filter_shape(), f.filter_shape())?;
                register.nodes(f.nodes())
            },
            IRNodeExpr::Limit(l) => {
                register.integer(ShaclNodeExprVocab::shnex_limit(), l.limit())?;
                register.nodes(l.nodes())
            },
            IRNodeExpr::Offset(o) => {
                register.integer(ShaclNodeExprVocab::shnex_offset(), o.offset())?;
                register.nodes(o.nodes())
            },
            IRNodeExpr::OrderBy(ob) => {
                register.expr(ShaclNodeExprVocab::shnex_order_by(), ob.order_by())?;
                if let Some(desc) = ob.desc() {
                    register.boolean(ShaclNodeExprVocab::shnex_desc(), desc)?;
                }
                register.nodes(ob.nodes())
            },
            IRNodeExpr::FlatMap(fm) => {
                register.expr(ShaclNodeExprVocab::shnex_flat_map(), fm.flat_map())?;
                register.optional_expr(ShaclNodeExprVocab::shnex_nodes(), fm.nodes().as_ref())
            },
            IRNodeExpr::FindFirst(ff) => {
                register.shape(ShaclNodeExprVocab::shnex_find_first(), ff.find_first())?;
                register.nodes(ff.nodes())
            },
            IRNodeExpr::MatchAll(ma) => {
                register.shape(ShaclNodeExprVocab::shnex_match_all(), ma.match_all())?;
                register.optional_expr(ShaclNodeExprVocab::shnex_nodes(), ma.nodes())
            },
            IRNodeExpr::Count(c) => register.expr(ShaclNodeExprVocab::shnex_count(), c.count()),
            IRNodeExpr::Min(m) => register.expr(ShaclNodeExprVocab::shnex_min(), m.min()),
            IRNodeExpr::Max(m) => register.expr(ShaclNodeExprVocab::shnex_max(), m.max()),
            IRNodeExpr::Sum(s) => register.expr(ShaclNodeExprVocab::shnex_sum(), s.sum()),
            IRNodeExpr::InstancesOf(io) => register.iri(ShaclNodeExprVocab::shnex_instances_of(), io.iri()),
            IRNodeExpr::NodesMatching(nm) => register.shape(ShaclNodeExprVocab::shnex_nodes_matching(), nm.shape()),
            IRNodeExpr::ConformsToShape(cts) => {
                register.expr_list(ShaclNodeExprVocab::shnex_conforms_to_shape(), [cts.node(), cts.shape()])
            },
        }?;

        Ok(term)
    }

    pub fn as_constant(&self) -> Option<Object> {
        match self {
            IRNodeExpr::Iri(i) => Some(Object::Iri(i.iri().clone())),
            IRNodeExpr::Literal(l) => Some(Object::Literal(l.literal().clone())),
            IRNodeExpr::TripleTerm(t) => Some(t.triple().clone()),
            _ => None,
        }
    }

    /// Whether the value can differ between focus nodes.
    pub(crate) fn reads_focus(&self) -> bool {
        match self {
            IRNodeExpr::Iri(_)
            | IRNodeExpr::Literal(_)
            | IRNodeExpr::TripleTerm(_)
            | IRNodeExpr::Empty(_)
            | IRNodeExpr::List(_)
            | IRNodeExpr::InstancesOf(_)
            | IRNodeExpr::NodesMatching(_) => false,
            // The focus node, or a variable of the scope
            IRNodeExpr::Var(_) => true,
            IRNodeExpr::PathValues(pv) => pv.focus_nodes().is_none_or(Self::reads_focus),
            IRNodeExpr::Exists(e) => e.exists().reads_focus(),
            IRNodeExpr::IfExpression(ie) => [Some(ie.if_condition()), ie.then(), ie.else_expression()]
                .into_iter()
                .flatten()
                .any(Self::reads_focus),
            IRNodeExpr::Distinct(d) => d.distinct().reads_focus(),
            IRNodeExpr::Intersection(i) => i.intersection().iter().any(Self::reads_focus),
            IRNodeExpr::Concat(c) => c.concat().iter().any(Self::reads_focus),
            IRNodeExpr::Remove(r) => r.nodes().reads_focus() || r.remove().reads_focus(),
            IRNodeExpr::Limit(l) => l.nodes().reads_focus(),
            IRNodeExpr::Offset(o) => o.nodes().reads_focus(),
            IRNodeExpr::Count(c) => c.count().reads_focus(),
            IRNodeExpr::Min(m) => m.min().reads_focus(),
            IRNodeExpr::Max(m) => m.max().reads_focus(),
            IRNodeExpr::Sum(s) => s.sum().reads_focus(),
            IRNodeExpr::ConformsToShape(cts) => cts.node().reads_focus() || cts.shape().reads_focus(),
            // These take each of their nodes as the focus node of their shape or inner expression
            IRNodeExpr::Filter(f) => f.nodes().reads_focus(),
            IRNodeExpr::OrderBy(ob) => ob.nodes().reads_focus(),
            IRNodeExpr::FlatMap(fm) => fm.nodes().as_ref().is_none_or(Self::reads_focus),
            IRNodeExpr::FindFirst(ff) => ff.nodes().reads_focus(),
            IRNodeExpr::MatchAll(ma) => ma.nodes().is_none_or(Self::reads_focus),
        }
    }

    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        store: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let out = match self {
            IRNodeExpr::Iri(i) => i.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Literal(l) => l.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::TripleTerm(t) => t.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Empty(e) => e.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Var(v) => v.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::List(l) => l.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::PathValues(pv) => pv.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Exists(e) => e.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::IfExpression(ie) => ie.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Distinct(d) => d.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Intersection(i) => i.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Concat(c) => c.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Remove(r) => r.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Filter(f) => f.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Limit(l) => l.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Offset(o) => o.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::OrderBy(ob) => ob.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::FlatMap(fm) => fm.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::FindFirst(ff) => ff.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::MatchAll(ma) => ma.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Count(c) => c.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Min(m) => m.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Max(m) => m.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Sum(s) => s.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::InstancesOf(io) => io.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::NodesMatching(nm) => nm.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::ConformsToShape(cts) => cts.evaluate(store, focus, scope, runner, ir),
        }?;

        // The data can fan an expression out exponentially, as nested shnex:flatMap over
        // nodes with two values each does, so the work for one focus node is capped
        let state = runner.node_expr_state();
        state.steps += 1;
        if state.steps > MAX_EVAL_STEPS {
            return Err(NodeExprEvalError::TooManySteps(MAX_EVAL_STEPS).into());
        }

        Ok(out)
    }
}

impl Display for IRNodeExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IRNodeExpr::Iri(i) => write!(f, "{i}"),
            IRNodeExpr::Literal(l) => write!(f, "{l}"),
            IRNodeExpr::TripleTerm(t) => write!(f, "{t}"),
            IRNodeExpr::Empty(e) => write!(f, "{e}"),
            IRNodeExpr::Var(v) => write!(f, "{v}"),
            IRNodeExpr::List(l) => write!(f, "{l}"),
            IRNodeExpr::PathValues(pv) => write!(f, "{pv}"),
            IRNodeExpr::Exists(e) => write!(f, "{e}"),
            IRNodeExpr::IfExpression(ie) => write!(f, "{ie}"),
            IRNodeExpr::Distinct(d) => write!(f, "{d}"),
            IRNodeExpr::Intersection(i) => write!(f, "{i}"),
            IRNodeExpr::Concat(c) => write!(f, "{c}"),
            IRNodeExpr::Remove(r) => write!(f, "{r}"),
            IRNodeExpr::Filter(fi) => write!(f, "{fi}"),
            IRNodeExpr::Limit(l) => write!(f, "{l}"),
            IRNodeExpr::Offset(o) => write!(f, "{o}"),
            IRNodeExpr::OrderBy(ob) => write!(f, "{ob}"),
            IRNodeExpr::FlatMap(fm) => write!(f, "{fm}"),
            IRNodeExpr::FindFirst(ff) => write!(f, "{ff}"),
            IRNodeExpr::MatchAll(ma) => write!(f, "{ma}"),
            IRNodeExpr::Count(c) => write!(f, "{c}"),
            IRNodeExpr::Min(m) => write!(f, "{m}"),
            IRNodeExpr::Max(m) => write!(f, "{m}"),
            IRNodeExpr::Sum(s) => write!(f, "{s}"),
            IRNodeExpr::InstancesOf(io) => write!(f, "{io}"),
            IRNodeExpr::NodesMatching(nm) => write!(f, "{nm}"),
            IRNodeExpr::ConformsToShape(cts) => write!(f, "{cts}"),
        }
    }
}
