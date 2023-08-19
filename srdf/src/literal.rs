use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

use crate::lang::Lang;
use iri_s::iris::IriS;

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum Literal {
    StringLiteral {
        lexical_form: String,
        lang: Option<Lang>,
    },
    DatatypeLiteral {
        lexical_form: String,
        datatype: IriS,
    },
}

impl Default for Literal {
    fn default() -> Self {
        Literal::StringLiteral {
            lexical_form: String::default(),
            lang: None,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::StringLiteral {
                lexical_form,
                lang: None,
            } => write!(f, "\"{lexical_form}\""),
            Literal::StringLiteral {
                lexical_form,
                lang: Some(lang),
            } => write!(f, "\"{lexical_form}\"@{lang}"),
            Literal::DatatypeLiteral {
                lexical_form,
                datatype,
            } => write!(f, "\"{lexical_form}\"^^{datatype}"),
        }
    }
}
