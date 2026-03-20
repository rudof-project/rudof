use iri_s::IriS;
use prefixmap::IriRefError;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::fmt::{Display, Formatter};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub(crate) enum Severity {
    Trace,
    Debug,
    Info,
    Warning,
    Violation,
    Generic(IriS),
}

impl From<&IriS> for Severity {
    fn from(value: &IriS) -> Self {
        match value.named_node().as_str() {
            ShaclVocab::SH_TRACE => Severity::Trace,
            ShaclVocab::SH_DEBUG => Severity::Debug,
            ShaclVocab::SH_INFO => Severity::Info,
            ShaclVocab::SH_WARNING => Severity::Warning,
            ShaclVocab::SH_VIOLATION => Severity::Violation,
            _ => Severity::Generic(value.clone())
        }
    }
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
            Severity::Generic(iri) => Ok(iri),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Trace => write!(f, "Trace"),
            Severity::Debug => write!(f, "Debug"),
            Severity::Info => write!(f, "Info"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Violation => write!(f, "Violation"),
            Severity::Generic(iri) => write!(f, "Severity({iri})"),
        }
    }
}
