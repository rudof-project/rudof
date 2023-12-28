#[derive(Debug, Clone)]
pub enum NodeKind {
    Iri,
    Literal,
    BlankNode,
    BlankNodeOrIri,
    BlankNodeOrLiteral,
    IRIOrLiteral
}