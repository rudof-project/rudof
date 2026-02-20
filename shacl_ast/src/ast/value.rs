use std::fmt::Display;

use iri_s::IriS;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Iri(IriRef),
    Literal(ConcreteLiteral),
}

impl From<IriS> for Value {
    fn from(value: IriS) -> Self {
        Value::Iri(IriRef::iri(value))
    }
}

impl From<ConcreteLiteral> for Value {
    fn from(value: ConcreteLiteral) -> Self {
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
