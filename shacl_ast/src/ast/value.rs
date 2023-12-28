use prefixmap::IriRef;
use srdf::literal::Literal;

#[derive(Debug, Clone)]
pub enum Value {
    Iri(IriRef),
    Literal(Literal)
}