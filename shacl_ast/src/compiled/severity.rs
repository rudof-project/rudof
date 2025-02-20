use iri_s::iri;
use iri_s::IriS;
use srdf::Rdf;

use crate::severity::Severity;
use crate::*;

use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum CompiledSeverity<S: Rdf> {
    Violation,
    Warning,
    Info,
    Generic(S::IRI),
}

impl<S: Rdf> CompiledSeverity<S> {
    pub fn compile(severity: Option<Severity>) -> Result<Option<Self>, CompiledShaclError> {
        let ans = match severity {
            Some(severity) => {
                let severity = match severity {
                    Severity::Violation => CompiledSeverity::Violation,
                    Severity::Warning => CompiledSeverity::Warning,
                    Severity::Info => CompiledSeverity::Info,
                    Severity::Generic(iri_ref) => {
                        CompiledSeverity::Generic(convert_iri_ref::<S>(iri_ref)?)
                    }
                };
                Some(severity)
            }
            None => None,
        };

        Ok(ans)
    }
}

impl<S: Rdf> From<&CompiledSeverity<S>> for IriS {
    fn from(value: &CompiledSeverity<S>) -> Self {
        match value {
            CompiledSeverity::Violation => iri!(SH_VIOLATION_STR),
            CompiledSeverity::Warning => iri!(SH_WARNING_STR),
            CompiledSeverity::Info => iri!(SH_INFO_STR),
            CompiledSeverity::Generic(iri) => S::iri2iri_s(iri),
        }
    }
}
