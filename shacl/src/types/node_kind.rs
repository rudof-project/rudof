use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum NodeKind {
    Iri,
    Lit,
    BNode,
    BNodeOrIri,
    BNodeOrLit,
    IriOrLit,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Iri => write!(f, "Iri"),
            NodeKind::Lit => write!(f, "Literal"),
            NodeKind::BNode => write!(f, "BlankNode"),
            NodeKind::BNodeOrIri => write!(f, "BlankNodeOrIri"),
            NodeKind::BNodeOrLit => write!(f, "BlankNodeOrLiteral"),
            NodeKind::IriOrLit => write!(f, "IriOrLiteral"),
        }
    }
}