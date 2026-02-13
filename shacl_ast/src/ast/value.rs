use std::fmt::Display;

use iri_s::IriS;
use prefixmap::IriRef;
use srdf::SLiteral;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Iri(IriRef),
    Literal(SLiteral),
}

impl From<IriS> for Value {
    fn from(value: IriS) -> Self {
        Value::Iri(IriRef::iri(value))
    }
}

impl From<SLiteral> for Value {
    fn from(value: SLiteral) -> Self {
        Value::Literal(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Iri(iri) => write!(f, "value({iri})"),
            Value::Literal(lit) => write!(f, "literal({lit})"),
        }
    }
}
