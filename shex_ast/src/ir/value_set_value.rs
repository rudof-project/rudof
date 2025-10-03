use super::object_value::ObjectValue;
use crate::ir::exclusion::{IriExclusion, LanguageExclusion, LiteralExclusion};
use iri_s::IriS;
use srdf::{Object, lang::Lang};
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
                            IriExclusion::Iri(iri) => match object {
                                Object::Iri(iri_s) => {
                                    if iri_s.as_str() == iri.as_str() {
                                        return false;
                                    }
                                }
                                _ => {}
                            },
                            IriExclusion::IriStem(stem) => match object {
                                Object::Iri(iri_s) => {
                                    if iri_s.as_str().starts_with(stem.as_str()) {
                                        return false;
                                    }
                                }
                                _ => {}
                            },
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
                            LiteralExclusion::Literal(s) => match object {
                                Object::Literal(lit) => {
                                    let str = lit.lexical_form();
                                    if str == *s {
                                        return false;
                                    }
                                }
                                _ => {}
                            },
                            LiteralExclusion::LiteralStem(stem) => match object {
                                Object::Literal(lit) => {
                                    let str = lit.lexical_form();
                                    if str.starts_with(stem) {
                                        return false;
                                    }
                                }
                                _ => {}
                            },
                        }
                    }
                }
                true
            }
            ValueSetValue::Language { language_tag } => match object {
                Object::Literal(sliteral) => match sliteral {
                    srdf::SLiteral::StringLiteral { lang, .. } => match lang {
                        Some(lang) => language_tag == lang,
                        None => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            ValueSetValue::LanguageStem { stem } => match object {
                Object::Literal(sliteral) => match sliteral {
                    srdf::SLiteral::StringLiteral { lang, .. } => match lang {
                        Some(lang) => match stem {
                            LangOrWildcard::Lang(stem_lang) => {
                                lang.as_str().starts_with(stem_lang.as_str())
                            }
                            LangOrWildcard::Wildcard { .. } => true,
                        },
                        None => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                let matches_stem = match stem {
                    LangOrWildcard::Lang(lang) => match object {
                        Object::Literal(sliteral) => match sliteral {
                            srdf::SLiteral::StringLiteral { lang: Some(l), .. } => {
                                l.as_str().starts_with(lang.as_str())
                            }
                            _ => false,
                        },
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
                            LanguageExclusion::Language(lang) => match object {
                                Object::Literal(sliteral) => match sliteral {
                                    srdf::SLiteral::StringLiteral { lang: Some(l), .. } => {
                                        if l == lang {
                                            return false;
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            LanguageExclusion::LanguageStem(stem) => match object {
                                Object::Literal(sliteral) => match sliteral {
                                    srdf::SLiteral::StringLiteral { lang: Some(l), .. } => {
                                        if l.as_str().starts_with(stem.as_str()) {
                                            return false;
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                        }
                    }
                }
                true
            }
            ValueSetValue::ObjectValue(v) => v.match_value(object),
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
