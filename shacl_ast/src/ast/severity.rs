use iri_s::IriS;
use prefixmap::IriRef;

use crate::SH_INFO_STR;
use crate::SH_VIOLATION_STR;
use crate::SH_WARNING_STR;

#[derive(Debug, Clone)]
pub enum Severity {
    Violation,
    Warning,
    Info,
    Generic(IriRef),
}

impl From<Severity> for IriS {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Violation => IriS::new_unchecked(SH_VIOLATION_STR),
            Severity::Warning => IriS::new_unchecked(SH_WARNING_STR),
            Severity::Info => IriS::new_unchecked(SH_INFO_STR),
            Severity::Generic(iri_ref) => iri_ref.get_iri().unwrap(),
        }
    }
}
