use super::compiled_shacl_error::CompiledShaclError;
use iri_s::IriS;
use shacl_ast::severity::Severity;
use shacl_ast::shacl_vocab::{sh_info, sh_violation, sh_warning};
use shacl_ast::{sh_debug, sh_trace};
use std::fmt::Display;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub enum CompiledSeverity {
    Trace,
    Debug,
    Info,
    Warning,
    Violation,
    Generic(IriS),
}

impl CompiledSeverity {
    pub fn iri(&self) -> IriS {
        match self {
            CompiledSeverity::Trace => sh_trace().clone(),
            CompiledSeverity::Debug => sh_debug().clone(),
            CompiledSeverity::Violation => sh_violation().clone(),
            CompiledSeverity::Warning => sh_warning().clone(),
            CompiledSeverity::Info => sh_info().clone(),
            CompiledSeverity::Generic(iri) => iri.clone(),
        }
    }

    pub fn compile(severity: Option<Severity>) -> Result<Option<Self>, Box<CompiledShaclError>> {
        let ans = match severity {
            Some(severity) => {
                let severity = match severity {
                    Severity::Trace => CompiledSeverity::Trace,
                    Severity::Debug => CompiledSeverity::Debug,
                    Severity::Violation => CompiledSeverity::Violation,
                    Severity::Warning => CompiledSeverity::Warning,
                    Severity::Info => CompiledSeverity::Info,
                    Severity::Generic(iri_ref) => {
                        let iri = iri_ref.get_iri().map_err(|e| {
                            CompiledShaclError::IriRefConversion {
                                iri_ref: iri_ref.to_string(),
                                err: e.to_string(),
                            }
                        })?;
                        CompiledSeverity::Generic(iri)
                    }
                };
                Some(severity)
            }
            None => None,
        };

        Ok(ans)
    }

    pub fn from_iri(iri: &IriS) -> Option<Self> {
        if iri == sh_trace() {
            Some(CompiledSeverity::Trace)
        } else if iri == sh_debug() {
            Some(CompiledSeverity::Debug)
        } else if iri == sh_violation() {
            Some(CompiledSeverity::Violation)
        } else if iri == sh_warning() {
            Some(CompiledSeverity::Warning)
        } else if iri == sh_info() {
            Some(CompiledSeverity::Info)
        } else {
            Some(CompiledSeverity::Generic(iri.clone()))
        }
    }

    pub fn to_iri(&self) -> IriS {
        match self {
            CompiledSeverity::Trace => sh_trace().clone(),
            CompiledSeverity::Debug => sh_debug().clone(),
            CompiledSeverity::Violation => sh_violation().clone(),
            CompiledSeverity::Warning => sh_warning().clone(),
            CompiledSeverity::Info => sh_info().clone(),
            CompiledSeverity::Generic(iri) => iri.clone(),
        }
    }
}

impl From<&CompiledSeverity> for IriS {
    fn from(value: &CompiledSeverity) -> Self {
        CompiledSeverity::to_iri(value)
    }
}

impl Display for CompiledSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledSeverity::Trace => write!(f, "Trace"),
            CompiledSeverity::Debug => write!(f, "Debug"),
            CompiledSeverity::Violation => write!(f, "Violation"),
            CompiledSeverity::Warning => write!(f, "Warning"),
            CompiledSeverity::Info => write!(f, "Info"),
            CompiledSeverity::Generic(iri) => write!(f, "Generic({iri})"),
        }
    }
}
