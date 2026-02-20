use iri_s::IriS;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_core::term::{Object, literal::ConcreteLiteral};
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ObjectValue {
    IriRef(IriS),
    ObjectLiteral(ConcreteLiteral),
}

impl ObjectValue {
    pub(crate) fn match_value(&self, object: &Object) -> bool {
        match self {
            ObjectValue::IriRef(iri_expected) => match object {
                Object::Iri(iri) => iri == iri_expected,
                _ => false,
            },
            ObjectValue::ObjectLiteral(literal_expected) => match object {
                Object::Literal(lit) => lit.match_literal(literal_expected),
                _ => false,
            },
        }
    }

    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        match self {
            ObjectValue::IriRef(iri) => prefixmap.qualify(iri),
            ObjectValue::ObjectLiteral(lit) => lit.to_string(),
        }
    }
}

impl Display for ObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectValue::IriRef(iri) => {
                write!(f, "{iri}")?;
                Ok(())
            },
            ObjectValue::ObjectLiteral(lit) => {
                write!(f, "{lit}")
            },
        }
    }
}
