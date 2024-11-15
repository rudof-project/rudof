use std::fmt::Display;

use srdf::model::rdf::Iri;
use srdf::model::rdf::Literal;
use srdf::model::Triple;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value<T: Triple> {
    Iri(Iri<T>),
    Literal(Literal<T>),
}

impl<T: Triple> Value<T> {
    pub fn iri(iri: Iri<T>) -> Value<T> {
        Value::Iri(iri)
    }

    pub fn literal(literal: Literal<T>) -> Value<T> {
        Value::Literal(literal)
    }
}

impl<T: Triple> Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Iri(iri) => write!(f, "value({iri})"),
            Value::Literal(lit) => write!(f, "value({lit})"),
        }
    }
}
