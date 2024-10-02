use srdf::SRDFBasic;

use crate::severity::Severity;

use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;

#[derive(Hash, PartialEq, Eq)]
pub enum CompiledSeverity<S: SRDFBasic> {
    Violation,
    Warning,
    Info,
    Generic(S::IRI),
}

impl<S: SRDFBasic> CompiledSeverity<S> {
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
