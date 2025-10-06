use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum NodeKind {
    Iri,
    BNode,
    NonLiteral,
    Literal,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Iri => write!(f, "iri"),
            NodeKind::BNode => write!(f, "bnode"),
            NodeKind::NonLiteral => write!(f, "nonliteral"),
            NodeKind::Literal => write!(f, "literal"),
        }
    }
}
