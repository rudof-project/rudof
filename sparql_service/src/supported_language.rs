use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
pub enum SupportedLanguage {
    SPARQL10Query,

    #[default]
    SPARQL11Query,

    SPARQL11Update,
}

impl Display for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLanguage::SPARQL10Query => write!(f, "SPARQL10Query"),
            SupportedLanguage::SPARQL11Query => write!(f, "SPARQL11Query"),
            SupportedLanguage::SPARQL11Update => write!(f, "SPARQL11Update"),
        }
    }
}
