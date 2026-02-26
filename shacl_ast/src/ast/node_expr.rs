use crate::shape::Shape;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use rudof_rdf::rdf_core::{Rdf, SHACLPath};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NodeExpr<RDF: Rdf> {
    // Constants
    Iri(IriRef),
    Literal(ConcreteLiteral),

    // Basic expressions
    Empty,
    Var(String),
    List(Vec<Object>),
    PathValues {
        path: SHACLPath,
        focus_node: Option<Box<NodeExpr<RDF>>>,
    },
    Exists(Box<NodeExpr<RDF>>),
    IfExpression {
        if_condition: Box<NodeExpr<RDF>>,
        then: Option<Box<NodeExpr<RDF>>>,
        else_expression: Option<Box<NodeExpr<RDF>>>,
    },

    // List expressions
    Distinct(Box<NodeExpr<RDF>>),
    Intersection(Vec<NodeExpr<RDF>>),
    Concat(Vec<NodeExpr<RDF>>),
    Remove {
        remove: Box<NodeExpr<RDF>>,
        nodes: Box<NodeExpr<RDF>>,
    },
    Filter {
        filter_shape: Shape<RDF>,
        nodes: Box<NodeExpr<RDF>>,
    },
    Limit {
        limit: usize,
        nodes: Box<NodeExpr<RDF>>,
    },
    Offset {
        offset: usize,
        nodes: Box<NodeExpr<RDF>>,
    },
    // OrderBy, // TODO - Not yet defined in the draft

    // Advance sequence operations
    FlatMap {
        flat_map: Box<NodeExpr<RDF>>,
        nodes: Box<NodeExpr<RDF>>,
    },
    FindFirst {
        find_first: Shape<RDF>,
        nodes: Box<NodeExpr<RDF>>,
    },
    MatchAll {
        match_all: Shape<RDF>,
        nodes: Box<NodeExpr<RDF>>,
    },

    // Aggregation expressions
    Count(Box<NodeExpr<RDF>>),
    Min(Box<NodeExpr<RDF>>),
    Max(Box<NodeExpr<RDF>>),
    Sum(Box<NodeExpr<RDF>>), // The draft mentions that maybe is removed

    // Miscellaneous expressions
    InstancesOf(IriRef),
    NodesMatching(IriOrBlankNode),
}

impl<RDF: Rdf> Display for NodeExpr<RDF> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeExpr::Iri(i) => write!(f, "iri({i})"),
            NodeExpr::Literal(l) => write!(f, "literal({l})"),
            NodeExpr::Empty => write!(f, "empty"),
            NodeExpr::Var(v) => write!(f, "var({v})"),
            NodeExpr::List(l) => write!(
                f,
                "list({})",
                l.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(", ")
            ),
            NodeExpr::PathValues { focus_node, path } => {
                let fnode = match focus_node {
                    None => "".to_string(),
                    Some(n) => n.to_string(),
                };
                write!(f, "pathValues({path}, {fnode})")
            },
            NodeExpr::Exists(e) => write!(f, "exists({e})"),
            NodeExpr::IfExpression {
                if_condition,
                then,
                else_expression,
            } => {
                write!(
                    f,
                    "if({if_condition}{}{})",
                    if let Some(then) = then {
                        format!(" then {then}")
                    } else {
                        "".to_string()
                    },
                    if let Some(else_) = else_expression {
                        format!(" else {else_}")
                    } else {
                        "".to_string()
                    }
                )
            },
            NodeExpr::Distinct(d) => write!(f, "distinct({d})"),
            NodeExpr::Intersection(i) => write!(
                f,
                "intersection({})",
                i.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
            ),
            NodeExpr::Concat(c) => write!(
                f,
                "concat({})",
                c.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
            ),
            NodeExpr::Remove { remove, nodes } => write!(f, "remove({remove} in {nodes})"),
            NodeExpr::Filter { filter_shape, nodes } => write!(f, "filter({filter_shape} in {nodes})"),
            NodeExpr::Limit { limit, nodes } => writeln!(f, "limit({limit} in {nodes})"),
            NodeExpr::Offset { offset, nodes } => writeln!(f, "offset({offset} in {nodes})"),
            NodeExpr::FlatMap { flat_map, nodes } => write!(f, "flatMap({flat_map} in {nodes}"),
            NodeExpr::FindFirst { find_first, nodes } => write!(f, "findFirst({find_first} in {nodes}"),
            NodeExpr::MatchAll { match_all, nodes } => write!(f, "matchAll({match_all} in {nodes}"),
            NodeExpr::Count(c) => write!(f, "count({c})"),
            NodeExpr::Min(m) => write!(f, "min({m})"),
            NodeExpr::Max(m) => write!(f, "max({m}"),
            NodeExpr::Sum(s) => write!(f, "sum({s})"),
            NodeExpr::InstancesOf(i) => write!(f, "instancesOf({i})"),
            NodeExpr::NodesMatching(n) => write!(f, "nodesMatching({n})"),
        }
    }
}

