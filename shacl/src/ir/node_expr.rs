use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use crate::ast::{ASTNodeExpr, ASTSchema};
use crate::error::IRError;
use crate::ir::{convert_iri_ref, IRSchema};
use crate::ir::node_exprs::{Concat, Count, Distinct, Empty, Exists, Filter, FindFirst, FlatMap, IfExpression, InstancesOf, Intersection, Iri, Limit, List, Literal, MatchAll, Max, Min, NodesMatching, Offset, OrderBy, PathValues, Remove, Sum, Var};
use crate::validator::engine::Engine;

#[derive(Debug, Clone)]
pub enum IRNodeExpr {
    // Constants
    Iri(Iri),
    Literal(Literal),

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
}

impl IRNodeExpr {
    pub fn compile(expr: &ASTNodeExpr, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let result = match expr.clone() {
            ASTNodeExpr::Iri(iri) => Self::Iri(Iri::new(convert_iri_ref(iri)?)),
            ASTNodeExpr::Literal(lit) => Self::Literal(Literal::new(lit)),
            ASTNodeExpr::Empty => Self::Empty(Empty),
            ASTNodeExpr::Var(var) => Self::Var(Var::new(var)),
            ASTNodeExpr::List(list) => Self::List(List::new(list)),
            ASTNodeExpr::PathValues { path, focus_node } => {
                let fnode = if let Some(fnode) = focus_node {
                    Some(IRNodeExpr::compile(fnode.deref(), ast, ir)?)
                } else { None };
                Self::PathValues(PathValues::new(path).with_focus_node(fnode))
            },
            ASTNodeExpr::Exists(exists) => {
                let e = IRNodeExpr::compile(exists.deref(), ast, ir)?;
                Self::Exists(Exists::new(e))
            },
            ASTNodeExpr::IfExpression { if_condition, then, else_expression } => {
                let i = IRNodeExpr::compile(if_condition.deref(), ast, ir)?;
                let t = then
                    .as_deref()
                    .map(|t| IRNodeExpr::compile(t, ast, ir))
                    .transpose()?;
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
            }
            ASTNodeExpr::Concat(concat) => {
                let v = concat
                    .iter()
                    .map(|e| IRNodeExpr::compile(e, ast, ir))
                    .collect::<Result<Vec<_>, _>>()?;
                Self::Concat(Concat::new(v))
            }
            ASTNodeExpr::Remove { nodes, remove } => {
                let r = IRNodeExpr::compile(remove.deref(), ast, ir)?;
                let n = IRNodeExpr::compile(nodes.deref(), ast, ir)?;
                Self::Remove(Remove::new(r, n))
            },
            ASTNodeExpr::Filter { nodes, filter_shape } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let f = ir.register_shape(filter_shape.id(), Some(&filter_shape), ast)?;
                Self::Filter(Filter::new(f, n))
            }
            ASTNodeExpr::Limit { nodes, limit } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                Self::Limit(Limit::new(limit, n))
            }
            ASTNodeExpr::Offset { nodes, offset } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                Self::Offset(Offset::new(offset, n))
            }
            ASTNodeExpr::OrderBy { nodes, order_by, desc } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let o = IRNodeExpr::compile(order_by.as_ref(), ast, ir)?;
                Self::OrderBy(OrderBy::new(o, n)
                    .with_desc(desc))
            }
            ASTNodeExpr::FlatMap { nodes, flat_map } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let f = IRNodeExpr::compile(flat_map.as_ref(), ast, ir)?;
                Self::FlatMap(FlatMap::new(f).with_nodes(Some(n)))
            }
            ASTNodeExpr::FindFirst { nodes, find_first } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let f = ir.register_shape(find_first.id(), Some(&find_first), ast)?;
                Self::FindFirst(FindFirst::new(f, n))
            }
            ASTNodeExpr::MatchAll { nodes, match_all } => {
                let n = IRNodeExpr::compile(nodes.as_ref(), ast, ir)?;
                let m = ir.register_shape(match_all.id(), Some(&match_all), ast)?;
                Self::MatchAll(MatchAll::new(m).with_nodes(Some(n)))
            }
            ASTNodeExpr::Count(count) => {
                let c = IRNodeExpr::compile(count.as_ref(), ast, ir)?;
                Self::Count(Count::new(c))
            }
            ASTNodeExpr::Min(min) => {
                let m = IRNodeExpr::compile(min.as_ref(), ast, ir)?;
                Self::Min(Min::new(m))
            }
            ASTNodeExpr::Max(max) => {
                let m = IRNodeExpr::compile(max.as_ref(), ast, ir)?;
                Self::Max(Max::new(m))
            }
            ASTNodeExpr::Sum(sum) => {
                let s = IRNodeExpr::compile(sum.as_ref(), ast, ir)?;
                Self::Sum(Sum::new(s))
            }
            ASTNodeExpr::InstancesOf(instances) => Self::InstancesOf(InstancesOf::new(convert_iri_ref(instances)?)),
            ASTNodeExpr::NodesMatching(node) => {
                let id = match node {
                    IriOrBlankNode::Iri(iri) => Object::iri(iri),
                    IriOrBlankNode::BlankNode(label) => Object::bnode(label),
                };
                let idx = ir.register_shape(&id, None, ast)?;
                Self::NodesMatching(NodesMatching::new(idx))
            }
        };

        Ok(result)
    }

    // TODO - Maybe this can be removed when implemented NodeExpr serialization
    pub fn as_constant(&self) -> Option<Object> {
        match self {
            IRNodeExpr::Iri(i) => Some(Object::Iri(i.iri().clone())),
            IRNodeExpr::Literal(l) => Some(Object::Literal(l.literal().clone())),
            _ => None,
        }
    }

    pub fn evaluate<Rdf: NeighsRDF + Debug>(&self, store: &Rdf, focus: &Rdf::Term, scope: &HashMap<String, Rdf::Term>, runner: &mut dyn Engine<Rdf>, ir: &IRSchema) -> Result<Vec<Rdf::Term>, IRError> {
        match self {
            IRNodeExpr::Iri(i) => i.evaluate(store, focus, scope, runner, ir),
            IRNodeExpr::Literal(l) => l.evaluate(store, focus, scope, runner, ir),
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
        }
    }
}

impl Display for IRNodeExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IRNodeExpr::Iri(i) => write!(f, "{i}"),
            IRNodeExpr::Literal(l) => write!(f, "{l}"),
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
        }
    }
}
