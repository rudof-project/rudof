use crate::ast::ASTShape;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use rudof_rdf::rdf_core::SHACLPath;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NodeExpr {
    // Constants
    Iri(IriRef),
    Literal(ConcreteLiteral),

    // Basic expressions
    Empty,
    Var(String),
    List(Vec<Object>),
    PathValues {
        path: SHACLPath,
        focus_node: Option<Box<NodeExpr>>
    },
    Exists(Box<NodeExpr>),
    IfExpression {
        if_condition: Box<NodeExpr>,
        then: Box<NodeExpr>,
        else_expression: Box<NodeExpr>,
    },

    // List expressions
    Distinct(Box<NodeExpr>),
    Intersection(Vec<NodeExpr>),
    Concat(Vec<NodeExpr>),
    Remove {
        remove: Box<NodeExpr>,
        nodes: Box<NodeExpr>,
    },
    Filter {
        filter_shape: ASTShape,
        nodes: Box<NodeExpr>,
    },
    Limit {
        limit: usize,
        nodes: Box<NodeExpr>,
    },
    Offset {
        offset: usize,
        nodes: Box<NodeExpr>,
    },
    // OrderBy, // TODO - Not yet defined in the draft

    // Advance sequence operations
    FlatMap {
        flat_map: Box<NodeExpr>,
        nodes: Box<NodeExpr>,
    },
    FindFirst {
        find_first: ASTShape,
        nodes: Box<NodeExpr>
    },
    MatchAll {
        match_all: ASTShape,
        nodes: Box<NodeExpr>,
    },

    // Aggregation expressions
    Count(Box<NodeExpr>),
    Min(Box<NodeExpr>),
    Max(Box<NodeExpr>),
    Sum(Box<NodeExpr>), // The draft mentions that maybe is removed

    // Miscellaneous expressions
    InstancesOf(IriRef),
    NodesMatching(IriOrBlankNode),
}

impl Display for NodeExpr {
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
            }
            NodeExpr::Exists(e) => write!(f, "exists({e})"),
            NodeExpr::IfExpression { if_condition, then, else_expression } => {
                write!(f, "if({if_condition} then {then} else {else_expression})")
            }
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
            NodeExpr::Limit { limit, nodes } => write!(f, "limit({limit} in {nodes})"),
            NodeExpr::Offset { offset, nodes } => write!(f, "offset({offset} in {nodes})"),
            NodeExpr::FlatMap { flat_map, nodes } => write!(f, "flatMap({flat_map} in {nodes})"),
            NodeExpr::FindFirst { find_first, nodes } => write!(f, "findFirst({find_first} in {nodes})"),
            NodeExpr::MatchAll { match_all, nodes } => write!(f, "matchAll({match_all} in {nodes})"),
            NodeExpr::Count(c) => write!(f, "count({c})"),
            NodeExpr::Min(m) => write!(f, "min({m})"),
            NodeExpr::Max(m) => write!(f, "max({m})"),
            NodeExpr::Sum(s) => write!(f, "sum({s})"),
            NodeExpr::InstancesOf(i) => write!(f, "instancesOf({i})"),
            NodeExpr::NodesMatching(n) => write!(f, "nodesMatching({n})"),
        }
    }
}
