#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum NodeKind {
    Iri,
    BNode,
    NonLiteral,
    Literal,
}
