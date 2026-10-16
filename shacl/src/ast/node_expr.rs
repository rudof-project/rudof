use crate::ast::ASTShape;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::SHACLPath;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNodeExpr {
    // Constants
    Iri(IriRef),
    Literal(ConcreteLiteral),

    // Basic expressions
    Empty,
    Var(String),
    List(Vec<Object>),
    PathValues {
        path: SHACLPath,
        focus_node: Option<Box<ASTNodeExpr>>,
    },
    Exists(Box<ASTNodeExpr>),
    IfExpression {
        if_condition: Box<ASTNodeExpr>,
        then: Option<Box<ASTNodeExpr>>,
        else_expression: Option<Box<ASTNodeExpr>>,
    },

    // List expressions
    Distinct(Box<ASTNodeExpr>),
    Intersection(Vec<ASTNodeExpr>),
    Concat(Vec<ASTNodeExpr>),
    Remove {
        remove: Box<ASTNodeExpr>,
        nodes: Box<ASTNodeExpr>,
    },
    Filter {
        filter_shape: ASTShape,
        nodes: Box<ASTNodeExpr>,
    },
    Limit {
        limit: usize,
        nodes: Box<ASTNodeExpr>,
    },
    Offset {
        offset: usize,
        nodes: Box<ASTNodeExpr>,
    },
    OrderBy {
        desc: Option<bool>,
        order_by: Box<ASTNodeExpr>,
        nodes: Box<ASTNodeExpr>,
    },

    // Advance sequence operations
    FlatMap {
        flat_map: Box<ASTNodeExpr>,
        nodes: Box<ASTNodeExpr>,
    },
    FindFirst {
        find_first: ASTShape,
        nodes: Box<ASTNodeExpr>,
    },
    MatchAll {
        match_all: ASTShape,
        nodes: Box<ASTNodeExpr>,
    },

    // Aggregation expressions
    Count(Box<ASTNodeExpr>),
    Min(Box<ASTNodeExpr>),
    Max(Box<ASTNodeExpr>),
    Sum(Box<ASTNodeExpr>),

    // Miscellaneous expressions
    InstancesOf(IriRef),
    NodesMatching(IriOrBlankNode),
    // ConformsToShape // TODO - Docs are a little bit unclear clear about the parameters
}

impl Display for ASTNodeExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNodeExpr::Iri(i) => write!(f, "iri({i})"),
            ASTNodeExpr::Literal(l) => write!(f, "literal({l})"),
            ASTNodeExpr::Empty => write!(f, "empty"),
            ASTNodeExpr::Var(v) => write!(f, "var({v})"),
            ASTNodeExpr::List(l) => write!(
                f,
                "list({})",
                l.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(", ")
            ),
            ASTNodeExpr::PathValues { focus_node, path } => {
                let fnode = match focus_node {
                    None => "".to_string(),
                    Some(n) => n.to_string(),
                };

                write!(f, "pathValues({path}, {fnode})")
            },
            ASTNodeExpr::Exists(e) => write!(f, "exists({e})"),
            ASTNodeExpr::IfExpression {
                if_condition,
                then,
                else_expression,
            } => {
                let then = then.as_deref().map(|t| t.to_string()).unwrap_or_default();
                let else_expression = else_expression.as_deref().map(|e| e.to_string()).unwrap_or_default();
                write!(f, "if({if_condition} then {then} else {else_expression})")
            },
            ASTNodeExpr::Distinct(d) => write!(f, "distinct({d})"),
            ASTNodeExpr::Intersection(i) => write!(
                f,
                "intersection({})",
                i.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
            ),
            ASTNodeExpr::Concat(c) => write!(
                f,
                "concat({})",
                c.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
            ),
            ASTNodeExpr::Remove { remove, nodes } => write!(f, "remove({remove} in {nodes})"),
            ASTNodeExpr::Filter { filter_shape, nodes } => write!(f, "filter({filter_shape} in {nodes})"),
            ASTNodeExpr::Limit { limit, nodes } => write!(f, "limit({limit} in {nodes})"),
            ASTNodeExpr::Offset { offset, nodes } => write!(f, "offset({offset} in {nodes})"),
            ASTNodeExpr::OrderBy { order_by, nodes, desc } => {
                let d = if let Some(desc) = desc {
                    &format!(", desc={desc}")
                } else {
                    ""
                };
                write!(f, "orderBy({order_by} in {nodes}{d})")
            },
            ASTNodeExpr::FlatMap { flat_map, nodes } => write!(f, "flatMap({flat_map} in {nodes})"),
            ASTNodeExpr::FindFirst { find_first, nodes } => write!(f, "findFirst({find_first} in {nodes})"),
            ASTNodeExpr::MatchAll { match_all, nodes } => write!(f, "matchAll({match_all} in {nodes})"),
            ASTNodeExpr::Count(c) => write!(f, "count({c})"),
            ASTNodeExpr::Min(m) => write!(f, "min({m})"),
            ASTNodeExpr::Max(m) => write!(f, "max({m})"),
            ASTNodeExpr::Sum(s) => write!(f, "sum({s})"),
            ASTNodeExpr::InstancesOf(i) => write!(f, "instancesOf({i})"),
            ASTNodeExpr::NodesMatching(n) => write!(f, "nodesMatching({n})"),
        }
    }
}
