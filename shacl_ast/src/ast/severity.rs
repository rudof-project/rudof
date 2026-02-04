use crate::ShaclVocab;
use iri_s::IriS;
use prefixmap::IriRef;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Trace,
    Debug,
    Info,
    Warning,
    Violation,
    Generic(IriRef),
}

impl From<Severity> for IriS {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Trace => IriS::new_unchecked(ShaclVocab::SH_TRACE),
            Severity::Debug => IriS::new_unchecked(ShaclVocab::SH_DEBUG),
            Severity::Info => IriS::new_unchecked(ShaclVocab::SH_INFO),
            Severity::Warning => IriS::new_unchecked(ShaclVocab::SH_WARNING),
            Severity::Violation => IriS::new_unchecked(ShaclVocab::SH_VIOLATION),
            Severity::Generic(iri_ref) => iri_ref.get_iri().unwrap().clone(),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Trace => write!(f, "Trace"),
            Severity::Debug => write!(f, "Debug"),
            Severity::Violation => write!(f, "Violation"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Info => write!(f, "Info"),
            Severity::Generic(iri_ref) => write!(f, "Severity({iri_ref})"),
        }
    }
}
