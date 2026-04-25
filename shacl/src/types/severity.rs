use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::fmt::{Display, Formatter};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum Severity {
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
            _ => Severity::Generic(value.clone()),
        }
    }
}

impl From<Severity> for IriS {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Trace => ShaclVocab::sh_trace(),
            Severity::Debug => ShaclVocab::sh_debug(),
            Severity::Info => ShaclVocab::sh_info(),
            Severity::Warning => ShaclVocab::sh_warning(),
            Severity::Violation => ShaclVocab::sh_violation(),
            Severity::Generic(iri) => iri,
        }
    }
}

impl From<&Severity> for IriS {
    fn from(value: &Severity) -> Self {
        match value {
            Severity::Trace => ShaclVocab::sh_trace(),
            Severity::Debug => ShaclVocab::sh_debug(),
            Severity::Info => ShaclVocab::sh_info(),
            Severity::Warning => ShaclVocab::sh_warning(),
            Severity::Violation => ShaclVocab::sh_violation(),
            Severity::Generic(iri) => iri.clone(),
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
