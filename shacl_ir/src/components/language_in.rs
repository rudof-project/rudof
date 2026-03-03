use rudof_rdf::rdf_core::term::literal::Lang;
use std::fmt::Display;

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
#[derive(Debug, Clone)]
pub struct LanguageIn {
    langs: Vec<Lang>,
}

impl LanguageIn {
    pub fn new(langs: Vec<Lang>) -> Self {
        LanguageIn { langs }
    }

    pub fn langs(&self) -> &Vec<Lang> {
        &self.langs
    }
}

impl Display for LanguageIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let langs = self
            .langs()
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "LanguageIn: [{langs}]")
    }
}
