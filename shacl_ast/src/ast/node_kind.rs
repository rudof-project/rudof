use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum NodeKind {
    Iri,
    Literal,
    BlankNode,
    BlankNodeOrIri,
    BlankNodeOrLiteral,
    IRIOrLiteral
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Iri => write!(f, "Iri"),
            NodeKind::Literal => write!(f, "Literal"),
            NodeKind::BlankNode => write!(f, "BlankNode"),
            NodeKind::BlankNodeOrIri => write!(f, "BlankNodeOrIri"),
            NodeKind::BlankNodeOrLiteral => write!(f, "BlankNodeOrLiteral"),
            NodeKind::IRIOrLiteral => write!(f, "IriOrLiteral"),
        }
    }
}
