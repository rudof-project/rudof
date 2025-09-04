use std::fmt::Display;

use iri_s::IriS;
use srdf::{Object, literal::SLiteral};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ObjectValue {
    IriRef(IriS),
    ObjectLiteral(SLiteral),
}

impl ObjectValue {
    pub(crate) fn match_value(&self, object: &Object) -> bool {
        match self {
            ObjectValue::IriRef(iri_expected) => match object {
                Object::Iri(iri) => iri == iri_expected,
                _ => false,
            },
            ObjectValue::ObjectLiteral(literal_expected) => match object {
                Object::Literal(lit) => {
                    // We compare lexical forms and datatypes because some parsed literals are not optimized as primitive literals (like integers)
                    literal_expected.datatype() == lit.datatype()
                        && literal_expected.lexical_form() == lit.lexical_form()
                }
                _ => false,
            },
        }
    }
}

impl Display for ObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectValue::IriRef(iri) => {
                write!(f, "{iri}")?;
                Ok(())
            }
            ObjectValue::ObjectLiteral(lit) => {
                write!(f, "{lit}")
            }
        }
    }
}
