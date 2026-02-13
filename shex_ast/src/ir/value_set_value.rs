use super::object_value::ObjectValue;
use crate::ir::exclusion::{IriExclusion, LanguageExclusion, LiteralExclusion};
use iri_s::IriS;
use prefixmap::PrefixMap;
use rdf::rdf_core::term::{Object, literal::{Lang, ConcreteLiteral}};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueSetValue {
    IriStem {
        stem: IriS,
    },
    IriStemRange {
        stem: IriOrWildcard,
        exclusions: Option<Vec<IriExclusion>>,
    },
    LiteralStem {
        stem: String,
    },
    LiteralStemRange {
        stem: StringOrWildcard,
        exclusions: Option<Vec<LiteralExclusion>>,
    },
    Language {
        language_tag: Lang,
    },
    LanguageStem {
        stem: LangOrWildcard,
    },
    LanguageStemRange {
        stem: LangOrWildcard,
        exclusions: Option<Vec<LanguageExclusion>>,
    },
    ObjectValue(ObjectValue),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StringOrLiteralStem {
    String(String),
    LiteralStem { stem: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum IriOrWildcard {
    Iri(IriS),
    Wildcard { type_: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StringOrWildcard {
    String(String),

    // TODO: Document the need for the type_ field
    Wildcard { type_: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum LangOrWildcard {
    Lang(Lang),

    // TODO: Document the need for the type_ field
    Wildcard { type_: String },
}

impl Display for LangOrWildcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LangOrWildcard::Lang(lang) => write!(f, "{lang}"),
            LangOrWildcard::Wildcard { .. } => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StringOrIriStem {
    String(String),
    IriStem { stem: String },
}

impl ValueSetValue {
    pub fn match_value(&self, object: &Object) -> bool {
        match self {
            ValueSetValue::IriStem { stem } => match object {
                Object::Iri(iri_s) => iri_s.as_str().starts_with(stem.as_str()),
                _ => false,
            },
            ValueSetValue::IriStemRange { stem, exclusions } => {
                let matches_stem = match stem {
                    IriOrWildcard::Iri(iri) => match object {
                        Object::Iri(iri_s) => iri_s.as_str().starts_with(iri.as_str()),
                        _ => false,
                    },
                    IriOrWildcard::Wildcard { type_: _ } => true, // Matches everything for now
                };
                if !matches_stem {
                    return false;
                }
                if let Some(exclusions) = exclusions {
                    for ex in exclusions {
                        match ex {
                            IriExclusion::Iri(iri) => {
                                if let Object::Iri(iri_s) = object
                                    && iri_s.as_str() == iri.as_str()
                                {
                                    return false;
                                }
                            }
                            IriExclusion::IriStem(stem) => {
                                if let Object::Iri(iri_s) = object
                                    && iri_s.as_str().starts_with(stem.as_str())
                                {
                                    return false;
                                }
                            }
                        }
                    }
                }
                true
            }
            ValueSetValue::LiteralStem { stem } => match object {
                Object::Literal(lit) => {
                    let str = lit.lexical_form();
                    str.starts_with(stem)
                }
                _ => false,
            },
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                let matches_stem = match stem {
                    StringOrWildcard::String(s) => match object {
                        Object::Literal(lit) => {
                            let str = lit.lexical_form();
                            str.starts_with(s)
                        }
                        _ => false,
                    },
                    StringOrWildcard::Wildcard { type_: _ } => true, // Matches everything for now
                };
                if !matches_stem {
                    return false;
                }
                if let Some(exclusions) = exclusions {
                    for ex in exclusions {
                        match ex {
                            LiteralExclusion::Literal(s) => {
                                if let Object::Literal(lit) = object {
                                    let str = lit.lexical_form();
                                    if str == *s {
                                        return false;
                                    }
                                }
                            }
                            LiteralExclusion::LiteralStem(stem) => {
                                if let Object::Literal(lit) = object {
                                    let str = lit.lexical_form();
                                    if str.starts_with(stem) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
                true
            }
            ValueSetValue::Language { language_tag } => object
                .lang()
                .map(|lang| language_tag == lang)
                .unwrap_or(false),
            ValueSetValue::LanguageStem { stem } => object
                .lang()
                .map(|lang| match stem {
                    LangOrWildcard::Lang(s) => lang.as_str().starts_with(s.as_str()),
                    LangOrWildcard::Wildcard { .. } => true, // Matches everything for now
                })
                .unwrap_or(false),
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                let matches_stem = match stem {
                    LangOrWildcard::Lang(lang) => match object {
                        Object::Literal(ConcreteLiteral::StringLiteral { lang: Some(l), .. }) => {
                            l.as_str().starts_with(lang.as_str())
                        }
                        _ => false,
                    },
                    LangOrWildcard::Wildcard { type_: _ } => true, // Matches everything for now
                };
                if !matches_stem {
                    return false;
                }
                if let Some(exclusions) = exclusions {
                    for ex in exclusions {
                        match ex {
                            LanguageExclusion::Language(lang) => {
                                if let Object::Literal(ConcreteLiteral::StringLiteral {
                                    lang: Some(l),
                                    ..
                                }) = object
                                    && l == lang
                                {
                                    return false;
                                }
                            }
                            LanguageExclusion::LanguageStem(stem) => {
                                if let Object::Literal(ConcreteLiteral::StringLiteral {
                                    lang: Some(l),
                                    ..
                                }) = object
                                    && l.as_str().starts_with(stem.as_str())
                                {
                                    return false;
                                }
                            }
                        }
                    }
                }
                true
            }
            ValueSetValue::ObjectValue(v) => v.match_value(object),
        }
    }

    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        match self {
            ValueSetValue::IriStem { stem } => format!("{}~", prefixmap.qualify(stem)),
            ValueSetValue::IriStemRange { stem, exclusions } => {
                let mut s = match stem {
                    IriOrWildcard::Iri(iri) => prefixmap.qualify(iri),
                    IriOrWildcard::Wildcard { type_: _ } => "*".to_string(),
                };
                s.push('~');
                if let Some(exclusions) = exclusions {
                    s.push_str(" - ");
                    let mut first = true;
                    for ex in exclusions {
                        if !first {
                            s.push(' ');
                        }
                        s.push_str(&ex.to_string());
                        first = false;
                    }
                }
                s
            }
            ValueSetValue::LiteralStem { stem } => {
                format!("{stem}~")
            }
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                let mut s = match stem {
                    StringOrWildcard::String(s) => s.clone(),
                    StringOrWildcard::Wildcard { type_: _ } => "*".to_string(),
                };
                s.push('~');
                if let Some(exclusions) = exclusions {
                    s.push_str(" - ");
                    let mut first = true;
                    for ex in exclusions {
                        if !first {
                            s.push(' ');
                        }
                        s.push_str(&ex.to_string());
                        first = false;
                    }
                }
                s
            }
            ValueSetValue::Language { language_tag } => {
                format!("@{}", language_tag)
            }
            ValueSetValue::LanguageStem { stem } => {
                format!("@{stem}~")
            }
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                let mut s = match stem {
                    LangOrWildcard::Lang(lang) => lang.to_string(),
                    LangOrWildcard::Wildcard { type_: _ } => "*".to_string(),
                };
                s.push('~');
                if let Some(exclusions) = exclusions {
                    s.push_str(" - ");
                    let mut first = true;
                    for ex in exclusions {
                        if !first {
                            s.push_str(", ");
                        }
                        s.push_str(&ex.to_string());
                        first = false;
                    }
                }
                s
            }
            ValueSetValue::ObjectValue(object_value) => object_value.show_qualified(prefixmap),
        }
    }
}

impl Display for ValueSetValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueSetValue::IriStem { stem } => write!(f, "{stem}~"),
            ValueSetValue::IriStemRange { stem, exclusions } => {
                match stem {
                    IriOrWildcard::Iri(iri) => write!(f, "{iri}~"),
                    IriOrWildcard::Wildcard { type_ } => write!(f, "*{type_}"),
                }?;
                if let Some(exclusions) = exclusions {
                    write!(f, " EXCEPT ")?;
                    let mut first = true;
                    for ex in exclusions {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{ex}")?;
                        first = false;
                    }
                }
                Ok(())
            }
            ValueSetValue::LiteralStem { stem } => write!(f, "{stem}~"),
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                match stem {
                    StringOrWildcard::String(s) => write!(f, "{s}~"),
                    StringOrWildcard::Wildcard { type_ } => write!(f, "*{type_}"),
                }?;
                if let Some(exclusions) = exclusions {
                    write!(f, " EXCEPT ")?;
                    let mut first = true;
                    for ex in exclusions {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{ex}")?;
                        first = false;
                    }
                }
                Ok(())
            }
            ValueSetValue::Language { language_tag } => write!(f, "@{language_tag}"),
            ValueSetValue::LanguageStem { stem } => write!(f, "{stem}~"),
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                match stem {
                    LangOrWildcard::Lang(lang) => write!(f, "{lang}~"),
                    LangOrWildcard::Wildcard { type_ } => write!(f, "*{type_}"),
                }?;
                if let Some(exclusions) = exclusions {
                    write!(f, " EXCEPT ")?;
                    let mut first = true;
                    for ex in exclusions {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{ex}")?;
                        first = false;
                    }
                }
                Ok(())
            }
            ValueSetValue::ObjectValue(ov) => write!(f, "{ov}"),
        }
    }
}