impl<RDF: Rdf> Clone for NodeExpr<RDF> {
    fn clone(&self) -> Self {
        match self {
            NodeExpr::Iri(i) => NodeExpr::Iri(i.clone()),
            NodeExpr::Literal(l) => NodeExpr::Literal(l.clone()),
            NodeExpr::Empty => NodeExpr::Empty,
            NodeExpr::Var(v) => NodeExpr::Var(v.clone()),
            NodeExpr::List(l) => NodeExpr::List(l.clone()),
            NodeExpr::PathValues { path, focus_node } => NodeExpr::PathValues {
                path: path.clone(),
                focus_node: focus_node.clone(),
            },
            NodeExpr::Exists(e) => NodeExpr::Exists(e.clone()),
            NodeExpr::IfExpression {
                if_condition,
                then,
                else_expression,
            } => NodeExpr::IfExpression {
                if_condition: if_condition.clone(),
                then: then.clone(),
                else_expression: else_expression.clone(),
            },
            NodeExpr::Distinct(d) => NodeExpr::Distinct(d.clone()),
            NodeExpr::Intersection(i) => NodeExpr::Intersection(i.clone()),
            NodeExpr::Concat(c) => NodeExpr::Concat(c.clone()),
            NodeExpr::Remove { remove, nodes } => NodeExpr::Remove {
                remove: remove.clone(),
                nodes: nodes.clone(),
            },
            NodeExpr::Filter { filter_shape, nodes } => NodeExpr::Filter {
                filter_shape: filter_shape.clone(),
                nodes: nodes.clone(),
            },
            NodeExpr::Limit { limit, nodes } => NodeExpr::Limit {
                limit: *limit,
                nodes: nodes.clone(),
            },
            NodeExpr::Offset { offset, nodes } => NodeExpr::Offset {
                offset: *offset,
                nodes: nodes.clone(),
            },
            NodeExpr::FlatMap { flat_map, nodes } => NodeExpr::FlatMap {
                flat_map: flat_map.clone(),
                nodes: nodes.clone(),
            },
            NodeExpr::FindFirst { find_first, nodes } => NodeExpr::FindFirst {
                find_first: find_first.clone(),
                nodes: nodes.clone(),
            },
            NodeExpr::MatchAll { match_all, nodes } => NodeExpr::MatchAll {
                match_all: match_all.clone(),
                nodes: nodes.clone(),
            },
            NodeExpr::Count(c) => NodeExpr::Count(c.clone()),
            NodeExpr::Min(m) => NodeExpr::Min(m.clone()),
            NodeExpr::Max(m) => NodeExpr::Max(m.clone()),
            NodeExpr::Sum(s) => NodeExpr::Sum(s.clone()),
            NodeExpr::InstancesOf(i) => NodeExpr::InstancesOf(i.clone()),
            NodeExpr::NodesMatching(n) => NodeExpr::NodesMatching(n.clone()),
        }
    }
}

impl<RDF: Rdf> PartialEq for NodeExpr<RDF> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeExpr::Iri(l), NodeExpr::Iri(r)) => l == r,
            (NodeExpr::Literal(l), NodeExpr::Literal(r)) => l == r,
            (NodeExpr::Empty, NodeExpr::Empty) => true,
            (NodeExpr::Var(l), NodeExpr::Var(r)) => l == r,
            (NodeExpr::List(l), NodeExpr::List(r)) => l == r,
            (
                NodeExpr::PathValues {
                    focus_node: fl,
                    path: pl,
                },
                NodeExpr::PathValues {
                    focus_node: fr,
                    path: pr,
                },
            ) => pl == pr && fl == fr,
            (NodeExpr::Exists(l), NodeExpr::Exists(r)) => l == r,
            (
                NodeExpr::IfExpression {
                    if_condition: il,
                    then: tl,
                    else_expression: el,
                },
                NodeExpr::IfExpression {
                    if_condition: ir,
                    then: tr,
                    else_expression: er,
                },
            ) => il == ir && tl == tr && el == er,
            (NodeExpr::Distinct(l), NodeExpr::Distinct(r)) => l == r,
            (NodeExpr::Intersection(l), NodeExpr::Intersection(r)) => l == r,
            (NodeExpr::Concat(l), NodeExpr::Concat(r)) => l == r,
            (NodeExpr::Remove { remove: rl, nodes: nl }, NodeExpr::Remove { remove: rr, nodes: nr }) => {
                rl == rr && nl == nr
            },
            (
                NodeExpr::Filter {
                    filter_shape: fl,
                    nodes: nl,
                },
                NodeExpr::Filter {
                    filter_shape: fr,
                    nodes: nr,
                },
            ) => fl == fr && nl == nr,
            (NodeExpr::Limit { limit: ll, nodes: nl }, NodeExpr::Limit { limit: lr, nodes: nr }) => {
                ll == lr && nl == nr
            },
            (NodeExpr::Offset { offset: ol, nodes: nl }, NodeExpr::Offset { offset: or, nodes: nr }) => {
                ol == or && nl == nr
            },
            (
                NodeExpr::FlatMap {
                    flat_map: fl,
                    nodes: nl,
                },
                NodeExpr::FlatMap {
                    flat_map: fr,
                    nodes: nr,
                },
            ) => fl == fr && nl == nr,
            (
                NodeExpr::FindFirst {
                    find_first: fl,
                    nodes: nl,
                },
                NodeExpr::FindFirst {
                    find_first: fr,
                    nodes: nr,
                },
            ) => fl == fr && nl == nr,
            (
                NodeExpr::MatchAll {
                    match_all: ml,
                    nodes: nl,
                },
                NodeExpr::MatchAll {
                    match_all: mr,
                    nodes: nr,
                },
            ) => ml == mr && nl == nr,
            (NodeExpr::Count(l), NodeExpr::Count(r)) => l == r,
            (NodeExpr::Min(l), NodeExpr::Min(r)) => l == r,
            (NodeExpr::Max(l), NodeExpr::Max(r)) => l == r,
            (NodeExpr::Sum(l), NodeExpr::Sum(r)) => l == r,
            (NodeExpr::InstancesOf(l), NodeExpr::InstancesOf(r)) => l == r,
            (NodeExpr::NodesMatching(l), NodeExpr::NodesMatching(r)) => l == r,
            _ => false,
        }
    }
}
