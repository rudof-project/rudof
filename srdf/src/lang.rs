use oxilangtag::LanguageTag;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct Lang {
    lang: LanguageTag<String>,
}

impl Lang {
    pub fn new(lang: impl Into<String>) -> Result<Lang, LangParseError> {
        let lang = oxilangtag::LanguageTag::parse_and_normalize(&lang.into())?;
        Ok(Lang { lang })
    }

    pub fn new_unchecked(lang: impl Into<String>) -> Lang {
        Lang {
            lang: oxilangtag::LanguageTag::parse_and_normalize(&lang.into()).unwrap(),
        }
    }
}

impl PartialEq for Lang {
    fn eq(&self, other: &Self) -> bool {
        if self.lang.primary_language() == other.lang.primary_language() {
            let l1 = self.lang.extended_language();
            let l2 = other.lang.extended_language();
            match (l1, l2) {
                (Some(l1), Some(l2)) => l1 == l2,
                _ => true,
            }
        } else {
            false
        }
    }
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang)
    }
}

#[derive(Error, Debug)]
pub enum LangParseError {
    #[error("Invalid language tag: {0}")]
    InvalidLangTag(#[from] oxilangtag::LanguageTagParseError),
}
