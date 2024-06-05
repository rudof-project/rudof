use std::fmt::Display;

use iri_s::IriS;
use prefixmap::IriRef;
use srdf::literal::Literal;

#[derive(Debug, Clone)]
pub enum Value {
    Iri(IriRef),
    Literal(Literal),
}

impl Value {
    pub fn iri(iri: IriS) -> Value {
        Value::Iri(IriRef::iri(iri))
    }

    pub fn literal(literal: Literal) -> Value {
        Value::Literal(literal)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Iri(iri) => write!(f, "value({iri})"),
            Value::Literal(lit) => write!(f, "value({lit})"),
        }
    }
}
