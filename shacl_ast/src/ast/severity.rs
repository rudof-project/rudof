use crate::SH_DEBUG_STR;
use crate::SH_TRACE_STR;
use crate::shacl_vocab::SH_INFO_STR;
use crate::shacl_vocab::SH_VIOLATION_STR;
use crate::shacl_vocab::SH_WARNING_STR;
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
            Severity::Trace => IriS::new_unchecked(SH_TRACE_STR),
            Severity::Debug => IriS::new_unchecked(SH_DEBUG_STR),
            Severity::Info => IriS::new_unchecked(SH_INFO_STR),
            Severity::Warning => IriS::new_unchecked(SH_WARNING_STR),
            Severity::Violation => IriS::new_unchecked(SH_VIOLATION_STR),
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
