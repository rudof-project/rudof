use std::hash::Hash;

use oxilangtag::LanguageTag;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Lang {
    lang: LanguageTag<String>,
}

impl Lang {
    pub fn new(lang: impl Into<String>) -> Result<Lang, LangParseError> {
        let lang = oxilangtag::LanguageTag::parse_and_normalize(&lang.into())?;
        Ok(Lang { lang })
    }

    /*pub fn new_unchecked(lang: impl Into<String>) -> Lang {
        let str: String = lang.into();
        let lang = match oxilangtag::LanguageTag::parse_and_normalize(str.as_str()) {
            Ok(lang) => lang,
            Err(e) => panic!("Invalid language tag {str}: {e}"),
        };
        Lang { lang }
    }*/

    pub fn as_str(&self) -> &str {
        self.lang.as_str()
    }
}

impl PartialEq for Lang {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang
        /*if self.lang.primary_language() == other.lang.primary_language() {
            let l1 = self.lang.extended_language();
            let l2 = other.lang.extended_language();
            match (l1, l2) {
                (Some(l1), Some(l2)) => l1 == l2,
                _ => true,
            }
        } else {
            false
        }*/
    }
}

impl Hash for Lang {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lang.hash(state);
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

#[cfg(test)]
mod tests {
    use crate::lang::Lang;

    #[test]
    fn test_lang_equality() {
        let en = Lang::new("en").unwrap();
        let en_us = Lang::new("en-US").unwrap();
        let fr = Lang::new("fr").unwrap();
        let en1 = Lang::new("en").unwrap();
        let en_fr = Lang::new("en-fr").unwrap();

        assert_ne!(en, en_us);
        assert_ne!(en, fr);
        assert_eq!(en, en1);
        assert_ne!(en, en_fr);
    }
}
