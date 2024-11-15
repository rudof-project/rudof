use std::fmt::Display;

use iri_s::iri;

use crate::vocab::SH_BLANKNODE;
use crate::vocab::SH_BLANK_NODE_OR_IRI;
use crate::vocab::SH_BLANK_NODE_OR_LITERAL;
use crate::vocab::SH_IRI;
use crate::vocab::SH_IRI_OR_LITERAL;
use crate::vocab::SH_LITERAL;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
            NodeKind::Iri => iri!(SH_IRI),
            NodeKind::Literal => iri!(SH_LITERAL),
            NodeKind::BlankNode => iri!(SH_BLANKNODE),
            NodeKind::BlankNodeOrIri => iri!(SH_BLANK_NODE_OR_IRI),
            NodeKind::BlankNodeOrLiteral => iri!(SH_BLANK_NODE_OR_LITERAL),
            NodeKind::IRIOrLiteral => iri!(SH_IRI_OR_LITERAL),
        };
        write!(f, "{}", node)
    }
}
