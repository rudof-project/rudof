use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum NodeKind {
    Iri,
    Literal,
    BlankNode,
    BlankNodeOrIri,
    BlankNodeOrLiteral,
    IRIOrLiteral,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node = match self {
            NodeKind::Iri => crate::SH_IRI.as_named_node(),
            NodeKind::Literal => crate::SH_LITERAL.as_named_node(),
            NodeKind::BlankNode => crate::SH_BLANKNODE.as_named_node(),
            NodeKind::BlankNodeOrIri => crate::SH_BLANK_NODE_OR_IRI.as_named_node(),
            NodeKind::BlankNodeOrLiteral => crate::SH_BLANK_NODE_OR_LITERAL.as_named_node(),
            NodeKind::IRIOrLiteral => crate::SH_IRI_OR_LITERAL.as_named_node(),
        };
        write!(f, "{}", node)
    }
}
