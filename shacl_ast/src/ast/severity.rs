use crate::ShaclVocab;
use iri_s::IriS;
use prefixmap::{IriRef, IriRefError};
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

impl TryFrom<Severity> for IriS {
    type Error = IriRefError;

    fn try_from(value: Severity) -> Result<Self, Self::Error> {
        match value {
            Severity::Trace => Ok(ShaclVocab::sh_trace().clone()),
            Severity::Debug => Ok(ShaclVocab::sh_debug().clone()),
            Severity::Info => Ok(ShaclVocab::sh_info().clone()),
            Severity::Warning => Ok(ShaclVocab::sh_warning().clone()),
            Severity::Violation => Ok(ShaclVocab::sh_violation().clone()),
            Severity::Generic(iri_ref) => iri_ref.get_iri().cloned(),
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
