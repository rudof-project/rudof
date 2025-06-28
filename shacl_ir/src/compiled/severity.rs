use iri_s::IriS;
use shacl_ast::shacl_vocab::{sh_info, sh_violation, sh_warning};
use srdf::Rdf;

use shacl_ast::severity::Severity;

use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum CompiledSeverity {
    Violation,
    Warning,
    Info,
    Generic(IriS),
}

impl CompiledSeverity {
    pub fn compile(severity: Option<Severity>) -> Result<Option<Self>, CompiledShaclError> {
        let ans = match severity {
            Some(severity) => {
                let severity = match severity {
                    Severity::Violation => CompiledSeverity::Violation,
                    Severity::Warning => CompiledSeverity::Warning,
                    Severity::Info => CompiledSeverity::Info,
                    Severity::Generic(iri_ref) => {
                        let iri = iri_ref
                            .get_iri()
                            .map_err(|_| CompiledShaclError::IriRefConversion)?;
                        CompiledSeverity::Generic(iri)
                    }
                };
                Some(severity)
            }
            None => None,
        };

        Ok(ans)
    }
}

impl From<&CompiledSeverity> for IriS {
    fn from(value: &CompiledSeverity) -> Self {
        match value {
            CompiledSeverity::Violation => sh_violation().clone(),
            CompiledSeverity::Warning => sh_warning().clone(),
            CompiledSeverity::Info => sh_info().clone(),
            CompiledSeverity::Generic(iri) => iri.clone().into(),
        }
    }
}
