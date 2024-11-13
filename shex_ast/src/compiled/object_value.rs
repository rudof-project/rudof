use std::fmt::Display;

use iri_s::IriS;
use srdf::graph::lang::Lang;
use srdf::graph::literal::Literal;
use srdf::Object;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ObjectValue {
    IriRef(IriS),
    ObjectLiteral {
        value: String,
        language: Option<Lang>,
        // type_: Option<String>,
    },
}

impl ObjectValue {
    pub(crate) fn match_value(&self, object: &Object) -> bool {
        match self {
            ObjectValue::IriRef(iri_expected) => match object {
                Object::Iri(iri) => iri == iri_expected,
                _ => false,
            },
            ObjectValue::ObjectLiteral { value, language } => match object {
                Object::Literal(lit) => match lit {
                    Literal::StringLiteral { lexical_form, lang } => {
                        value == lexical_form && language == lang
                    }
                    Literal::DatatypeLiteral { .. } => todo!(),
                    Literal::BooleanLiteral(_) => todo!(),
                    Literal::NumericLiteral(_) => todo!(),
                },
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
            ObjectValue::ObjectLiteral { value, language } => {
                write!(f, "\"{value}\"")?;
                match language {
                    None => Ok(()),
                    Some(lang) => {
                        write!(f, "@{lang}")?;
                        Ok(())
                    }
                }
            }
        }
    }
}
